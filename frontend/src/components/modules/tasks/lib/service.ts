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
    type CreateTaskResponse,
    type DeleteTaskResponse,
    type PaginatedTaskResolvedListResponse,
    type TaskErrors,
    type TaskResolvedResponse,
    type TaskResponse,
    type TaskUserInput,
    type UpdateTaskResponse,
} from "@/components/modules/tasks/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateTaskResponse,
    isDeleteTaskResponse,
    isPaginatedTaskResolvedListResponse,
    isTaskErrors,
    isTaskResolvedResponse,
    isTaskResponse,
    isUpdateTaskResponse,
} from "@/components/modules/tasks/lib/guards.ts";
import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";

export async function create(
    {
        id,
        worksheetId,
        serviceId,
        currencyCode,
        quantity,
        price,
        taxId,
        status,
        priority,
        dueDate,
        description,
    }: TaskUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<CreateTaskResponse, TaskErrors>> {
    return apiRequest(`/api/tasks/create`, {
        method: "POST",
        token,
        signal,
        guard: isCreateTaskResponse,
        formErrorGuard: isTaskErrors,
        body: {
            id,
            worksheet_id: worksheetId,
            service_id: serviceId,
            currency_code: currencyCode,
            quantity,
            price,
            tax_id: taxId,
            status,
            priority,
            due_date: dueDate,
            description,
        },
    });
}

export async function list(
    query: string | null,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedTaskResolvedListResponse>> {
    const uri =
        query === null ? `/api/tasks/list` : `/api/tasks/list?q=${query}`;

    return apiRequest(uri, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedTaskResolvedListResponse,
    });
}

export async function select_list(
    list: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<SelectOptionListResponse>> {
    return apiRequest(`/api/tasks/select_list?list=${list}`, {
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
): Promise<ApiResult<TaskResolvedResponse>> {
    return apiRequest(`/api/tasks/get_resolved?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isTaskResolvedResponse,
    });
}

export async function update(
    {
        id,
        worksheetId,
        serviceId,
        currencyCode,
        quantity,
        price,
        taxId,
        status,
        priority,
        dueDate,
        description,
    }: TaskUserInput,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<UpdateTaskResponse, TaskErrors>> {
    return apiRequest(`/api/tasks/update`, {
        method: "PUT",
        token,
        signal,
        guard: isUpdateTaskResponse,
        formErrorGuard: isTaskErrors,
        body: {
            id,
            worksheet_id: worksheetId,
            service_id: serviceId,
            currency_code: currencyCode,
            quantity,
            price,
            tax_id: taxId,
            status,
            priority,
            due_date: dueDate,
            description,
        },
    });
}

export async function get(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<TaskResponse>> {
    return apiRequest(`/api/tasks/get?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: isTaskResponse,
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteTaskResponse>> {
    return apiRequest(`/api/tasks/delete?uuid=${uuid}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteTaskResponse,
    });
}

export async function print(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<Uint8Array<ArrayBuffer> | null>> {
    return apiRequest(`/api/tasks/print?uuid=${uuid}`, {
        method: "GET",
        token,
        signal,
        guard: (data): data is Uint8Array<ArrayBuffer> =>
            data instanceof Uint8Array,
    });
}
