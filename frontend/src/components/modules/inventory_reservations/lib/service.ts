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
    type CreateInventoryReservationResponse,
    type DeleteInventoryReservationResponse,
    type InventoryReservationError,
    type InventoryReservationResolvedResponse,
    type InventoryReservationResponse,
    type InventoryReservationUserInput,
    type PaginatedInventoryReservationResolvedListResponse,
} from "@/components/modules/inventory_reservations/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateInventoryReservationResponse,
    isDeleteInventoryReservationResponse,
    isInventoryReservationError,
    isInventoryReservationResolvedResponse,
    isInventoryReservationResponse,
    isPaginatedInventoryReservationResolvedListResponse,
} from "@/components/modules/inventory_reservations/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    input: InventoryReservationUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<
    ApiResult<CreateInventoryReservationResponse, InventoryReservationError>
> {
    return apiRequest("/api/inventory_reservations/create", {
        method: "POST",
        token,
        signal,
        guard: isCreateInventoryReservationResponse,
        formErrorGuard: isInventoryReservationError,
        body: input,
    });
}

export async function list(
    params: { inventoryId: string; query: string | null },
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedInventoryReservationResolvedListResponse>> {
    const uri =
        params.query === null
            ? `/api/inventory_reservations/list?inventory_id=${params.inventoryId}`
            : `/api/inventory_reservations/list?inventory_id=${params.inventoryId}&q=${params.query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedInventoryReservationResolvedListResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<InventoryReservationResponse>> {
    return apiRequest(`/api/inventory_reservations/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isInventoryReservationResponse,
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<InventoryReservationResolvedResponse>> {
    return apiRequest(`/api/inventory_reservations/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isInventoryReservationResolvedResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteInventoryReservationResponse>> {
    return apiRequest(`/api/inventory_reservations/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteInventoryReservationResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/inventory_reservations/select_list?list=${list}`, {
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
    return apiRequest(`/api/inventory_reservations/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
