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
import { currencySchema } from "@/components/modules/currencies/lib/schemas";
import { customerFullSchema } from "@/components/modules/customers/lib/schemas";
import {
    paginatedSuccessResponseSchema,
    successResponseSchema,
} from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const handoverAndAcceptanceUserInputSchema = z.looseObject({
    id: nullableString,
    customer_id: z.string(),
    item_make_id: z.string(),
    new_item_make: z.string(),
    item_model_id: z.string(),
    new_item_model: z.string(),
    item_identifier_type_id: z.string(),
    new_item_identifier_type: z.string(),
    item_identifier_value: z.string(),
    reported_issue: z.string(),
    condition_at_receipt: z.string(),
    included_accessories: z.array(z.string()),
    authorized_work: z.string(),
    repair_limit: z.string(),
    currency_code: z.string(),
    terms_version: z.string(),
    received_at: z.string(),
});

export const handoverFormSchema = z
    .object(handoverAndAcceptanceUserInputSchema.shape)
    .omit({ id: true });

export const handoverAndAcceptanceSchema = z.looseObject({
    id: z.string(),
    display_id: z.string(),
    customer_id: z.string(),
    item_make_id: nullableString,
    item_model_id: nullableString,
    item_identifier_type_id: z.string(),
    item_identifier_value: z.string(),
    reported_issue: nullableString,
    condition_at_receipt: nullableString,
    included_accessories: z.array(z.string()).nullable(),
    authorized_work: nullableString,
    repair_limit: nullableString,
    currency_code: nullableString,
    terms_version: nullableString,
    accepted_by: z.string(),
    received_at: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const handoverAndAcceptanceErrorSchema = z.looseObject({
    id: nullableString,
    customer_id: nullableString,
    item_make_id: nullableString,
    new_item_make: nullableString,
    item_model_id: nullableString,
    new_item_model: nullableString,
    item_identifier_type_id: nullableString,
    new_item_identifier_type: nullableString,
    item_identifier_value: nullableString,
    reported_issue: nullableString,
    condition_at_receipt: nullableString,
    included_accessories: z.array(nullableString),
    authorized_work: nullableString,
    repair_limit: nullableString,
    currency_code: nullableString,
    terms_version: nullableString,
    received_at: nullableString,
});

export const itemIdentifierTypeSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    is_active: z.boolean(),
    created_at: z.string(),
});
export const itemMakeSchema = itemIdentifierTypeSchema;
export const itemModelFullSchema = itemMakeSchema.extend({
    make: itemMakeSchema,
});
export const handoverCurrencySchema = currencySchema;

export const handoverUserSchema = z.looseObject({
    id: z.string(),
    email: z.string(),
    first_name: nullableString,
    last_name: nullableString,
    phone: nullableString,
    status: z.string(),
    profile_picture_url: nullableString,
    locale: z.string(),
    invited_by: nullableString,
    email_verified_at: nullableString,
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const handoverAndAcceptanceFullSchema = handoverAndAcceptanceSchema
    .omit({
        customer_id: true,
        item_make_id: true,
        item_model_id: true,
        item_identifier_type_id: true,
    })
    .extend({
        customer: customerFullSchema,
        item_make: itemMakeSchema.nullable(),
        item_model: itemModelFullSchema.nullable(),
        item_identifier_type: itemIdentifierTypeSchema,
        currency_code: handoverCurrencySchema.nullable(),
        accepted_by: handoverUserSchema,
    });

export const handoverAndAcceptanceFullListSchema = z.array(
    handoverAndAcceptanceFullSchema,
);
export const handoverAndAcceptanceResponseSchema = successResponseSchema(
    handoverAndAcceptanceSchema,
);
export const createHandoverAndAcceptanceResponseSchema =
    handoverAndAcceptanceResponseSchema;
export const updateHandoverAndAcceptanceResponseSchema =
    handoverAndAcceptanceResponseSchema;
export const deleteHandoverAndAcceptanceResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const handoverAndAcceptanceFullResponseSchema = successResponseSchema(
    handoverAndAcceptanceFullSchema,
);
export const paginatedHandoverAndAcceptanceFullListResponseSchema =
    paginatedSuccessResponseSchema(handoverAndAcceptanceFullListSchema);
