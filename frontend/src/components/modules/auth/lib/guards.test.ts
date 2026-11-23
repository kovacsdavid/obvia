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

import { describe, expect, it } from "vitest";
import {
    isAuthSuccess,
    isClaims,
    isLoginFields,
    isLoginResponse,
    isRegisterFields,
    isUser,
} from "./guards";

const claims = {
    sub: "user",
    exp: 2,
    iat: 1,
    nbf: 1,
    iss: "issuer",
    aud: "audience",
    jti: "id",
    loc: "hu",
    tz: "Europe/Budapest",
    family_id: null,
    active_tenant: null,
};
const user = {
    id: "user",
    email: "user@example.com",
    first_name: null,
    last_name: null,
    status: "active",
    profile_picture_url: null,
    is_mfa_enabled: false,
};

describe("auth schema guards", () => {
    it("validates nullable claims and user fields while accepting extra properties", () => {
        expect(isClaims({ ...claims, extra: true })).toBe(true);
        expect(
            isClaims({
                ...claims,
                active_tenant: "tenant",
                family_id: "family",
            }),
        ).toBe(true);
        expect(isClaims({ ...claims, family_id: undefined })).toBe(false);
        expect(isUser({ ...user, extra: true })).toBe(true);
        expect(isUser({ ...user, profile_picture_url: 1 })).toBe(false);
    });

    it("preserves login and registration field validation", () => {
        expect(isLoginFields({ otp: null })).toBe(true);
        expect(isLoginFields({})).toBe(false);
        expect(isLoginFields({ otp: undefined })).toBe(false);
        expect(isLoginFields({ otp: "123", extra: null })).toBe(false);
        expect(
            isRegisterFields({
                first_name: null,
                last_name: null,
                email: null,
                password: null,
                password_confirm: null,
                extra: 1,
            }),
        ).toBe(true);
        expect(isRegisterFields({ email: null })).toBe(false);
    });

    it("validates success envelopes and nested payloads", () => {
        const data = { claims, user, token: "token" };
        expect(isLoginResponse({ data, meta: null, extra: true })).toBe(true);
        expect(isLoginResponse({})).toBe(false);
        expect(isLoginResponse({ data: null })).toBe(false);
        expect(isLoginResponse({ data, error: { message: "error" } })).toBe(
            false,
        );
        expect(
            isLoginResponse({
                data: { ...data, claims: { ...claims, tz: null } },
            }),
        ).toBe(false);
    });

    it("applies the shared envelope contract to the generic auth guard", () => {
        const isString = (value: unknown): value is string =>
            typeof value === "string";
        expect(
            isAuthSuccess({ data: "ok", meta: null, extra: true }, isString),
        ).toBe(true);
        expect(isAuthSuccess({ data: "ok", meta: { page: 1 } }, isString)).toBe(
            false,
        );
        expect(
            isAuthSuccess(
                { data: "ok", error: { message: "error" } },
                isString,
            ),
        ).toBe(false);
        expect(isAuthSuccess({ data: 1 }, isString)).toBe(false);
        expect(isAuthSuccess({}, isString)).toBe(false);
    });
});
