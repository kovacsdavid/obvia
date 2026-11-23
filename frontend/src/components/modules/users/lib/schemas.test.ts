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
import {
    isOtpFields,
    isOtpMessageResponse,
    isUnimplementedResponse,
} from "./guards";
import { disableOtp, verifyOtp } from "./service";

afterEach(() => vi.unstubAllGlobals());

describe("users schemas", () => {
    it("validates OTP fields and success envelopes without discarding extras", () => {
        expect(isOtpFields({ otp: null, extra: 1 })).toBe(true);
        expect(isOtpFields({})).toBe(false);
        expect(isOtpFields({ otp: 1 })).toBe(false);
        expect(
            isOtpMessageResponse({
                meta: null,
                data: { message: "Done", extra: 1 },
            }),
        ).toBe(true);
        expect(isOtpMessageResponse({ data: null })).toBe(false);
        expect(
            isOtpMessageResponse({
                data: { message: "Done" },
                error: { message: "Error" },
            }),
        ).toBe(false);
        expect(isUnimplementedResponse(undefined)).toBe(false);
        expect(isUnimplementedResponse({ data: {} })).toBe(false);
    });

    it("returns the original success payload including extra fields", async () => {
        const body = { data: { message: "Done", extra: 1 }, trace: "request" };
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                new Response(JSON.stringify(body), {
                    headers: { "Content-Type": "application/json" },
                }),
            ),
        );
        expect(await verifyOtp("123456", null)).toEqual({
            kind: "success",
            status: 200,
            data: body,
        });
    });

    it.each([verifyOtp, disableOtp])(
        "rejects non-JSON success responses",
        async (operation) => {
            const pdf = new Response("pdf", {
                headers: { "Content-Type": "application/pdf" },
            });
            Object.defineProperty(pdf, "bytes", {
                value: async () => new Uint8Array([1, 2]),
            });
            vi.stubGlobal(
                "fetch",
                vi
                    .fn()
                    .mockResolvedValueOnce(new Response(null, { status: 204 }))
                    .mockResolvedValueOnce(
                        new Response("text", {
                            headers: { "Content-Type": "text/plain" },
                        }),
                    )
                    .mockResolvedValueOnce(pdf),
            );
            expect(await operation("123456", null)).toEqual({
                kind: "invalid_response",
                status: 204,
            });
            expect(await operation("123456", null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
            expect(await operation("123456", null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        },
    );
});
