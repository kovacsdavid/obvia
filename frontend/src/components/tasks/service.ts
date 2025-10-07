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

import {globalRequestTimeout, unexpectedError, unexpectedFormError} from "@/services/utils/consts.ts";
import {
  type TaskUserInput,
  type CreateTaskResponse,
  type PaginatedTaskResolvedListResponse,
  type TaskResolvedResponse, type UpdateTaskResponse, type TaskResponse, type DeleteTaskResponse
} from "@/components/tasks/interface.ts";
import {
  isSelectOptionListResponse,
  type ProcessedResponse,
  ProcessResponse,
  type SelectOptionListResponse
} from "@/lib/interfaces/common.ts";
import {
  isCreateTaskResponse, isDeleteTaskResponse,
  isPaginatedTaskResolvedListResponse,
  isTaskResolvedResponse, isTaskResponse, isUpdateTaskResponse
} from "@/components/tasks/guards.ts";

export async function create({
                               id,
                               worksheetId,
                               title,
                               description,
                               status,
                               priority,
                               dueDate
                             }: TaskUserInput, token: string | null): Promise<ProcessedResponse<CreateTaskResponse>> {
  return await fetch(`/api/tasks/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      worksheet_id: worksheetId,
      title,
      description,
      status,
      priority,
      due_date: dueDate
    })
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateTaskResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedTaskResolvedListResponse>> {
  const uri = query === null ? `/api/tasks/list` : `/api/tasks/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isPaginatedTaskResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function select_list(list: string, token: string | null): Promise<ProcessedResponse<SelectOptionListResponse>> {
  return await fetch(`/api/tasks/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isSelectOptionListResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<TaskResolvedResponse>> {
  return await fetch(`/api/tasks/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isTaskResolvedResponse,
    ) ?? unexpectedError;
  });
}

export async function update({
                               id,
                               worksheetId,
                               title,
                               description,
                               status,
                               priority,
                               dueDate
                             }: TaskUserInput, token: string | null): Promise<ProcessedResponse<UpdateTaskResponse>> {
  return await fetch(`/api/tasks/update`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      worksheet_id: worksheetId,
      title,
      description,
      status,
      priority,
      due_date: dueDate
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isUpdateTaskResponse
    ) ?? unexpectedFormError;
  });
}

export async function get(uuid: string, token: string | null): Promise<ProcessedResponse<TaskResponse>> {
  return await fetch(`/api/tasks/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isTaskResponse
    ) ?? unexpectedError;
  });
}

export async function deleteTask(uuid: string, token: string | null): Promise<ProcessedResponse<DeleteTaskResponse>> {
  return await fetch(`/api/tasks/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isDeleteTaskResponse
    ) ?? unexpectedError;
  });
}
