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

import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import type {
    CreateTaxResponse,
    DeleteTaxResponse,
    PaginatedTaxResolvedListResponse,
    TaxError,
    TaxResolvedResponse,
    TaxResponse,
    TaxUserInput,
    UpdateTaxResponse,
} from "./interface";
import {
    isCreateTaxResponse,
    isDeleteTaxResponse,
    isPaginatedTaxResolvedListResponse,
    isTaxError,
    isTaxResolvedResponse,
    isTaxResponse,
    isUpdateTaxResponse,
} from "./guards";

export async function create(
    input: TaxUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateTaxResponse, TaxError>> {
    return apiRequest(`/api/taxes/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateTaxResponse,
        formErrorGuard: isTaxError,
        body: input,
    });
}

export async function update(
    input: TaxUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateTaxResponse, TaxError>> {
    return apiRequest(`/api/taxes/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateTaxResponse,
        formErrorGuard: isTaxError,
        body: input,
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedTaxResolvedListResponse>> {
    const uri =
        query === null ? `/api/taxes/list` : `/api/taxes/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedTaxResolvedListResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<TaxResolvedResponse>> {
    return apiRequest(`/api/taxes/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isTaxResolvedResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<TaxResponse>> {
    return apiRequest(`/api/taxes/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isTaxResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteTaxResponse>> {
    return apiRequest(`/api/taxes/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteTaxResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/taxes/select_list?list=${list}`, {
        method: "GET",
        token,
        signal,
        guard: isSelectOptionListResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/taxes/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
