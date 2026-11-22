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

import { afterEach, describe, expect, it, vi } from "vitest";
import * as auth from "./service";

const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
        status,
        headers: { "Content-Type": "application/json" },
    });

const loginData = {
    token: "token",
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

describe("auth logout API", () => {
    afterEach(() => vi.unstubAllGlobals());

    it.each([200, 204])("accepts an empty %s response", async (status) => {
        const fetchMock = vi
            .fn()
            .mockResolvedValue(new Response(null, { status }));
        vi.stubGlobal("fetch", fetchMock);

        expect(await auth.logout()).toEqual({
            kind: "success",
            status,
            data: undefined,
        });
        expect(fetchMock).toHaveBeenCalledWith(
            "/api/auth/t/logout",
            expect.objectContaining({ method: "POST" }),
        );
    });
});

describe("auth API contracts", () => {
    afterEach(() => vi.unstubAllGlobals());

    it("sends login with optional OTP and validates login data", async () => {
        const fetchMock = vi.fn().mockResolvedValue(json({ data: loginData }));
        vi.stubGlobal("fetch", fetchMock);
        expect(
            await auth.login({
                email: "user@example.com",
                password: "pw",
                otp: " ",
            }),
        ).toMatchObject({ kind: "success", status: 200 });
        expect(JSON.parse(fetchMock.mock.calls[0][1].body)).toEqual({
            email: "user@example.com",
            password: "pw",
        });
        await auth.login({
            email: "user@example.com",
            password: "pw",
            otp: "123456",
        });
        expect(JSON.parse(fetchMock.mock.calls[1][1].body).otp).toBe("123456");
        fetchMock.mockResolvedValueOnce(
            json({
                data: { ...loginData, claims: { ...loginData.claims, tz: 4 } },
            }),
        );
        expect(await auth.refresh()).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(fetchMock.mock.calls[2][0]).toBe("/api/auth/t/refresh");
        expect(fetchMock.mock.calls[2][1].credentials).toBeUndefined();
    });

    it("preserves the message-only OTP challenge", async () => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValue(
                    json({ error: { message: "totp-required" } }, 401),
                ),
        );
        expect(
            await auth.login({ email: "a", password: "b", otp: null }),
        ).toEqual({
            kind: "http_error",
            status: 401,
            body: { error: { message: "totp-required" } },
        });
    });

    it("accepts typed login field errors and rejects malformed fields", async () => {
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValueOnce(
                    json(
                        {
                            error: {
                                message: "Invalid",
                                fields: { otp: "Invalid code" },
                            },
                        },
                        422,
                    ),
                )
                .mockResolvedValueOnce(
                    json(
                        { error: { message: "Invalid", fields: { otp: 123 } } },
                        422,
                    ),
                ),
        );
        const credentials = { email: "a", password: "b", otp: "123456" };
        expect(await auth.login(credentials)).toMatchObject({
            kind: "http_error",
            body: { error: { fields: { otp: "Invalid code" } } },
        });
        expect(await auth.login(credentials)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it.each([
        [
            "register",
            () =>
                auth.register({
                    firstName: "First",
                    lastName: "Last",
                    email: "a",
                    password: "p",
                    passwordConfirm: "p",
                }),
            "/api/auth/register",
            {
                first_name: "First",
                last_name: "Last",
                email: "a",
                password: "p",
                password_confirm: "p",
            },
            201,
        ],
        [
            "forgotten",
            () => auth.forgottenPassword({ email: "a" }),
            "/api/auth/forgotten_password",
            { email: "a" },
            200,
        ],
        [
            "new password",
            () =>
                auth.newPassword({
                    token: "id",
                    password: "p",
                    password_confirm: "p",
                }),
            "/api/auth/new_password",
            { token: "id", password: "p", password_confirm: "p" },
            200,
        ],
    ])(
        "sends %s fields and accepts the backend status",
        async (_name, invoke, url, body, status) => {
            const fetchMock = vi
                .fn()
                .mockResolvedValue(json({ data: { message: "ok" } }, status));
            vi.stubGlobal("fetch", fetchMock);
            expect(await invoke()).toMatchObject({ kind: "success", status });
            expect(fetchMock.mock.calls[0][0]).toBe(url);
            expect(fetchMock.mock.calls[0][1].method).toBe("POST");
            expect(JSON.parse(fetchMock.mock.calls[0][1].body)).toEqual(body);
        },
    );

    it.each([
        [
            () =>
                auth.register({
                    firstName: "",
                    lastName: "",
                    email: "",
                    password: "",
                    passwordConfirm: "",
                }),
            {
                email: null,
                first_name: null,
                last_name: null,
                password: "invalid",
                password_confirm: null,
            },
        ],
        [() => auth.forgottenPassword({ email: "" }), { email: "invalid" }],
        [
            () =>
                auth.newPassword({
                    token: "",
                    password: "",
                    password_confirm: "",
                }),
            { token: null, password: "invalid", password_confirm: null },
        ],
    ])("validates complete backend field errors", async (invoke, fields) => {
        const fetchMock = vi
            .fn()
            .mockResolvedValueOnce(
                json({ error: { message: "invalid", fields } }, 422),
            )
            .mockResolvedValueOnce(
                json(
                    {
                        error: {
                            message: "invalid",
                            fields: { ...fields, password: 5, email: 5 },
                        },
                    },
                    422,
                ),
            );
        vi.stubGlobal("fetch", fetchMock);
        expect(await invoke()).toMatchObject({
            kind: "http_error",
            body: { error: { fields } },
        });
        expect(await invoke()).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it("encodes verification tokens and rejects missing success data", async () => {
        const fetchMock = vi
            .fn()
            .mockResolvedValueOnce(json({ data: { message: "ok" } }))
            .mockResolvedValueOnce(json({}));
        vi.stubGlobal("fetch", fetchMock);
        expect(await auth.verfiy_email("a&b")).toMatchObject({
            kind: "success",
        });
        expect(fetchMock.mock.calls[0][0]).toBe(
            "/api/auth/verify_email?id=a%26b",
        );
        expect(await auth.verfiy_email("id")).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it("forwards cancellation to an individual request", async () => {
        const fetchMock = vi
            .fn()
            .mockResolvedValue(json({ data: { message: "ok" } }));
        vi.stubGlobal("fetch", fetchMock);
        const controller = new AbortController();
        await auth.forgottenPassword({ email: "a" }, controller.signal);
        const signal = fetchMock.mock.calls[0][1].signal as AbortSignal;
        expect(signal.aborted).toBe(false);
        controller.abort();
        expect(signal.aborted).toBe(true);
    });
});
