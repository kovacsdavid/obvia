/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
    type CreateCustomerResponse,
    type CustomerErrors,
    type CustomerFullResponse,
    type CustomerResponse,
    type CustomerUserInput,
    type DeleteCustomerResponse,
    type PaginatedCustomerResolvedListResponse,
    type UpdateCustomerResponse,
} from "@/components/modules/customers/lib/interface.ts";
import {
    isCreateCustomerResponse,
    isCustomerErrors,
    isCustomerFullResponse,
    isCustomerResponse,
    isDeleteCustomerResponse,
    isPaginatedCustomerResolvedListResponse,
    isUpdateCustomerResponse,
} from "@/components/modules/customers/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    {
        id,
        name,
        contactName,
        email,
        phoneNumber,
        status,
        customerType,
        billingAddress,
        mailingAddress,
    }: CustomerUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateCustomerResponse, CustomerErrors>> {
    return apiRequest(`/api/customers/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateCustomerResponse,
        formErrorGuard: isCustomerErrors,
        body: {
            id,
            name,
            contact_name: contactName,
            email,
            phone_number: phoneNumber,
            status: typeof status === "undefined" ? null : status,
            customer_type:
                typeof customerType === "undefined" ? null : customerType,
            billing_address: billingAddress ?? null,
            mailing_address: mailingAddress ?? null,
        },
    });
}

export async function update(
    {
        id,
        name,
        contactName,
        email,
        phoneNumber,
        status,
        customerType,
        billingAddress,
        mailingAddress,
    }: CustomerUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateCustomerResponse, CustomerErrors>> {
    return apiRequest(`/api/customers/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateCustomerResponse,
        formErrorGuard: isCustomerErrors,
        body: {
            id,
            name,
            contact_name: contactName,
            email,
            phone_number: phoneNumber,
            status: typeof status === "undefined" ? null : status,
            customer_type:
                typeof customerType === "undefined" ? null : customerType,
            billing_address: billingAddress ?? null,
            mailing_address: mailingAddress ?? null,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedCustomerResolvedListResponse>> {
    const uri =
        query === null
            ? `/api/customers/list`
            : `/api/customers/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedCustomerResolvedListResponse,
    });
}

export async function get_full(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CustomerFullResponse>> {
    return apiRequest(`/api/customers/get_full?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isCustomerFullResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CustomerResponse>> {
    return apiRequest(`/api/customers/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isCustomerResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteCustomerResponse>> {
    return apiRequest(`/api/customers/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteCustomerResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/customers/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
