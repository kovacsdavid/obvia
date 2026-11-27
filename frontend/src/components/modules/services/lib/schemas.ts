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

export const serviceUserInputSchema = z.looseObject({
    id: nullableString,
    name: z.string(),
    description: nullableString,
    default_price: nullableString,
    default_tax_id: nullableString,
    currency_code: nullableString,
    status: z.string(),
    unit_of_measure_id: z.string(),
    new_unit_of_measure: z.string(),
});

export const serviceFormSchema = z
    .object(serviceUserInputSchema.shape)
    .omit({ id: true, default_price: true })
    .extend({
        description: z.string(),
        default_tax_id: z.string(),
        currency_code: z.string(),
    });

export const serviceErrorSchema = z.looseObject({
    id: nullableString,
    name: nullableString,
    description: nullableString,
    default_price: nullableString,
    default_tax_id: nullableString,
    currency_code: nullableString,
    status: nullableString,
    unit_of_measure_id: nullableString,
    new_unit_of_measure: nullableString,
});

export const serviceSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    description: nullableString,
    default_price: nullableString,
    default_tax_id: nullableString,
    currency_code: nullableString,
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
    unit_of_measure_id: z.string(),
});

export const serviceResolvedSchema = serviceSchema.extend({
    default_tax: nullableString,
    created_by: z.string(),
    unit_of_measure: z.string(),
});

export const serviceResolvedListSchema = z.array(serviceResolvedSchema);
export const serviceResponseSchema = successResponseSchema(serviceSchema);
export const createServiceResponseSchema = serviceResponseSchema;
export const updateServiceResponseSchema = serviceResponseSchema;
export const deleteServiceResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const serviceResolvedResponseSchema = successResponseSchema(
    serviceResolvedSchema,
);
export const paginatedServiceResolvedListResponseSchema =
    paginatedSuccessResponseSchema(serviceResolvedListSchema);
