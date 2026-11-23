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
import { schemaGuard } from "@/lib/api/schemas";
import {
    activeDatabaseResponseSchema,
    createDatabaseSchema,
    databaseErrorSchema,
    databaseListSchema,
    databaseResponseSchema,
    databaseSchema,
    newTokenResponseSchema,
    paginatedDatabaseListResponseSchema,
} from "./schemas";

export type CreateDatabase = z.infer<typeof createDatabaseSchema>;
export type DatabaseError = z.infer<typeof databaseErrorSchema>;
export const isDatabaseError = schemaGuard(databaseErrorSchema);

export type CreateDatabaseResponse = z.infer<typeof databaseResponseSchema>;

export const isCreateDatabaseResponse = schemaGuard(databaseResponseSchema);

export type Database = z.infer<typeof databaseSchema>;

export type DatabaseResponse = z.infer<typeof databaseResponseSchema>;

export const isDatabaseResponse = schemaGuard(databaseResponseSchema);

export type DatabaseList = z.infer<typeof databaseListSchema>;
export const isDatabase = schemaGuard(databaseSchema);
export const isDatabaseList = schemaGuard(databaseListSchema);

export type PaginatedDatabaseListResponse = z.infer<
    typeof paginatedDatabaseListResponseSchema
>;

export const isPaginatedDatabaseListResponse = schemaGuard(
    paginatedDatabaseListResponseSchema,
);
export type NewTokenResponse = z.infer<typeof newTokenResponseSchema>;
export const isNewTokenResponse = schemaGuard(newTokenResponseSchema);

export type ActiveDatabaseResponse = z.infer<
    typeof activeDatabaseResponseSchema
>;

export const isActiveDatabaseResponse = schemaGuard(
    activeDatabaseResponseSchema,
);
