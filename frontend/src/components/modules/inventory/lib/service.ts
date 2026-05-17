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
    globalRequestTimeout,
    unexpectedError,
    unexpectedFormError,
} from "@/services/utils/consts.ts";
import {
    type CreateInventoryResponse,
    type DeleteInventoryResponse,
    type InventoryResolvedResponse,
    type InventoryResponse,
    type InventoryUserInput,
    type PaginatedInventoryResolvedListResponse,
    type UpdateInventoryResponse,
} from "@/components/modules/inventory/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type ProcessedJsonResponse,
    ProcessJsonResponse,
    type SelectOptionListResponse,
    type ProcessedBytesResponse,
    ProcessBytesResponse,
} from "@/lib/interface.ts";
import {
    isCreateInventoryResponse,
    isDeleteInventoryResponse,
    isInventoryResolvedResponse,
    isInventoryResponse,
    isPaginatedInventoryResolvedListResponse,
    isUpdateInventoryResponse,
} from "@/components/modules/inventory/lib/guards.ts";

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
): Promise<ProcessedJsonResponse<CreateInventoryResponse>> {
    return await fetch(`/api/inventory/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            product_id: productId,
            warehouse_id: warehouseId,
            minimum_stock: minimumStock,
            maximum_stock: maximumStock,
            currency_code: currencyCode,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isCreateInventoryResponse)) ??
            unexpectedFormError
        );
    });
}

export async function list(
    query: string | null,
    token: string | null,
): Promise<ProcessedJsonResponse<PaginatedInventoryResolvedListResponse>> {
    const uri =
        query === null
            ? `/api/inventory/list`
            : `/api/inventory/list?q=${query}`;
    return await fetch(uri, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isPaginatedInventoryResolvedListResponse,
            )) ?? unexpectedError
        );
    });
}

export async function select_list(
    list: string,
    token: string | null,
): Promise<ProcessedJsonResponse<SelectOptionListResponse>> {
    return await fetch(`/api/inventory/select_list?list=${list}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isSelectOptionListResponse)) ??
            unexpectedError
        );
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<InventoryResolvedResponse>> {
    return await fetch(`/api/inventory/get_resolved?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isInventoryResolvedResponse,
            )) ?? unexpectedError
        );
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
): Promise<ProcessedJsonResponse<UpdateInventoryResponse>> {
    return await fetch(`/api/inventory/update`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            product_id: productId,
            warehouse_id: warehouseId,
            minimum_stock: minimumStock,
            maximum_stock: maximumStock,
            currency_code: currencyCode,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isUpdateInventoryResponse)) ??
            unexpectedFormError
        );
    });
}

export async function get(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<InventoryResponse>> {
    return await fetch(`/api/inventory/get?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isInventoryResponse)) ??
            unexpectedError
        );
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<DeleteInventoryResponse>> {
    return await fetch(`/api/inventory/delete?uuid=${uuid}`, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isDeleteInventoryResponse)) ??
            unexpectedError
        );
    });
}

export async function print(
    uuid: string,
    token: string | null,
): Promise<ProcessedBytesResponse> {
    return await fetch(`/api/inventory/print?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return ProcessBytesResponse(response);
    });
}
