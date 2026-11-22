import { configureStore } from "@reduxjs/toolkit";
import { beforeEach, describe, expect, it, vi } from "vitest";
import authReducer, {
    get_claims,
    loginUser,
    logoutAndRevokeRefreshToken,
} from "./slice";
import type { LoginData } from "./interface";
import type { ApiResult } from "@/lib/api/types";
import type { RootState } from "@/store";
import * as authApi from "./service";
import * as customersApi from "@/components/modules/customers/lib/service";
import * as usersApi from "@/components/modules/users/lib/service";
import * as databasesApi from "@/components/modules/databases/lib/service";
import * as customers from "@/components/modules/customers/lib/slice";
import * as users from "@/components/modules/users/lib/slice";
import * as databases from "@/components/modules/databases/lib/slice";
import { getErrorMessage } from "@/lib/api/messages";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

vi.mock("./service", () => ({
    refresh: vi.fn(),
    get_claims: vi.fn(),
    logout: vi.fn(),
}));
vi.mock("@/components/modules/customers/lib/service", () => ({
    list: vi.fn(),
    print: vi.fn(),
}));
vi.mock("@/components/modules/users/lib/service", () => ({
    enableOtp: vi.fn(),
}));
vi.mock("@/components/modules/databases/lib/service", () => ({
    list: vi.fn(),
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));

const login: LoginData = {
    token: "expired-access-token",
    claims: {
        sub: "user",
        exp: 1,
        iat: 1,
        nbf: 1,
        iss: "issuer",
        aud: "audience",
        jti: "id",
        loc: "hu",
        tz: "Europe/Budapest",
        family_id: null,
        active_tenant: null,
    },
    user: {
        id: "user",
        email: "user@example.com",
        first_name: null,
        last_name: null,
        status: "active",
        profile_picture_url: null,
        is_mfa_enabled: false,
    },
};

const expired = { kind: "http_error", status: 401 } as const;
const store = () =>
    configureStore({
        reducer: (
            state = {
                auth: authReducer(undefined, { type: "init" }),
            } as RootState,
            action,
        ) => ({ ...state, auth: authReducer(state.auth, action) }),
    });

describe("protected requests after refresh", () => {
    beforeEach(() => vi.resetAllMocks());

    it("stops concurrent and subsequent requests and reports the session expiry once", async () => {
        vi.mocked(authApi.refresh).mockResolvedValue({
            ...expired,
            body: { error: { message: "backend refresh token error" } },
        });
        const app = store();
        app.dispatch(loginUser(login));
        const results = await Promise.all([
            app
                .dispatch(customers.list({ query: null, silent: true }))
                .unwrap(),
            app.dispatch(customers.print({ uuid: "id" })).unwrap(),
            app.dispatch(users.enableOtp()).unwrap(),
            app.dispatch(databases.list({ query: null })).unwrap(),
            app.dispatch(get_claims()).unwrap(),
        ]);
        expect(results).toEqual([expired, expired, expired, expired, expired]);
        expect(
            await app.dispatch(customers.list({ query: null })).unwrap(),
        ).toEqual(expired);
        expect(authApi.refresh).toHaveBeenCalledOnce();
        expect(customersApi.list).not.toHaveBeenCalled();
        expect(customersApi.print).not.toHaveBeenCalled();
        expect(usersApi.enableOtp).not.toHaveBeenCalled();
        expect(databasesApi.list).not.toHaveBeenCalled();
        expect(authApi.get_claims).not.toHaveBeenCalled();
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(expired);
        expect(getErrorMessage(expired)).toBe(
            "A munkameneted lejárt. Kérlek, jelentkezz be újra!",
        );
    });

    it("allows a non-silent request to report a previously silent expiry and resets on login", async () => {
        vi.mocked(authApi.refresh).mockResolvedValue(expired);
        const app = store();
        app.dispatch(loginUser(login));
        await app.dispatch(customers.list({ query: null, silent: true }));
        expect(notifyApiError).not.toHaveBeenCalled();
        await app.dispatch(users.enableOtp());
        expect(notifyApiError).toHaveBeenCalledTimes(1);
        app.dispatch(loginUser(login));
        await app.dispatch(users.enableOtp());
        expect(authApi.refresh).toHaveBeenCalledTimes(2);
        expect(notifyApiError).toHaveBeenCalledTimes(2);
    });

    it("keeps concurrent silent requests silent", async () => {
        vi.mocked(authApi.refresh).mockResolvedValue(expired);
        const app = store();
        app.dispatch(loginUser(login));
        const results = await Promise.all([
            app
                .dispatch(customers.list({ query: null, silent: true }))
                .unwrap(),
            app.dispatch(users.enableOtp({ silent: true })).unwrap(),
        ]);
        expect(results).toEqual([expired, expired]);
        expect(notifyApiError).not.toHaveBeenCalled();
        expect(customersApi.list).not.toHaveBeenCalled();
        expect(usersApi.enableOtp).not.toHaveBeenCalled();
    });

    it("does not run protected requests when logout interrupts a refresh", async () => {
        let resolve!: (result: ApiResult<{ data: LoginData }>) => void;
        vi.mocked(authApi.refresh).mockReturnValue(
            new Promise((done) => {
                resolve = done;
            }),
        );
        vi.mocked(authApi.logout).mockResolvedValue({
            kind: "success",
            status: 204,
            data: undefined,
        });
        const app = store();
        app.dispatch(loginUser(login));
        const request = app.dispatch(customers.list({ query: null })).unwrap();
        const logout = app.dispatch(
            logoutAndRevokeRefreshToken({ silent: true }),
        );
        resolve({ kind: "success", status: 200, data: { data: login } });
        expect(await request).toEqual({ kind: "aborted" });
        await logout;
        expect(customersApi.list).not.toHaveBeenCalled();
        expect(notifyApiError).not.toHaveBeenCalled();
    });

    it("does not use a valid token while logout is in progress", async () => {
        let resolveLogout!: (result: ApiResult<undefined>) => void;
        vi.mocked(authApi.logout).mockReturnValue(
            new Promise((done) => {
                resolveLogout = done;
            }),
        );
        const app = store();
        app.dispatch(
            loginUser({
                ...login,
                claims: { ...login.claims, exp: 9999999999 },
            }),
        );
        const request = app.dispatch(customers.list({ query: null })).unwrap();
        const logout = app.dispatch(
            logoutAndRevokeRefreshToken({ silent: true }),
        );
        expect(app.getState().auth.login.token).toBe(login.token);
        expect(await request).toEqual({ kind: "aborted" });
        expect(customersApi.list).not.toHaveBeenCalled();
        resolveLogout({ kind: "success", status: 204, data: undefined });
        await logout;
    });

    it("converts thrown refresh errors to a network failure", async () => {
        vi.mocked(authApi.refresh).mockRejectedValue(new Error("unavailable"));
        const app = store();
        app.dispatch(loginUser(login));
        expect(await app.dispatch(users.enableOtp()).unwrap()).toEqual({
            kind: "network_error",
            offline: false,
        });
        expect(usersApi.enableOtp).not.toHaveBeenCalled();
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith({
            kind: "network_error",
            offline: false,
        });
    });

    it.each([
        { kind: "network_error", offline: false } as const,
        { kind: "timeout" } as const,
        { kind: "http_error", status: 503 } as const,
    ])(
        "returns refresh failure without invoking the protected endpoint: $kind",
        async (failure) => {
            vi.mocked(authApi.refresh).mockResolvedValue(failure);
            const app = store();
            app.dispatch(loginUser(login));
            expect(
                await app.dispatch(customers.list({ query: null })).unwrap(),
            ).toEqual(failure);
            expect(customersApi.list).not.toHaveBeenCalled();
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(failure);
        },
    );

    it("uses the renewed token after successful refresh", async () => {
        const renewed = {
            ...login,
            token: "renewed",
            claims: { ...login.claims, exp: 9999999999 },
        };
        vi.mocked(authApi.refresh).mockResolvedValue({
            kind: "success",
            status: 200,
            data: { data: renewed },
        });
        vi.mocked(usersApi.enableOtp).mockResolvedValue({ kind: "timeout" });
        const app = store();
        app.dispatch(loginUser(login));
        expect(await app.dispatch(users.enableOtp()).unwrap()).toEqual({
            kind: "timeout",
        });
        expect(usersApi.enableOtp).toHaveBeenCalledExactlyOnceWith(
            "renewed",
            expect.any(AbortSignal),
        );
    });
});
