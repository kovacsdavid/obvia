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

import {isPaginatedDataResponse, type PaginatedDataResponse} from "@/lib/interfaces/common.ts";

export interface CreateTask {
  worksheetId: string
  title: string
  description: string
  status: string
  priority: string
  dueDate: string
}

export interface Task {
  id: string,
  worksheet_id: string,
  title: string,
  description: string | null,
  created_by: string,
  status: string,
  priority: string | null,
  due_date: string | null,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type TaskList = Task[];

export function isTask(data: unknown): data is Task {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "worksheet_id" in data &&
    typeof data.worksheet_id === "string" &&
    "title" in data &&
    typeof data.title === "string" &&
    "description" in data &&
    (data.description === null || typeof data.description === "string") &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "status" in data &&
    typeof data.status === "string" &&
    "priority" in data &&
    (data.priority === null || typeof data.priority === "string") &&
    "due_date" in data &&
    (data.due_date === null || typeof data.due_date === "string") &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isTaskList(data: unknown): data is TaskList {
  return (
    Array.isArray(data) &&
    data.every(item => isTask(item))
  );
}

export function isPaginatedTaskListResponse(data: unknown): data is PaginatedDataResponse<TaskList> {
  return isPaginatedDataResponse(
    data,
    isTaskList,
  )
}

