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

import type {
    ClaimsResponse,
    ForgottenPasswordFields,
    ForgottenPasswordRequest,
    ForgottenPasswordResponse,
    LoginResponse,
    LoginFields,
    LoginRequest,
    NewPasswordFields,
    NewPasswordRequest,
    NewPasswordResponse,
    RegisterFields,
    RegisterRequest,
    RegisterResponse,
    VerifyEmailResponse,
} from "@/components/modules/auth/lib/interface.ts";
import {
    isForgottenPasswordFields,
    isLoginFields,
    isNewPasswordFields,
    isRegisterFields,
} from "@/components/modules/auth/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";
import { schemaGuard } from "@/lib/api/schemas";
import {
    authClaimsSuccessSchema,
    authLoginSuccessSchema,
    authMessageSuccessSchema,
    logoutSuccessSchema,
} from "./schemas";

const loginGuard = schemaGuard(authLoginSuccessSchema);
const messageGuard = schemaGuard(authMessageSuccessSchema);
const claimsGuard = schemaGuard(authClaimsSuccessSchema);

export async function login(
    { email, password, otp }: LoginRequest,
    signal?: AbortSignal,
): Promise<ApiResult<LoginResponse, LoginFields>> {
    return apiRequest("/api/auth/login", {
        method: "POST",
        body: {
            email,
            password,
            ...(typeof otp === "string" && otp.trim().length > 0
                ? { otp }
                : {}),
        },
        guard: loginGuard,
        formErrorGuard: isLoginFields,
        signal,
    });
}

export async function refresh(): Promise<ApiResult<LoginResponse>> {
    return apiRequest("/api/auth/t/refresh", {
        method: "POST",
        guard: loginGuard,
    });
}

export async function logout(): Promise<ApiResult<undefined>> {
    return apiRequest("/api/auth/t/logout", {
        method: "POST",
        guard: schemaGuard(logoutSuccessSchema),
    });
}

export async function register(
    { firstName, lastName, email, password, passwordConfirm }: RegisterRequest,
    signal?: AbortSignal,
): Promise<ApiResult<RegisterResponse, RegisterFields>> {
    return apiRequest("/api/auth/register", {
        method: "POST",
        body: {
            first_name: firstName,
            last_name: lastName,
            email,
            password,
            password_confirm: passwordConfirm,
        },
        guard: messageGuard,
        formErrorGuard: isRegisterFields,
        signal,
    });
}

export async function get_claims(
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ClaimsResponse>> {
    return apiRequest("/api/auth/get_claims", {
        token,
        signal,
        guard: claimsGuard,
    });
}

export async function verfiy_email(
    id: string,
    signal?: AbortSignal,
): Promise<ApiResult<VerifyEmailResponse>> {
    return apiRequest(`/api/auth/verify_email?${new URLSearchParams({ id })}`, {
        signal,
        guard: messageGuard,
    });
}

export async function forgottenPassword(
    { email }: ForgottenPasswordRequest,
    signal?: AbortSignal,
): Promise<ApiResult<ForgottenPasswordResponse, ForgottenPasswordFields>> {
    return apiRequest("/api/auth/forgotten_password", {
        method: "POST",
        body: { email },
        guard: messageGuard,
        formErrorGuard: isForgottenPasswordFields,
        timeoutMs: 10000,
        signal,
    });
}

export async function newPassword(
    input: NewPasswordRequest,
    signal?: AbortSignal,
): Promise<ApiResult<NewPasswordResponse, NewPasswordFields>> {
    return apiRequest("/api/auth/new_password", {
        method: "POST",
        body: {
            token: input.token,
            password: input.password,
            password_confirm: input.password_confirm,
        },
        guard: messageGuard,
        formErrorGuard: isNewPasswordFields,
        signal,
    });
}
