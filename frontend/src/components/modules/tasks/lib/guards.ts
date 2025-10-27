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
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  isSimpleMessageData
} from "@/lib/interfaces/common.ts";
import type {
  CreateTaskResponse,
  DeleteTaskResponse,
  PaginatedTaskResolvedListResponse,
  Task,
  TaskResolved,
  TaskResolvedList,
  TaskResolvedResponse,
  TaskResponse,
  UpdateTaskResponse
} from "@/components/modules/tasks/lib/interface.ts";

export function isCreateTaskResponse(data: unknown): data is CreateTaskResponse {
  return isCommonResponse(
    data,
    isTask,
    isFormError
  )
}

export function isTaskResolvedResponse(data: unknown): data is TaskResolvedResponse {
  return isCommonResponse(
    data,
    isTaskResolved,
    isSimpleError
  )
}

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
    "service_id" in data &&
    typeof data.service_id === "string" &&
    "service" in data &&
    typeof data.service === "string" &&
    "currency_code" in data &&
    typeof data.currency_code === "string" &&
    "quantity" in data &&
    (data.quantity === null || typeof data.quantity === "string") &&
    "price" in data &&
    (data.price === null || typeof data.price === "string") &&
    "tax_id" in data &&
    typeof data.tax_id === "string" &&
    "tax" in data &&
    typeof data.tax === "string" &&
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
    (data.deleted_at === null || typeof data.deleted_at === "string") &&
    "description" in data &&
    (data.description === null || typeof data.description === "string")
  );
}

export function isTaskResolvedList(data: unknown): data is TaskResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isTaskResolved(item))
  );
}

export function isPaginatedTaskResolvedListResponse(data: unknown): data is PaginatedTaskResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isTaskResolvedList,
  )
}

export function isTask(data: unknown): data is Task {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "worksheet_id" in data &&
    typeof data.worksheet_id === "string" &&
    "service_id" in data &&
    typeof data.service_id === "string" &&
    "currency_code" in data &&
    typeof data.currency_code === "string" &&
    "quantity" in data &&
    (data.quantity === null || typeof data.quantity === "string") &&
    "price" in data &&
    (data.price === null || typeof data.price === "string") &&
    "tax_id" in data &&
    typeof data.tax_id === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
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
    (data.deleted_at === null || typeof data.deleted_at === "string") &&
    "description" in data &&
    (data.description === null || typeof data.description === "string")
  );
}

export function isTaskResponse(data: unknown): data is TaskResponse {
  return isCommonResponse(
    data,
    isTask,
    isSimpleError
  )
}

export function isUpdateTaskResponse(data: unknown): data is UpdateTaskResponse {
  return isCommonResponse(
    data,
    isTask,
    isFormError
  )
}

export function isDeleteTaskResponse(data: unknown): data is DeleteTaskResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}
