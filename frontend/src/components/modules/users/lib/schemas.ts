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

export const createUserSchema = z.looseObject({
    email: z.string(),
    lastName: z.string(),
    firstName: z.string(),
    phone: z.string(),
    status: z.string(),
});

export const otpFieldsSchema = z.looseObject({
    otp: z.string().nullable(),
});

export const otpMessageResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);

export const unimplementedResponseSchema = z.never();
