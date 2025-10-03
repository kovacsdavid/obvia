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
  isSimpleMessageData,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateWorksheet {
  name: string
  description: string
  projectId: string
  status: string
}

export type CreateWorksheetResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateWorksheetResponse(data: unknown): data is CreateWorksheetResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export interface WorksheetResolved {
  id: string,
  name: string,
  description: string | null,
  project_id: string,
  project: string,
  created_by_id: string,
  created_by: string,
  status: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type WorksheetResolvedList = WorksheetResolved[];

export function isWorksheetResolved(data: unknown): data is WorksheetResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "description" in data &&
    (data.description === null || typeof data.description === "string") &&
    "project_id" in data &&
    typeof data.project_id === "string" &&
    "project" in data &&
    typeof data.project === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "status" in data &&
    typeof data.status === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isWorksheetResolvedList(data: unknown): data is WorksheetResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isWorksheetResolved(item))
  );
}

export type PaginatedWorksheetResolvedListResponse = PaginatedDataResponse<WorksheetResolvedList, SimpleError>;

export function isPaginatedWorksheetResolvedListResponse(data: unknown): data is PaginatedWorksheetResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isWorksheetResolvedList,
  )
}