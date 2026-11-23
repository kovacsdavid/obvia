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
import { claimsSchema } from "@/components/modules/auth/lib/schemas";
import {
    paginatedSuccessResponseSchema,
    successResponseSchema,
} from "@/lib/api/schemas";

export const createDatabaseSchema = z.object({ name: z.string() });
export const databaseErrorSchema = z.looseObject({
    name: z.string().nullable(),
});
export const databaseSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    db_host: z.string(),
    db_port: z.number(),
    db_name: z.string(),
    db_user: z.string(),
    db_password: z.string(),
    db_max_pool_size: z.number(),
    db_ssl_mode: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: z.string().nullable(),
});
export const databaseListSchema = z.array(databaseSchema);
export const newTokenResponseSchema = z.looseObject({
    token: z.string(),
    claims: claimsSchema,
});
export const databaseResponseSchema = successResponseSchema(databaseSchema);
export const paginatedDatabaseListResponseSchema =
    paginatedSuccessResponseSchema(databaseListSchema);
export const activeDatabaseResponseSchema = successResponseSchema(
    newTokenResponseSchema,
);
