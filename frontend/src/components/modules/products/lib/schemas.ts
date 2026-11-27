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

export const productUserInputSchema = z.looseObject({
    id: nullableString,
    name: z.string(),
    description: z.string(),
    unit_of_measure_id: z.string(),
    new_unit_of_measure: z.string(),
    status: z.string(),
});

export const productErrorSchema = z.looseObject({
    id: nullableString,
    name: nullableString,
    description: nullableString,
    unit_of_measure_id: nullableString,
    new_unit_of_measure: nullableString,
    status: nullableString,
});

export const productSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    description: nullableString,
    unit_of_measure_id: z.string(),
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const productResolvedSchema = productSchema.extend({
    unit_of_measure: z.string(),
    created_by: z.string(),
});

export const productResolvedListSchema = z.array(productResolvedSchema);
export const productResponseSchema = successResponseSchema(productSchema);
export const createProductResponseSchema = productResponseSchema;
export const updateProductResponseSchema = productResponseSchema;
export const deleteProductResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const productResolvedResponseSchema = successResponseSchema(
    productResolvedSchema,
);
export const paginatedProductResolvedListResponseSchema =
    paginatedSuccessResponseSchema(productResolvedListSchema);
