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
  type CreateProject,
  type CreateProjectResponse,
  isCreateProjectResponse,
  isPaginatedProjectResolvedListResponse,
  type PaginatedProjectResolvedListResponse
} from "@/components/projects/interface.ts";
import {type ProcessedResponse, ProcessResponse} from "@/lib/interfaces/common.ts";

export async function create({
                               name,
                               description,
                               status,
                               startDate,
                               endDate
                             }: CreateProject, token: string | null): Promise<ProcessedResponse<CreateProjectResponse>> {
  return await fetch(`/api/projects/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
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