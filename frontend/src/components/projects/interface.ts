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

export interface CreateProject {
  name: string
  description: string
  status: string
  startDate: string
  endDate: string
}

export type CreateProjectResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateProjectResponse(data: unknown): data is CreateProjectResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export interface ProjectResolved {
  id: string,
  name: string,
  description: string | null,
  created_by_id: string,
  created_by: string,
  status: string,
  start_date: string | null,
  end_date: string | null,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type ProjectResolvedList = ProjectResolved[];

export function isProjectResolved(data: unknown): data is ProjectResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "description" in data &&
    (data.description === null || typeof data.description === "string") &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "status" in data &&
    typeof data.status === "string" &&
    "start_date" in data &&
    (data.start_date === null || typeof data.start_date === "string") &&
    "end_date" in data &&
    (data.end_date === null || typeof data.end_date === "string") &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isProjectResolvedList(data: unknown): data is ProjectResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isProjectResolved(item))
  );
}

export type PaginatedProjectResolvedListResponse = PaginatedDataResponse<ProjectResolvedList, SimpleError>;

export function isPaginatedProjectResolvedListResponse(
  data: unknown
): data is PaginatedProjectResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isProjectResolvedList,
  )
}
