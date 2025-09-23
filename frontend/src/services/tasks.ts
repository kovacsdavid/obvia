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

export interface CreateTask {
  worksheetId: string
  title: string
  description: string
  status: string
  priority: string
  dueDate: string
}

export async function create({
                               worksheetId,
                               title,
                               description,
                               status,
                               priority,
                               dueDate
                             }: CreateTask, token: string | null): Promise<Response>
{
  return await fetch(`/api/tasks/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      worksheet_id: worksheetId,
      title,
      description,
      status,
      priority,
      due_date: dueDate
    })
  });
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/tasks/list` : `/api/tasks/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export interface WorksheetsSelectListItem {
  id: string,
  name: string,
  description: string | null,
  project_id: string,
  created_by: string,
  status: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null
}

export interface WorksheetsSelectListResponse {
  success: boolean,
  data: WorksheetsSelectListItem[],
}


export function isWorksheetsSelectListItem(obj: unknown): obj is WorksheetsSelectListItem {
  if (typeof obj !== 'object' || obj === null) {
    return false;
  }

  const item = obj as Record<string, unknown>;

  return (
    typeof item.id === 'string' &&
    typeof item.name === 'string' &&
    (typeof item.description === 'string' || item.description === null) &&
    typeof item.project_id === 'string' &&
    typeof item.created_by === 'string' &&
    typeof item.status === 'string' &&
    typeof item.created_at === 'string' &&
    typeof item.updated_at === 'string' &&
    (typeof item.deleted_at === 'string' || item.deleted_at === null)
  );
}

export function isWorksheetsSelectListResponse(obj: unknown): obj is WorksheetsSelectListResponse {
  if (typeof obj !== 'object' || obj === null) {
    return false;
  }

  const response = obj as Record<string, unknown>;

  return (
    typeof response.success === 'boolean' &&
    Array.isArray(response.data) &&
    response.data.every(item => isWorksheetsSelectListItem(item))
  );
}

export async function select_list(list: string, token: string | null): Promise<Response> {
  return await fetch(`/api/tasks/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}
