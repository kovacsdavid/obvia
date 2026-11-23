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

import { z } from "zod";
import { schemaGuard, successResponseSchema } from "@/lib/api/schemas";
import {
    claimsSchema,
    claimsResponseSchema,
    enableOtpResponseSchema,
    forgottenPasswordFieldsSchema,
    loginDataSchema,
    loginFieldsSchema,
    loginResponseSchema,
    messageResponseSchema,
    newPasswordFieldsSchema,
    registerFieldsSchema,
    userSchema,
} from "./schemas";

export const isLoginFields = schemaGuard(loginFieldsSchema);

export const isRegisterFields = schemaGuard(registerFieldsSchema);

export const isForgottenPasswordFields = schemaGuard(
    forgottenPasswordFieldsSchema,
);

export const isNewPasswordFields = schemaGuard(newPasswordFieldsSchema);

export function isAuthSuccess<T>(
    data: unknown,
    guard: (value: unknown) => value is T,
): data is { data: T } {
    return successResponseSchema(z.custom<T>(guard)).safeParse(data).success;
}

export const isClaims = schemaGuard(claimsSchema);
export const isClaimsResponse = schemaGuard(claimsResponseSchema);
export const isUser = schemaGuard(userSchema);
export const isLoginData = schemaGuard(loginDataSchema);
export const isLoginResponse = schemaGuard(loginResponseSchema);
export const isRegisterResponse = schemaGuard(messageResponseSchema);
export const isVerifyEmailResponse = schemaGuard(messageResponseSchema);
export const isForgottenPasswordResponse = schemaGuard(messageResponseSchema);
export const isNewPasswordResponse = schemaGuard(messageResponseSchema);
export const isEnableOtpResponse = schemaGuard(enableOtpResponseSchema);
export const isVerifyOtpResponse = schemaGuard(messageResponseSchema);
export const isDisableOtpResponse = schemaGuard(messageResponseSchema);
