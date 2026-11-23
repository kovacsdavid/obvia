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

export const taxUserInputSchema = z.looseObject({
    id: nullableString,
    rate: nullableString,
    description: z.string(),
    countryCode: z.string(),
    taxCategory: z.string(),
    isRateApplicable: z.boolean().nullable(),
    legalText: nullableString,
    reportingCode: nullableString,
    isDefault: z.boolean(),
    status: z.string(),
});

export const taxErrorSchema = z.looseObject({
    id: nullableString,
    rate: nullableString,
    description: nullableString,
    country_code: nullableString,
    tax_category: nullableString,
    is_rate_applicable: nullableString,
    legal_text: nullableString,
    reporting_code: nullableString,
    is_default: nullableString,
    status: nullableString,
});

export const taxSchema = z.looseObject({
    id: z.string(),
    rate: nullableString,
    description: z.string(),
    country_code: z.string(),
    tax_category: z.string(),
    is_rate_applicable: z.boolean(),
    legal_text: nullableString,
    reporting_code: nullableString,
    is_default: z.boolean(),
    status: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const taxResolvedSchema = taxSchema.extend({
    country: z.string(),
    created_by: z.string(),
});
export const taxResolvedListSchema = z.array(taxResolvedSchema);
export const taxResponseSchema = successResponseSchema(taxSchema);
export const createTaxResponseSchema = taxResponseSchema;
export const updateTaxResponseSchema = taxResponseSchema;
export const deleteTaxResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const taxResolvedResponseSchema =
    successResponseSchema(taxResolvedSchema);
export const paginatedTaxResolvedListResponseSchema =
    paginatedSuccessResponseSchema(taxResolvedListSchema);
