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
  CreateProjectResponse,
  DeleteProjectResponse,
  PaginatedProjectResolvedListResponse,
  Project,
  ProjectResolved,
  ProjectResolvedList,
  ProjectResolvedResponse,
  ProjectResponse,
  UpdateProjectResponse
} from "@/components/modules/projects/lib/interface.ts";

export function isCreateProjectResponse(data: unknown): data is CreateProjectResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isProjectResolvedResponse(data: unknown): data is ProjectResolvedResponse {
  return isCommonResponse(
    data,
    isProjectResolved,
    isSimpleError,
  )
}

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

export function isPaginatedProjectResolvedListResponse(
  data: unknown
): data is PaginatedProjectResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isProjectResolvedList,
  )
}

export function isProject(data: unknown): data is Project {
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

export function isProjectResponse(data: unknown): data is ProjectResponse {
  return isCommonResponse(
    data,
    isProject,
    isSimpleError
  )
}

export function isUpdateProjectResponse(data: unknown): data is UpdateProjectResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteProjectResponse(data: unknown): data is DeleteProjectResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}