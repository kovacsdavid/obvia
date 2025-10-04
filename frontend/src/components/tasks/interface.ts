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
  type CommonResponse,
  type FormError,
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  isSimpleMessageData,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateTask {
  worksheetId: string
  title: string
  description: string
  status: string
  priority: string
  dueDate: string
}

export type CreateTaskResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateTaskResponse(data: unknown): data is CreateTaskResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export interface TaskResolved {
  id: string,
  worksheet_id: string,
  worksheet: string,
  title: string,
  description: string | null,
  created_by_id: string,
  created_by: string,
  status: string,
  priority: string | null,
  due_date: string | null,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type TaskResolvedResponse = CommonResponse<TaskResolved, SimpleError>;

export function isTaskResolvedResponse(data: unknown): data is TaskResolvedResponse {
  return isCommonResponse(
    data,
    isTaskResolved,
    isSimpleError
  )
}

export type TaskResolvedList = TaskResolved[];

export function isTaskResolved(data: unknown): data is TaskResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "worksheet_id" in data &&
    typeof data.worksheet_id === "string" &&
    "worksheet" in data &&
    typeof data.worksheet === "string" &&
    "title" in data &&
    typeof data.title === "string" &&
    "description" in data &&
    (data.description === null || typeof data.description === "string") &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
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

export function isTaskResolvedList(data: unknown): data is TaskResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isTaskResolved(item))
  );
}

export type PaginatedTaskResolvedListResponse = PaginatedDataResponse<TaskResolvedList, SimpleError>;

export function isPaginatedTaskResolvedListResponse(data: unknown): data is PaginatedTaskResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isTaskResolvedList,
  )
}


