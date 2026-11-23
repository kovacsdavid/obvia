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
    addressErrorsSchema,
    addressSchema,
} from "@/components/modules/address/lib/schemas";
import { successResponseSchema } from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const ownerProfileUserInputSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    contactName: z.string(),
    email: z.string(),
    website: z.string(),
    documentPrefix: z.string(),
    phoneNumber: z.string(),
    ownerProfileType: z.string().optional(),
    billingAddress: addressSchema.nullish(),
    mailingAddress: addressSchema.nullish(),
});

export const ownerProfileSchema = z.looseObject({
    id: z.string(),
    name: z.string(),
    contact_name: nullableString,
    email: z.string(),
    website: nullableString,
    document_prefix: nullableString,
    phone_number: nullableString,
    owner_profile_type: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    billing_address: nullableString,
    mailing_address: nullableString,
});

export const ownerProfileFullSchema = ownerProfileSchema.extend({
    billing_address: addressSchema.nullable(),
    mailing_address: addressSchema.nullable(),
});

export const ownerProfileErrorsSchema = z.looseObject({
    name: nullableString,
    contact_name: nullableString,
    email: nullableString,
    website: nullableString,
    document_prefix: nullableString,
    phone_number: nullableString,
    owner_profile_type: nullableString,
    billing_address: addressErrorsSchema,
    mailing_address: addressErrorsSchema,
});

export const ownerProfileFullResponseSchema = successResponseSchema(
    ownerProfileFullSchema,
);
export const updateOwnerProfileResponseSchema = ownerProfileFullResponseSchema;
