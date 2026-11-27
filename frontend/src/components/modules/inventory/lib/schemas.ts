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

export const inventoryUserInputSchema = z.looseObject({
    id: nullableString,
    product_id: z.string(),
    warehouse_id: z.string(),
    minimum_stock: z.string(),
    maximum_stock: z.string(),
    currency_code: z.string(),
    status: z.string(),
});

export const inventoryErrorSchema = z.looseObject({
    id: nullableString,
    product_id: nullableString,
    warehouse_id: nullableString,
    minimum_stock: nullableString,
    maximum_stock: nullableString,
    currency_code: nullableString,
    status: nullableString,
});

export const inventorySchema = z.looseObject({
    id: z.string(),
    product_id: z.string(),
    warehouse_id: z.string(),
    quantity_on_hand: z.string(),
    quantity_reserved: z.string(),
    quantity_available: z.string(),
    minimum_stock: nullableString,
    maximum_stock: nullableString,
    currency_code: z.string(),
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const inventoryResolvedSchema = inventorySchema.extend({
    product: z.string(),
    warehouse: z.string(),
    currency: z.string(),
    created_by: z.string(),
});
export const inventoryResolvedListSchema = z.array(inventoryResolvedSchema);
export const inventoryResponseSchema = successResponseSchema(inventorySchema);
export const inventoryResolvedResponseSchema = successResponseSchema(
    inventoryResolvedSchema,
);
export const createInventoryResponseSchema = inventoryResponseSchema;
export const updateInventoryResponseSchema = inventoryResponseSchema;
export const deleteInventoryResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const paginatedInventoryResolvedListResponseSchema =
    paginatedSuccessResponseSchema(inventoryResolvedListSchema);
