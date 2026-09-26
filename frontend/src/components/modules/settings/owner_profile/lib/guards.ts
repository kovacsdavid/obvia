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
    isCommonResponse,
    isFormErrorV2,
    isSimpleError,
} from "@/lib/interface.ts";
import type {
    OwnerProfileErrors,
    OwnerProfile,
    OwnerProfileFull,
    OwnerProfileFullResponse,
    UpdateOwnerProfileResponse,
} from "@/components/modules/settings/owner_profile/lib/interface";
import {
    isAddress,
    isAddressError,
} from "@/components/modules/address/lib/guards";

export function isOwnerProfile(data: unknown): data is OwnerProfile {
    return (
        typeof data === "object" &&
        data !== null &&
        "name" in data &&
        typeof data.name === "string" &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        typeof data.email === "string" &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "owner_profile_type" in data &&
        typeof data.owner_profile_type === "string" &&
        "created_by_id" in data &&
        typeof data.created_by_id === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string") &&
        "billing_address" in data &&
        (data.billing_address === null ||
            typeof data.billing_address === "string") &&
        "mailing_address" in data &&
        (data.mailing_address === null ||
            typeof data.mailing_address === "string")
    );
}

export function isOwnerProfileFull(data: unknown): data is OwnerProfileFull {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "name" in data &&
        typeof data.name === "string" &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        typeof data.email === "string" &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "owner_profile_type" in data &&
        typeof data.owner_profile_type === "string" &&
        "created_by_id" in data &&
        typeof data.created_by_id === "string" &&
        "created_by" in data &&
        typeof data.created_by === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string") &&
        "billing_address" in data &&
        (data.billing_address === null || isAddress(data.billing_address)) &&
        "mailing_address" in data &&
        (data.mailing_address === null || isAddress(data.mailing_address))
    );
}

export function isOwnerProfileErrors(
    data: unknown,
): data is OwnerProfileErrors {
    return (
        typeof data === "object" &&
        data !== null &&
        "name" in data &&
        (data.name === null || typeof data.name === "string") &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        (data.email === null || typeof data.email === "string") &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "owner_profile_type" in data &&
        (data.owner_profile_type === null ||
            typeof data.owner_profile_type === "string") &&
        "billing_address" in data &&
        isAddressError(data.billing_address) &&
        "mailing_address" in data &&
        isAddressError(data.mailing_address)
    );
}
export function isUpdateOwnerProfileResponse(
    data: unknown,
): data is UpdateOwnerProfileResponse {
    return isCommonResponse(data, isOwnerProfile, (data: unknown) =>
        isFormErrorV2<OwnerProfileErrors>(data, isOwnerProfileErrors),
    );
}

export function isOwnerProfileFullResponse(
    data: unknown,
): data is OwnerProfileFullResponse {
    return isCommonResponse(data, isOwnerProfileFull, isSimpleError);
}
