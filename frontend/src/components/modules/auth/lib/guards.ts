/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import {
    type FormError,
    isCommonResponse,
    isFormError,
    isSimpleError,
    isSimpleMessageData,
    type SimpleMessageData,
    type SimpleError,
} from "@/lib/interface.ts";
import type {
    Claims,
    ClaimsResponse,
    DisableOtpResponse,
    EnableOtpResponse,
    ForgottenPasswordResponse,
    LoginData,
    LoginResponse,
    User,
    NewPasswordResponse,
    RegisterResponse,
    VerifyEmailResponse,
    VerifyOtpResponse,
    RegisterFields,
    ForgottenPasswordFields,
    NewPasswordFields,
    LoginFields,
} from "@/components/modules/auth/lib/interface.ts";

function hasNullableFields(data: unknown, fields: readonly string[]): boolean {
    return (
        typeof data === "object" &&
        data !== null &&
        fields.every((field) => {
            const value = (data as Record<string, unknown>)[field];
            return value === null || typeof value === "string";
        })
    );
}

export function isLoginFields(data: unknown): data is LoginFields {
    if (typeof data !== "object" || data === null || Array.isArray(data))
        return false;
    const entries = Object.entries(data);
    return (
        entries.length > 0 &&
        entries.every(
            ([field, value]) =>
                ["email", "password", "otp"].includes(field) &&
                (value === null || typeof value === "string"),
        )
    );
}

export function isRegisterFields(data: unknown): data is RegisterFields {
    return hasNullableFields(data, [
        "email",
        "first_name",
        "last_name",
        "password",
        "password_confirm",
    ]);
}

export function isForgottenPasswordFields(
    data: unknown,
): data is ForgottenPasswordFields {
    return hasNullableFields(data, ["email"]);
}

export function isNewPasswordFields(data: unknown): data is NewPasswordFields {
    return hasNullableFields(data, ["token", "password", "password_confirm"]);
}

export function isAuthSuccess<T>(
    data: unknown,
    guard: (value: unknown) => value is T,
): data is { data: T } {
    return (
        typeof data === "object" &&
        data !== null &&
        "data" in data &&
        guard(data.data)
    );
}

export function isClaims(data: unknown): data is Claims {
    return (
        typeof data === "object" &&
        data !== null &&
        "sub" in data &&
        typeof data.sub === "string" &&
        "exp" in data &&
        typeof data.exp === "number" &&
        "iat" in data &&
        typeof data.iat === "number" &&
        "nbf" in data &&
        typeof data.nbf === "number" &&
        "iss" in data &&
        typeof data.iss === "string" &&
        "aud" in data &&
        typeof data.aud === "string" &&
        "jti" in data &&
        typeof data.jti === "string" &&
        "loc" in data &&
        typeof data.loc === "string" &&
        "tz" in data &&
        typeof data.tz === "string" &&
        "family_id" in data &&
        (data.family_id === null || typeof data.family_id === "string") &&
        "active_tenant" in data &&
        (data.active_tenant === null || typeof data.active_tenant === "string")
    );
}

export function isClaimsResponse(data: unknown): data is ClaimsResponse {
    return isCommonResponse(data, isClaims, isSimpleError);
}

export function isUser(data: unknown): data is User {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "email" in data &&
        typeof data.email === "string" &&
        "first_name" in data &&
        (data.first_name === null || typeof data.first_name === "string") &&
        "last_name" in data &&
        (data.last_name === null || typeof data.last_name === "string") &&
        "status" in data &&
        typeof data.status === "string" &&
        "profile_picture_url" in data &&
        (data.profile_picture_url === null ||
            typeof data.profile_picture_url === "string") &&
        "is_mfa_enabled" in data &&
        typeof data.is_mfa_enabled === "boolean"
    );
}

export function isLoginData(data: unknown): data is LoginData {
    return (
        typeof data === "object" &&
        data !== null &&
        "claims" in data &&
        isClaims(data.claims) &&
        "user" in data &&
        isUser(data.user) &&
        "token" in data &&
        typeof data.token === "string"
    );
}

export function isLoginResponse(data: unknown): data is LoginResponse {
    return isCommonResponse<LoginData, FormError>(
        data,
        isLoginData,
        isFormError,
    );
}

export function isRegisterResponse(data: unknown): data is RegisterResponse {
    return isCommonResponse<SimpleMessageData, FormError>(
        data,
        isSimpleMessageData,
        isFormError,
    );
}

export function isVerifyEmailResponse(
    data: unknown,
): data is VerifyEmailResponse {
    return isCommonResponse<SimpleMessageData, SimpleError>(
        data,
        isSimpleMessageData,
        isSimpleError,
    );
}

export function isForgottenPasswordResponse(
    data: unknown,
): data is ForgottenPasswordResponse {
    return isCommonResponse<SimpleMessageData, FormError>(
        data,
        isSimpleMessageData,
        isFormError,
    );
}

export function isNewPasswordResponse(
    data: unknown,
): data is NewPasswordResponse {
    return isCommonResponse<SimpleMessageData, FormError>(
        data,
        isSimpleMessageData,
        isFormError,
    );
}

export function isEnableOtpResponse(data: unknown): data is EnableOtpResponse {
    return isCommonResponse<string, FormError>(
        data,
        (data: unknown): data is string => typeof data === "string",
        isFormError,
    );
}

export function isVerifyOtpResponse(data: unknown): data is VerifyOtpResponse {
    return isCommonResponse<SimpleMessageData, FormError>(
        data,
        isSimpleMessageData,
        isFormError,
    );
}

export function isDisableOtpResponse(
    data: unknown,
): data is DisableOtpResponse {
    return isCommonResponse<SimpleMessageData, FormError>(
        data,
        isSimpleMessageData,
        isFormError,
    );
}
