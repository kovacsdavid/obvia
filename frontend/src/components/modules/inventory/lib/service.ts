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
    type CreateInventoryResponse,
    type DeleteInventoryResponse,
    type InventoryResolvedResponse,
    type InventoryResponse,
    type InventoryError,
    type InventoryUserInput,
    type PaginatedInventoryResolvedListResponse,
    type UpdateInventoryResponse,
} from "@/components/modules/inventory/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateInventoryResponse,
    isDeleteInventoryResponse,
    isInventoryError,
    isInventoryResolvedResponse,
    isInventoryResponse,
    isPaginatedInventoryResolvedListResponse,
    isUpdateInventoryResponse,
} from "@/components/modules/inventory/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    {
        id,
        productId,
        warehouseId,
        minimumStock,
        maximumStock,
        currencyCode,
        status,
    }: InventoryUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateInventoryResponse, InventoryError>> {
    return apiRequest(`/api/inventory/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateInventoryResponse,
        formErrorGuard: isInventoryError,
        body: {
            id,
            product_id: productId,
            warehouse_id: warehouseId,
            minimum_stock: minimumStock,
            maximum_stock: maximumStock,
            currency_code: currencyCode,
            status,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedInventoryResolvedListResponse>> {
    const uri =
        query === null
            ? `/api/inventory/list`
            : `/api/inventory/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedInventoryResolvedListResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/inventory/select_list?list=${list}`, {
        method: "GET",
        token,
        signal,
        guard: isSelectOptionListResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<InventoryResolvedResponse>> {
    return apiRequest(`/api/inventory/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isInventoryResolvedResponse,
    });
}

export async function update(
    {
        id,
        productId,
        warehouseId,
        minimumStock,
        maximumStock,
        currencyCode,
        status,
    }: InventoryUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateInventoryResponse, InventoryError>> {
    return apiRequest(`/api/inventory/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateInventoryResponse,
        formErrorGuard: isInventoryError,
        body: {
            id,
            product_id: productId,
            warehouse_id: warehouseId,
            minimum_stock: minimumStock,
            maximum_stock: maximumStock,
            currency_code: currencyCode,
            status,
        },
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<InventoryResponse>> {
    return apiRequest(`/api/inventory/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isInventoryResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteInventoryResponse>> {
    return apiRequest(`/api/inventory/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteInventoryResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/inventory/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
