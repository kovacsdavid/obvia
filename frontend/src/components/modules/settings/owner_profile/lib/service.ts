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
    globalRequestTimeout,
    unexpectedError,
    unexpectedFormError,
} from "@/services/utils/consts.ts";
import type {
    OwnerProfileUserInput,
    UpdateOwnerProfile,
    OwnerProfileFullResponse,
} from "@/components/modules/settings/owner_profile/lib/interface.ts";
import {
    type ProcessedJsonResponse,
    ProcessJsonResponse,
} from "@/lib/interface.ts";
import {
    isUpdateOwnerProfileResponse,
    isOwnerProfileFullResponse,
} from "@/components/modules/settings/owner_profile/lib/guards.ts";

export async function update(
    {
        name,
        contactName,
        email,
        phoneNumber,
        ownerProfileType,
        billingAddress,
        mailingAddress,
    }: OwnerProfileUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<UpdateOwnerProfile>> {
    return await fetch(`/api/owner_profile/update`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            name,
            contact_name: contactName,
            email,
            phone_number: phoneNumber,
            owner_profile_type:
                typeof ownerProfileType === "undefined"
                    ? null
                    : ownerProfileType,
            billing_address: billingAddress ?? null,
            mailing_address: mailingAddress ?? null,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isUpdateOwnerProfileResponse,
            )) ?? unexpectedFormError
        );
    });
}

export async function get_full(
    token: string | null,
): Promise<ProcessedJsonResponse<OwnerProfileFullResponse>> {
    return await fetch(`/api/owner_profile/get_full`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isOwnerProfileFullResponse)) ??
            unexpectedError
        );
    });
}
