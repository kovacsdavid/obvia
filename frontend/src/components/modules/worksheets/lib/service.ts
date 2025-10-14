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
  type CreateWorksheetResponse,
  type DeleteWorksheetResponse,
  type PaginatedWorksheetResolvedListResponse,
  type UpdateWorksheetResponse,
  type WorksheetResolvedResponse,
  type WorksheetResponse,
  type WorksheetUserInput,
} from "@/components/modules/worksheets/lib/interface.ts";
import {
  isSelectOptionListResponse,
  type ProcessedResponse,
  ProcessResponse,
  type SelectOptionListResponse
} from "@/lib/interfaces/common.ts";
import {
  isCreateWorksheetResponse,
  isDeleteWorksheetResponse,
  isPaginatedWorksheetResolvedListResponse,
  isUpdateWorksheetResponse,
  isWorksheetResolvedResponse,
  isWorksheetResponse
} from "@/components/modules/worksheets/lib/guards.ts";

export async function create({
                               id,
                               name,
                               description,
                               projectId,
                               status
                             }: WorksheetUserInput, token: string | null): Promise<ProcessedResponse<CreateWorksheetResponse>> {
  return await fetch(`/api/worksheets/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      description,
      project_id: projectId,
      status
    })
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateWorksheetResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedWorksheetResolvedListResponse>> {
  const uri = query === null ? `/api/worksheets/list` : `/api/worksheets/list?q=${query}`;
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
      isPaginatedWorksheetResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function select_list(list: string, token: string | null): Promise<ProcessedResponse<SelectOptionListResponse>> {
  return await fetch(`/api/worksheets/select_list?list=${list}`, {
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

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<WorksheetResolvedResponse>> {
  return await fetch(`/api/worksheets/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isWorksheetResolvedResponse,
    ) ?? unexpectedError;
  });
}

export async function update({
                               id,
                               name,
                               description,
                               projectId,
                               status
                             }: WorksheetUserInput, token: string | null): Promise<ProcessedResponse<UpdateWorksheetResponse>> {
  return await fetch(`/api/worksheets/update`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      description,
      project_id: projectId,
      status
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isUpdateWorksheetResponse
    ) ?? unexpectedFormError;
  });
}

export async function get(uuid: string, token: string | null): Promise<ProcessedResponse<WorksheetResponse>> {
  return await fetch(`/api/worksheets/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isWorksheetResponse
    ) ?? unexpectedError;
  });
}

export async function deleteItem(uuid: string, token: string | null): Promise<ProcessedResponse<DeleteWorksheetResponse>> {
  return await fetch(`/api/worksheets/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isDeleteWorksheetResponse
    ) ?? unexpectedError;
  });
}