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
    type CreateWarehouseResponse,
    type DeleteWarehouseResponse,
    type PaginatedWarehouseResolvedListResponse,
    type UpdateWarehouseResponse,
    type WarehouseError,
    type WarehouseResolvedResponse,
    type WarehouseResponse,
    type WarehouseUserInput,
} from "@/components/modules/warehouses/lib/interface.ts";
import {
    isCreateWarehouseResponse,
    isDeleteWarehouseResponse,
    isPaginatedWarehouseResolvedListResponse,
    isUpdateWarehouseResponse,
    isWarehouseError,
    isWarehouseResolvedResponse,
    isWarehouseResponse,
} from "@/components/modules/warehouses/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    { id, name, contactName, contactPhone, status }: WarehouseUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateWarehouseResponse, WarehouseError>> {
    return apiRequest(`/api/warehouses/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateWarehouseResponse,
        formErrorGuard: isWarehouseError,
        body: {
            id,
            name,
            contact_name: contactName,
            contact_phone: contactPhone,
            status,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedWarehouseResolvedListResponse>> {
    const uri =
        query === null
            ? `/api/warehouses/list`
            : `/api/warehouses/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedWarehouseResolvedListResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<WarehouseResolvedResponse>> {
    return apiRequest(`/api/warehouses/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isWarehouseResolvedResponse,
    });
}

export async function update(
    { id, name, contactName, contactPhone, status }: WarehouseUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateWarehouseResponse, WarehouseError>> {
    return apiRequest(`/api/warehouses/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateWarehouseResponse,
        formErrorGuard: isWarehouseError,
        body: {
            id,
            name,
            contact_name: contactName,
            contact_phone: contactPhone,
            status,
        },
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<WarehouseResponse>> {
    return apiRequest(`/api/warehouses/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isWarehouseResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteWarehouseResponse>> {
    return apiRequest(`/api/warehouses/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteWarehouseResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/warehouses/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
