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

export const inventoryMovementUserInputSchema = z.looseObject({
    id: nullableString,
    inventory_id: z.string(),
    movement_type: z.string(),
    quantity: z.string(),
    reference_type: nullableString,
    reference_id: nullableString,
    unit_price: nullableString,
    total_price: nullableString,
    tax_id: z.string(),
});

export const inventoryMovementErrorSchema = z.looseObject({
    id: nullableString,
    inventory_id: nullableString,
    movement_type: nullableString,
    quantity: nullableString,
    reference_type: nullableString,
    reference_id: nullableString,
    unit_price: nullableString,
    total_price: nullableString,
    tax_id: nullableString,
});

export const inventoryMovementSchema = z.looseObject({
    id: z.string(),
    inventory_id: z.string(),
    movement_type: z.string(),
    quantity: z.string(),
    reference_type: nullableString,
    reference_id: nullableString,
    unit_price: nullableString,
    total_price: nullableString,
    tax_id: z.string(),
    movement_date: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
});

export const inventoryMovementResolvedSchema = inventoryMovementSchema.extend({
    tax: nullableString,
    created_by: z.string(),
});
export const inventoryMovementResolvedListSchema = z.array(
    inventoryMovementResolvedSchema,
);
export const inventoryMovementResponseSchema = successResponseSchema(
    inventoryMovementSchema,
);
export const inventoryMovementResolvedResponseSchema = successResponseSchema(
    inventoryMovementResolvedSchema,
);
export const createInventoryMovementResponseSchema =
    inventoryMovementResponseSchema;
export const deleteInventoryMovementResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const paginatedInventoryMovementResolvedListResponseSchema =
    paginatedSuccessResponseSchema(inventoryMovementResolvedListSchema);
