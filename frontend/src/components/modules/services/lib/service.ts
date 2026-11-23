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
    type CreateServiceResponse,
    type DeleteServiceResponse,
    type PaginatedServiceResolvedListResponse,
    type ServiceError,
    type ServiceResolvedResponse,
    type ServiceResponse,
    type ServiceUserInput,
    type UpdateServiceResponse,
} from "@/components/modules/services/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateServiceResponse,
    isDeleteServiceResponse,
    isPaginatedServiceResolvedListResponse,
    isServiceError,
    isServiceResolvedResponse,
    isServiceResponse,
    isUpdateServiceResponse,
} from "@/components/modules/services/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    {
        id,
        name,
        description,
        defaultPrice,
        defaultTaxId,
        currencyCode,
        status,
        unitOfMeasureId,
        newUnitOfMeasure,
    }: ServiceUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateServiceResponse, ServiceError>> {
    return apiRequest(`/api/services/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateServiceResponse,
        formErrorGuard: isServiceError,
        body: {
            id,
            name,
            description,
            default_price: defaultPrice,
            default_tax_id: defaultTaxId,
            currency_code: currencyCode,
            status,
            unit_of_measure_id: unitOfMeasureId,
            new_unit_of_measure: newUnitOfMeasure,
        },
    });
}

export async function update(
    {
        id,
        name,
        description,
        defaultPrice,
        defaultTaxId,
        currencyCode,
        status,
        unitOfMeasureId,
        newUnitOfMeasure,
    }: ServiceUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateServiceResponse, ServiceError>> {
    return apiRequest(`/api/services/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateServiceResponse,
        formErrorGuard: isServiceError,
        body: {
            id,
            name,
            description,
            default_price: defaultPrice,
            default_tax_id: defaultTaxId,
            currency_code: currencyCode,
            status,
            unit_of_measure_id: unitOfMeasureId,
            new_unit_of_measure: newUnitOfMeasure,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedServiceResolvedListResponse>> {
    const uri =
        query === null ? `/api/services/list` : `/api/services/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedServiceResolvedListResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ServiceResolvedResponse>> {
    return apiRequest(`/api/services/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isServiceResolvedResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<ServiceResponse>> {
    return apiRequest(`/api/services/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isServiceResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteServiceResponse>> {
    return apiRequest(`/api/services/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteServiceResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/services/select_list?list=${list}`, {
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
    return apiRequest(`/api/services/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
