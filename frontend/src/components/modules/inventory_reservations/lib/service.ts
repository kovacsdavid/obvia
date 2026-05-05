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
    type CreateInventoryReservationResponse,
    type DeleteInventoryReservationResponse,
    type InventoryReservationResolvedResponse,
    type InventoryReservationResponse,
    type InventoryReservationUserInput,
    type PaginatedInventoryReservationResolvedListResponse,
} from "@/components/modules/inventory_reservations/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type ProcessedJsonResponse,
    ProcessJsonResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateInventoryReservationResponse,
    isDeleteInventoryReservationResponse,
    isInventoryReservationResolvedResponse,
    isInventoryReservationResponse,
    isPaginatedInventoryReservationResolvedListResponse,
} from "@/components/modules/inventory_reservations/lib/guards.ts";

export async function create(
    input: InventoryReservationUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<CreateInventoryReservationResponse>> {
    return await fetch(`/api/inventory_reservations/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id: input.id,
            inventory_id: input.inventoryId,
            quantity: input.quantity,
            reference_type: input.referenceType,
            reference_id: input.referenceId,
            reserved_until: input.reservedUntil,
            status: input.status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isCreateInventoryReservationResponse,
            )) ?? unexpectedFormError
        );
    });
}

export async function list(
    params: {
        inventoryId: string;
        query: string | null;
    },
    token: string | null,
): Promise<
    ProcessedJsonResponse<PaginatedInventoryReservationResolvedListResponse>
> {
    const uri =
        params.query === null
            ? `/api/inventory_reservations/list?inventory_id=${params.inventoryId}`
            : `/api/inventory_reservations/list?inventory_id=${params.inventoryId}&q=${params.query}`;
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
                isPaginatedInventoryReservationResolvedListResponse,
            )) ?? unexpectedError
        );
    });
}

export async function get(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<InventoryReservationResponse>> {
    return await fetch(`/api/inventory_reservations/get?uuid=${uuid}`, {
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
                isInventoryReservationResponse,
            )) ?? unexpectedError
        );
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<InventoryReservationResolvedResponse>> {
    return await fetch(
        `/api/inventory_reservations/get_resolved?uuid=${uuid}`,
        {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
                ...(token ? { Authorization: `Bearer ${token}` } : {}),
            },
            signal: AbortSignal.timeout(globalRequestTimeout),
        },
    ).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isInventoryReservationResolvedResponse,
            )) ?? unexpectedError
        );
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<DeleteInventoryReservationResponse>> {
    return await fetch(`/api/inventory_reservations/delete?uuid=${uuid}`, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isDeleteInventoryReservationResponse,
            )) ?? unexpectedError
        );
    });
}

export async function select_list(
    list: string,
    token: string | null,
): Promise<ProcessedJsonResponse<SelectOptionListResponse>> {
    return await fetch(`/api/inventory_reservations/select_list?list=${list}`, {
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
