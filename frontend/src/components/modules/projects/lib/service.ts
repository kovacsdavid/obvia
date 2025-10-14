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
  type CreateProjectResponse,
  type DeleteProjectResponse,
  type PaginatedProjectResolvedListResponse,
  type ProjectResolvedResponse,
  type ProjectResponse,
  type ProjectUserInput,
  type UpdateProjectResponse
} from "@/components/modules/projects/lib/interface.ts";
import {type ProcessedResponse, ProcessResponse} from "@/lib/interfaces/common.ts";
import {
  isCreateProjectResponse,
  isDeleteProjectResponse,
  isPaginatedProjectResolvedListResponse,
  isProjectResolvedResponse,
  isProjectResponse,
  isUpdateProjectResponse
} from "@/components/modules/projects/lib/guards.ts";

export async function create({
                               id,
                               name,
                               description,
                               status,
                               startDate,
                               endDate
                             }: ProjectUserInput, token: string | null): Promise<ProcessedResponse<CreateProjectResponse>> {
  return await fetch(`/api/projects/create`, {
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
      status,
      start_date: startDate,
      end_date: endDate
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateProjectResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedProjectResolvedListResponse>> {
  const uri = query === null ? `/api/projects/list` : `/api/projects/list?q=${query}`;
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
      isPaginatedProjectResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<ProjectResolvedResponse>> {
  return await fetch(`/api/projects/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isProjectResolvedResponse,
    ) ?? unexpectedError;
  });
}

export async function update({
                               id,
                               name,
                               description,
                               status,
                               startDate,
                               endDate
                             }: ProjectUserInput, token: string | null): Promise<ProcessedResponse<UpdateProjectResponse>> {
  return await fetch(`/api/projects/update`, {
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
      status,
      start_date: startDate,
      end_date: endDate
    }),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isUpdateProjectResponse
    ) ?? unexpectedFormError;
  });
}

export async function get(uuid: string, token: string | null): Promise<ProcessedResponse<ProjectResponse>> {
  return await fetch(`/api/projects/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isProjectResponse
    ) ?? unexpectedError;
  });
}

export async function deleteItem(uuid: string, token: string | null): Promise<ProcessedResponse<DeleteProjectResponse>> {
  return await fetch(`/api/projects/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isDeleteProjectResponse
    ) ?? unexpectedError;
  });
}
