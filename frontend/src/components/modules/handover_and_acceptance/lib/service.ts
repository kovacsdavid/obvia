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
    CreateHandoverAndAcceptanceResponse,
    DeleteHandoverAndAcceptanceResponse,
    HandoverAndAcceptanceError,
    HandoverAndAcceptanceFullResponse,
    HandoverAndAcceptanceResponse,
    HandoverAndAcceptanceUserInput,
    PaginatedHandoverAndAcceptanceFullListResponse,
    SelectOptionListResponse,
    UpdateHandoverAndAcceptanceResponse,
} from "@/components/modules/handover_and_acceptance/lib/interface.ts";
import {
    isCreateHandoverAndAcceptanceResponse,
    isDeleteHandoverAndAcceptanceResponse,
    isHandoverAndAcceptanceError,
    isHandoverAndAcceptanceFullResponse,
    isHandoverAndAcceptanceResponse,
    isPaginatedHandoverAndAcceptanceFullListResponse,
    isSelectOptionListResponse,
    isUpdateHandoverAndAcceptanceResponse,
} from "@/components/modules/handover_and_acceptance/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    {
        id,
        customerId,
        itemMakeId,
        newItemMake,
        itemModelId,
        newItemModel,
        itemIdentifierTypeId,
        newItemIdentifierType,
        itemIdentifierValue,
        reportedIssue,
        conditionAtReceipt,
        includedAccessories,
        authorizedWork,
        repairLimit,
        currencyCode,
        termsVersion,
        receivedAt,
    }: HandoverAndAcceptanceUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<
    ApiResult<CreateHandoverAndAcceptanceResponse, HandoverAndAcceptanceError>
> {
    return apiRequest(`/api/handover_and_acceptance/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateHandoverAndAcceptanceResponse,
        formErrorGuard: isHandoverAndAcceptanceError,
        body: {
            id,
            customer_id: customerId,
            item_make_id: itemMakeId,
            new_item_make: newItemMake,
            item_model_id: itemModelId,
            new_item_model: newItemModel,
            item_identifier_type_id: itemIdentifierTypeId,
            new_item_identifier_type: newItemIdentifierType,
            item_identifier_value: itemIdentifierValue,
            reported_issue: reportedIssue,
            condition_at_receipt: conditionAtReceipt,
            included_accessories: includedAccessories,
            authorized_work: authorizedWork,
            repair_limit: repairLimit,
            currency_code: currencyCode,
            terms_version: termsVersion,
            received_at: receivedAt,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedHandoverAndAcceptanceFullListResponse>> {
    const uri =
        query === null
            ? `/api/handover_and_acceptance/list`
            : `/api/handover_and_acceptance/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedHandoverAndAcceptanceFullListResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
    param?: string,
): Promise<ApiResult<SelectOptionListResponse>> {
    const query = new URLSearchParams({ list });
    if (param !== undefined) query.set("param", param);
    return apiRequest(
        `/api/handover_and_acceptance/select_list?${query.toString()}`,
        {
            method: "GET",
            token,
            signal,
            guard: isSelectOptionListResponse,
        },
    );
}

export async function get_full(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<HandoverAndAcceptanceFullResponse>> {
    return apiRequest(`/api/handover_and_acceptance/get_full?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isHandoverAndAcceptanceFullResponse,
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<HandoverAndAcceptanceResponse>> {
    return apiRequest(`/api/handover_and_acceptance/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isHandoverAndAcceptanceResponse,
    });
}

export async function update(
    {
        id,
        customerId,
        itemMakeId,
        newItemMake,
        itemModelId,
        newItemModel,
        itemIdentifierTypeId,
        newItemIdentifierType,
        itemIdentifierValue,
        reportedIssue,
        conditionAtReceipt,
        includedAccessories,
        authorizedWork,
        repairLimit,
        currencyCode,
        termsVersion,
        receivedAt,
    }: HandoverAndAcceptanceUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<
    ApiResult<UpdateHandoverAndAcceptanceResponse, HandoverAndAcceptanceError>
> {
    return apiRequest(`/api/handover_and_acceptance/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateHandoverAndAcceptanceResponse,
        formErrorGuard: isHandoverAndAcceptanceError,
        body: {
            id,
            customer_id: customerId,
            item_make_id: itemMakeId,
            new_item_make: newItemMake,
            item_model_id: itemModelId,
            new_item_model: newItemModel,
            item_identifier_type_id: itemIdentifierTypeId,
            new_item_identifier_type: newItemIdentifierType,
            item_identifier_value: itemIdentifierValue,
            reported_issue: reportedIssue,
            condition_at_receipt: conditionAtReceipt,
            included_accessories: includedAccessories,
            authorized_work: authorizedWork,
            repair_limit: repairLimit,
            currency_code: currencyCode,
            terms_version: termsVersion,
            received_at: receivedAt,
        },
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteHandoverAndAcceptanceResponse>> {
    return apiRequest(`/api/handover_and_acceptance/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteHandoverAndAcceptanceResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/handover_and_acceptance/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
