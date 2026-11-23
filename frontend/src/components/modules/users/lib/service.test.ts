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

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import * as usersApi from "./service";
import { isOtpFields } from "./guards";

const input = {
    email: "user@example.com",
    lastName: "Last",
    firstName: "First",
    phone: "123",
    status: "active",
};

function jsonResponse(body: unknown, status = 200) {
    return new Response(JSON.stringify(body), {
        status,
        headers: { "Content-Type": "application/json" },
    });
}

describe("users API", () => {
    const fetchMock = vi.fn<typeof fetch>();

    beforeEach(() => {
        fetchMock.mockReset();
        fetchMock.mockImplementation(async () => jsonResponse({ data: {} }));
        vi.stubGlobal("fetch", fetchMock);
    });

    afterEach(() => vi.unstubAllGlobals());

    it.each([
        [
            "create",
            (signal: AbortSignal) => usersApi.create(input, "token", signal),
            "/api/users/create",
            "POST",
        ],
        [
            "list",
            (signal: AbortSignal) =>
                usersApi.list(
                    "paging%3A2-25%20filtering%3Aname-%7CA%20B%7C",
                    "token",
                    signal,
                ),
            "/api/users/list?q=paging%3A2-25%20filtering%3Aname-%7CA%20B%7C",
            "GET",
        ],
        [
            "enable",
            (signal: AbortSignal) => usersApi.enableOtp("token", signal),
            "/api/users/otp/enable",
            "GET",
        ],
        [
            "verify",
            (signal: AbortSignal) =>
                usersApi.verifyOtp("123456", "token", signal),
            "/api/users/otp/verify",
            "POST",
        ],
        [
            "disable",
            (signal: AbortSignal) =>
                usersApi.disableOtp("123456", "token", signal),
            "/api/users/otp/disable",
            "POST",
        ],
    ])(
        "sends %s with authorization and a cancellable signal",
        async (_name, invoke, url, method) => {
            const controller = new AbortController();
            await invoke(controller.signal);
            const [actualUrl, options] = fetchMock.mock.calls[0];
            expect(actualUrl).toBe(url);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
        },
    );

    it("maps the legacy create input without claiming a success contract", async () => {
        expect(await usersApi.create(input, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(JSON.parse(fetchMock.mock.calls[0][1]?.body as string)).toEqual({
            email: input.email,
            last_name: input.lastName,
            first_name: input.firstName,
            phone: input.phone,
            status: input.status,
        });
        expect(fetchMock.mock.calls[0][1]?.headers).not.toHaveProperty(
            "Authorization",
        );
        expect(await usersApi.list(null, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(fetchMock.mock.calls[1][0]).toBe("/api/users/list");
    });

    it.each(["verifyOtp", "disableOtp"] as const)(
        "validates %s success and field errors",
        async (operation) => {
            fetchMock.mockResolvedValueOnce(
                jsonResponse({ data: { message: "Done" } }),
            );
            expect(await usersApi[operation]("123456", null)).toEqual({
                kind: "success",
                status: 200,
                data: { data: { message: "Done" } },
            });
            expect(
                JSON.parse(fetchMock.mock.calls[0][1]?.body as string),
            ).toEqual({ otp: "123456" });
            const body = {
                error: { message: "Invalid", fields: { otp: "Required" } },
            };
            fetchMock.mockResolvedValueOnce(jsonResponse(body, 422));
            expect(await usersApi[operation]("", null)).toEqual({
                kind: "http_error",
                status: 422,
                body,
            });
            fetchMock.mockResolvedValueOnce(
                jsonResponse(
                    { error: { message: "Invalid", fields: { otp: 123 } } },
                    422,
                ),
            );
            expect(await usersApi[operation]("", null)).toEqual({
                kind: "http_error",
                status: 422,
                body: undefined,
            });
            fetchMock.mockResolvedValueOnce(jsonResponse({}));
            expect(await usersApi[operation]("123456", null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        },
    );

    it("preserves enable success and classifies HTTP and rate-limit failures", async () => {
        fetchMock.mockResolvedValueOnce(jsonResponse({ data: "SECRET" }));
        expect(await usersApi.enableOtp(null)).toEqual({
            kind: "success",
            status: 200,
            data: { data: "SECRET" },
        });
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ error: { message: "Invalid OTP" } }, 401),
        );
        expect(await usersApi.verifyOtp("123456", null)).toEqual({
            kind: "http_error",
            status: 401,
            body: { error: { message: "Invalid OTP" } },
        });
        fetchMock.mockResolvedValueOnce(
            new Response(null, {
                status: 429,
                headers: { "Retry-After": "12" },
            }),
        );
        expect(await usersApi.disableOtp("123456", null)).toEqual({
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: 12,
        });
    });

    it("recognizes the exact required OTP field value", () => {
        expect(isOtpFields({ otp: null })).toBe(true);
        expect(isOtpFields({ otp: "Invalid" })).toBe(true);
        expect(isOtpFields({})).toBe(false);
        expect(isOtpFields({ otp: undefined })).toBe(false);
        expect(isOtpFields({ otp: 1 })).toBe(false);
    });
});
