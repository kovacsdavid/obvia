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

export const inventoryReservationUserInputSchema = z.looseObject({
    id: nullableString,
    inventory_id: z.string(),
    quantity: z.string(),
    reference_type: nullableString,
    reference_id: nullableString,
    reserved_until: nullableString,
    status: z.string(),
});

export const inventoryReservationFormSchema = z
    .object(inventoryReservationUserInputSchema.shape)
    .omit({ id: true, quantity: true });

export const inventoryReservationErrorSchema = z.looseObject({
    id: nullableString,
    inventory_id: nullableString,
    quantity: nullableString,
    reference_type: nullableString,
    reference_id: nullableString,
    reserved_until: nullableString,
    status: nullableString,
});

export const inventoryReservationSchema = z.looseObject({
    id: z.string(),
    inventory_id: z.string(),
    quantity: z.string(),
    reference_type: nullableString,
    reference_id: nullableString,
    reserved_until: nullableString,
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
});

export const inventoryReservationResolvedSchema =
    inventoryReservationSchema.extend({
        created_by: z.string(),
    });
export const inventoryReservationResolvedListSchema = z.array(
    inventoryReservationResolvedSchema,
);
export const inventoryReservationResponseSchema = successResponseSchema(
    inventoryReservationSchema,
);
export const inventoryReservationResolvedResponseSchema = successResponseSchema(
    inventoryReservationResolvedSchema,
);
export const createInventoryReservationResponseSchema =
    inventoryReservationResponseSchema;
export const deleteInventoryReservationResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const paginatedInventoryReservationResolvedListResponseSchema =
    paginatedSuccessResponseSchema(inventoryReservationResolvedListSchema);
