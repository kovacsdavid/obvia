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

import {globalRequestTimeout} from "@/services/utils/consts.ts";

export interface CreateWorksheet {
  name: string
  description: string
  projectId: string
  status: string
}

export async function create({
                         name,
                         description,
                         projectId,
                         status
                       }: CreateWorksheet, token: string | null): Promise<Response> {
  return await fetch(`/api/worksheets/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      name,
      description,
      project_id: projectId,
      status
    })
  });
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/worksheets/list` : `/api/worksheets/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export interface ProjectsSelectListItem {
  id: string,
  name: string,
  description: string | null,
  created_by: string,
  status: string,
  start_date: string | null,
  end_date: string | null,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export interface ProjectsSelectsListResponse {
  success: boolean,
  data: ProjectsSelectListItem[],
}

export function isProjectsSelectListItem(obj: any): obj is ProjectsSelectListItem {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.id === 'string' &&
    typeof obj.name === 'string' &&
    (obj.description === null || typeof obj.description === 'string') &&
    typeof obj.created_by === 'string' &&
    typeof obj.status === 'string' &&
    (obj.start_date === null || typeof obj.start_date === 'string') &&
    (obj.end_date === null || typeof obj.end_date === 'string') &&
    typeof obj.created_at === 'string' &&
    typeof obj.updated_at === 'string' &&
    (obj.deleted_at === null || typeof obj.deleted_at === 'string')
  );
}

export function isProjectsSelectsListResponse(obj: any): obj is ProjectsSelectsListResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.success === 'boolean' &&
    Array.isArray(obj.data) &&
    obj.data.every((item: any) => isProjectsSelectListItem(item))
  );
}

export async function select_list(list: string, token: string | null): Promise<Response> {
  return await fetch(`/api/worksheets/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}
