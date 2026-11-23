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

import type { z } from "zod";
import type {
    claimsSchema,
    claimsResponseSchema,
    enableOtpResponseSchema,
    userSchema,
    loginDataSchema,
    loginResponseSchema,
    messageResponseSchema,
    registerRequestSchema,
    loginRequestSchema,
    forgottenPasswordRequestSchema,
    newPasswordRequestSchema,
    registerFieldsSchema,
    loginFieldsSchema,
    forgottenPasswordFieldsSchema,
    newPasswordFieldsSchema,
} from "@/components/modules/auth/lib/schemas";

export type User = z.infer<typeof userSchema>;
export type RegisterRequest = z.infer<typeof registerRequestSchema>;
export type LoginRequest = z.infer<typeof loginRequestSchema>;
export type LoginData = z.infer<typeof loginDataSchema>;
export type ForgottenPasswordRequest = z.infer<
    typeof forgottenPasswordRequestSchema
>;
export type NewPasswordRequest = z.infer<typeof newPasswordRequestSchema>;
export type RegisterFields = z.infer<typeof registerFieldsSchema>;
export type LoginFields = z.infer<typeof loginFieldsSchema>;
export type ForgottenPasswordFields = z.infer<
    typeof forgottenPasswordFieldsSchema
>;
export type NewPasswordFields = z.infer<typeof newPasswordFieldsSchema>;

export type Claims = z.infer<typeof claimsSchema>;

export type RegisterResponse = z.infer<typeof messageResponseSchema>;
export type LoginResponse = z.infer<typeof loginResponseSchema>;
export type ClaimsResponse = z.infer<typeof claimsResponseSchema>;
export type VerifyEmailResponse = z.infer<typeof messageResponseSchema>;
export type ForgottenPasswordResponse = z.infer<typeof messageResponseSchema>;
export type NewPasswordResponse = z.infer<typeof messageResponseSchema>;
export type EnableOtpResponse = z.infer<typeof enableOtpResponseSchema>;
export type VerifyOtpResponse = z.infer<typeof messageResponseSchema>;
export type DisableOtpResponse = z.infer<typeof messageResponseSchema>;
