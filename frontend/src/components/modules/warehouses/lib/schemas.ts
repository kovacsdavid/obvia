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
import {
    paginatedSuccessResponseSchema,
    successResponseSchema,
} from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const warehouseUserInputSchema = z.looseObject({
    id: nullableString,
    name: z.string(),
    contactName: z.string(),
    contactPhone: z.string(),
    status: z.string(),
});

export const warehouseErrorSchema = z.looseObject({
    id: nullableString,
    name: nullableString,
    contact_name: nullableString,
    contact_phone: nullableString,
    status: nullableString,
});

export const warehouseSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    contact_name: nullableString,
    contact_phone: nullableString,
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const warehouseResolvedSchema = warehouseSchema.extend({
    created_by: z.string(),
});
export const warehouseResolvedListSchema = z.array(warehouseResolvedSchema);
export const warehouseResponseSchema = successResponseSchema(warehouseSchema);
export const createWarehouseResponseSchema = warehouseResponseSchema;
export const updateWarehouseResponseSchema = warehouseResponseSchema;
export const deleteWarehouseResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const warehouseResolvedResponseSchema = successResponseSchema(
    warehouseResolvedSchema,
);
export const paginatedWarehouseResolvedListResponseSchema =
    paginatedSuccessResponseSchema(warehouseResolvedListSchema);
