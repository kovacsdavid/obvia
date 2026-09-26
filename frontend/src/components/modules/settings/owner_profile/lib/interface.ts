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

import {
    type CommonResponse,
    type FormErrorV2,
    type SimpleError,
} from "@/lib/interface.ts";
import {
    type Address,
    type AddressErrors,
    type Address as AddressInterface,
} from "@/components/modules/address/lib/interface";

export interface OwnerProfileUserInput {
    name: string;
    contactName: string;
    email: string;
    phoneNumber: string;
    ownerProfileType: string | undefined;
    billingAddress: AddressInterface | null | undefined;
    mailingAddress: AddressInterface | null | undefined;
}

export interface OwnerProfile {
    name: string;
    contact_name: string | null;
    email: string;
    phone_number: string | null;
    owner_profile_type: string;
    created_by_id: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
    billing_address: string | null;
    mailing_address: string | null;
}

export interface OwnerProfileErrors {
    name: string | null;
    contact_name: string | null;
    email: string | null;
    phone_number: string | null;
    owner_profile_type: string | null;
    billing_address: AddressErrors;
    mailing_address: AddressErrors;
}

export interface OwnerProfileFull {
    id: string;
    name: string;
    contact_name: string | null;
    email: string;
    phone_number: string | null;
    owner_profile_type: string;
    created_by_id: string;
    created_by: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
    billing_address: Address | null;
    mailing_address: Address | null;
}

export type UpdateOwnerProfile = CommonResponse<
    OwnerProfile,
    FormErrorV2<OwnerProfileFull>
>;

export type OwnerProfileFullResponse = CommonResponse<
    OwnerProfileFull,
    SimpleError
>;
