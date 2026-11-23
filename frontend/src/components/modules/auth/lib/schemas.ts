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

import { z } from "zod";
import { successResponseSchema } from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const registerRequestSchema = z.object({
    firstName: z.string(),
    lastName: z.string(),
    email: z.string(),
    password: z.string(),
    passwordConfirm: z.string(),
});

export const loginRequestSchema = z.object({
    email: z.string(),
    password: z.string(),
    otp: nullableString,
});

export const forgottenPasswordRequestSchema = z.object({ email: z.string() });
export const newPasswordRequestSchema = z.object({
    token: z.string(),
    password: z.string(),
    password_confirm: z.string(),
});

export const loginFieldsSchema = z
    .strictObject({
        email: nullableString.optional(),
        password: nullableString.optional(),
        otp: nullableString.optional(),
    })
    .refine(
        (fields) =>
            Object.keys(fields).length > 0 &&
            Object.values(fields).every((value) => value !== undefined),
    );

export const registerFieldsSchema = z.looseObject({
    first_name: nullableString,
    last_name: nullableString,
    email: nullableString,
    password: nullableString,
    password_confirm: nullableString,
});
export const forgottenPasswordFieldsSchema = z.looseObject({
    email: nullableString,
});
export const newPasswordFieldsSchema = z.looseObject({
    token: nullableString,
    password: nullableString,
    password_confirm: nullableString,
});

export const claimsSchema = z.looseObject({
    sub: z.string(),
    exp: z.number(),
    iat: z.number(),
    nbf: z.number(),
    iss: z.string(),
    aud: z.string(),
    jti: z.string(),
    loc: z.string(),
    tz: z.string(),
    family_id: z.string().nullable(),
    active_tenant: z.string().nullable(),
});

export const userSchema = z.looseObject({
    id: z.string(),
    email: z.string(),
    first_name: nullableString,
    last_name: nullableString,
    status: z.string(),
    profile_picture_url: nullableString,
    is_mfa_enabled: z.boolean(),
});

export const loginDataSchema = z.looseObject({
    claims: claimsSchema,
    user: userSchema,
    token: z.string(),
});

export const messageDataSchema = z.looseObject({ message: z.string() });
export const logoutSuccessSchema = z.undefined();
export const claimsResponseSchema = successResponseSchema(claimsSchema);
export const loginResponseSchema = successResponseSchema(loginDataSchema);
export const messageResponseSchema = successResponseSchema(messageDataSchema);
export const enableOtpResponseSchema = successResponseSchema(z.string());
export const authLoginSuccessSchema = loginResponseSchema;
export const authMessageSuccessSchema = messageResponseSchema;
export const authClaimsSuccessSchema = claimsResponseSchema;
