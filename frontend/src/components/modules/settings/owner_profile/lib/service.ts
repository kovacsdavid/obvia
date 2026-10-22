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

import type {
    OwnerProfileUserInput,
    UpdateOwnerProfileResponse,
    OwnerProfileFullResponse,
    OwnerProfileErrors,
} from "@/components/modules/settings/owner_profile/lib/interface";
import {
    isUpdateOwnerProfileResponse,
    isOwnerProfileFullResponse,
    isOwnerProfileErrors,
} from "@/components/modules/settings/owner_profile/lib/guards";
import type { ApiResult } from "@/lib/api/types";
import { apiRequest } from "@/lib/api/request";

export function update(
    input: OwnerProfileUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateOwnerProfileResponse, OwnerProfileErrors>> {
    return apiRequest(`/api/owner_profile/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateOwnerProfileResponse,
        formErrorGuard: isOwnerProfileErrors,
        body: {
            id: input.id,
            name: input.name,
            contact_name: input.contactName,
            email: input.email,
            website: input.website,
            document_prefix: input.documentPrefix,
            phone_number: input.phoneNumber,
            owner_profile_type: input.ownerProfileType ?? null,
            billing_address: input.billingAddress ?? null,
            mailing_address: input.mailingAddress ?? null,
        },
    });
}

export function get_full(
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<OwnerProfileFullResponse>> {
    return apiRequest(`/api/owner_profile/get_full`, {
        token,
        signal,
        guard: isOwnerProfileFullResponse,
    });
}
