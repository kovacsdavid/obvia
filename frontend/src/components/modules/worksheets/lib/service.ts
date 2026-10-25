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
    type CreateWorksheetResponse,
    type DeleteWorksheetResponse,
    type PaginatedWorksheetResolvedListResponse,
    type UpdateWorksheetResponse,
    type WorksheetError,
    type WorksheetResolvedResponse,
    type WorksheetResponse,
    type WorksheetUserInput,
} from "@/components/modules/worksheets/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateWorksheetResponse,
    isDeleteWorksheetResponse,
    isPaginatedWorksheetResolvedListResponse,
    isUpdateWorksheetResponse,
    isWorksheetError,
    isWorksheetResolvedResponse,
    isWorksheetResponse,
} from "@/components/modules/worksheets/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    { id, description, customerId, projectId, status }: WorksheetUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateWorksheetResponse, WorksheetError>> {
    return apiRequest(`/api/worksheets/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateWorksheetResponse,
        formErrorGuard: isWorksheetError,
        body: {
            id,
            description,
            customer_id: customerId,
            project_id: projectId,
            status,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedWorksheetResolvedListResponse>> {
    const uri =
        query === null
            ? `/api/worksheets/list`
            : `/api/worksheets/list?q=${query}`;
    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedWorksheetResolvedListResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/worksheets/select_list?list=${list}`, {
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
): Promise<ApiResult<WorksheetResolvedResponse>> {
    return apiRequest(`/api/worksheets/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isWorksheetResolvedResponse,
    });
}

export async function update(
    { id, description, customerId, projectId, status }: WorksheetUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateWorksheetResponse, WorksheetError>> {
    return apiRequest(`/api/worksheets/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateWorksheetResponse,
        formErrorGuard: isWorksheetError,
        body: {
            id,
            description,
            customer_id: customerId,
            project_id: projectId,
            status,
        },
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<WorksheetResponse>> {
    return apiRequest(`/api/worksheets/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isWorksheetResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteWorksheetResponse>> {
    return apiRequest(`/api/worksheets/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteWorksheetResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/worksheets/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
