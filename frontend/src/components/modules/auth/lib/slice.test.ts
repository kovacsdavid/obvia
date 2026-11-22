/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import { configureStore } from "@reduxjs/toolkit";
import { beforeEach, describe, expect, it, vi } from "vitest";
import * as api from "./service";
import reducer, {
    loginUser,
    logoutAndRevokeRefreshToken,
    refreshAccessToken,
    get_claims,
    verfiy_email,
} from "./slice";
import type { LoginData } from "./interface";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

vi.mock("./service", () => ({
    refresh: vi.fn(),
    logout: vi.fn(),
    get_claims: vi.fn(),
    verfiy_email: vi.fn(),
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));

const loginData: LoginData = {
    token: "fresh",
    claims: {
        sub: "user",
        exp: 9999999999,
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
const success = {
    kind: "success",
    status: 200,
    data: { data: loginData },
} as const;
const store = () => configureStore({ reducer: { auth: reducer } });

describe("auth session thunks", () => {
    beforeEach(() => vi.resetAllMocks());

    it("coalesces concurrent refreshes and installs the new login", async () => {
        let resolve!: (value: ApiResult<{ data: LoginData }>) => void;
        vi.mocked(api.refresh).mockReturnValue(
            new Promise((done) => {
                resolve = done;
            }),
        );
        const app = store();
        const first = app.dispatch(refreshAccessToken());
        const second = app.dispatch(refreshAccessToken());
        expect(api.refresh).toHaveBeenCalledOnce();
        resolve(success);
        await Promise.all([first, second]);
        expect(app.getState().auth.login).toMatchObject({
            token: "fresh",
            isLoggedIn: true,
        });
        await app.dispatch(refreshAccessToken());
        expect(api.refresh).toHaveBeenCalledOnce();
    });

    it("clears the session on refresh failure and allows retry", async () => {
        vi.mocked(api.refresh)
            .mockResolvedValueOnce({ kind: "network_error", offline: false })
            .mockResolvedValueOnce(success);
        const app = store();
        app.dispatch(
            loginUser({
                ...loginData,
                claims: { ...loginData.claims, exp: 1 },
            }),
        );
        await app.dispatch(refreshAccessToken());
        expect(app.getState().auth.login).toMatchObject({
            token: null,
            isLoggedIn: false,
        });
        expect(notifyApiError).not.toHaveBeenCalled();
        await app.dispatch(refreshAccessToken());
        expect(app.getState().auth.login.token).toBe("fresh");
    });

    it("waits for refresh before revoking, coalesces logout, and never restores the old session", async () => {
        let resolveRefresh!: (value: ApiResult<{ data: LoginData }>) => void;
        let resolveLogout!: (value: ApiResult<undefined>) => void;
        vi.mocked(api.refresh).mockReturnValue(
            new Promise((done) => {
                resolveRefresh = done;
            }),
        );
        vi.mocked(api.logout).mockReturnValue(
            new Promise((done) => {
                resolveLogout = done;
            }),
        );
        const app = store();
        const refresh = app.dispatch(refreshAccessToken());
        const logout = app.dispatch(logoutAndRevokeRefreshToken());
        const secondLogout = app.dispatch(logoutAndRevokeRefreshToken());
        const duringLogout = app.dispatch(refreshAccessToken());
        expect(api.logout).not.toHaveBeenCalled();
        resolveRefresh(success);
        await refresh;
        expect(api.logout).toHaveBeenCalledOnce();
        expect(app.getState().auth.login.isLoggedIn).toBe(false);
        resolveLogout({ kind: "http_error", status: 500 });
        await Promise.all([logout, secondLogout, duringLogout]);
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith({
            kind: "http_error",
            status: 500,
        });
        expect(api.refresh).toHaveBeenCalledOnce();
        expect(app.getState().auth.login).toMatchObject({
            token: null,
            isLoggedIn: false,
        });
    });

    it("clears local state even if revocation throws", async () => {
        vi.mocked(api.logout).mockRejectedValue(new Error("network"));
        const app = store();
        app.dispatch(loginUser(loginData));
        await app.dispatch(logoutAndRevokeRefreshToken());
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith({
            kind: "network_error",
            offline: false,
        });
        expect(app.getState().auth.login).toMatchObject({
            token: null,
            isLoggedIn: false,
        });
    });

    it("notifies claims and verification errors and supports silent requests", async () => {
        const failure = { kind: "network_error", offline: false } as const;
        vi.mocked(api.get_claims).mockResolvedValue(failure);
        vi.mocked(api.verfiy_email).mockResolvedValue(failure);
        const app = store();
        app.dispatch(loginUser(loginData));
        await app.dispatch(get_claims());
        await app.dispatch(verfiy_email("id"));
        expect(notifyApiError).toHaveBeenCalledTimes(2);
        await app.dispatch(get_claims({ silent: true }));
        await app.dispatch(verfiy_email({ id: "id", silent: true }));
        expect(notifyApiError).toHaveBeenCalledTimes(2);
    });

    it("supports silent logout without changing local logout behavior", async () => {
        vi.mocked(api.logout).mockResolvedValue({
            kind: "http_error",
            status: 500,
        });
        const app = store();
        app.dispatch(loginUser(loginData));
        await app.dispatch(logoutAndRevokeRefreshToken({ silent: true }));
        expect(notifyApiError).not.toHaveBeenCalled();
        expect(app.getState().auth.login.isLoggedIn).toBe(false);
    });
});
