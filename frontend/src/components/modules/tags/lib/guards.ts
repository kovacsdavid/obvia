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
  CreateTagResponse,
  DeleteTagResponse,
  PaginatedTagResolvedListResponse,
  Tag,
  TagResolved,
  TagResolvedList,
  TagResolvedResponse,
  TagResponse,
  UpdateTagResponse
} from "@/components/modules/tags/lib/interface.ts";

export function isCreateTagResponse(data: unknown): data is CreateTagResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export function isTagResolvedResponse(data: unknown): data is TagResolvedResponse {
  return isCommonResponse(
    data,
    isTagResolved,
    isSimpleError
  )
}

export function isTagResolved(data: unknown): data is TagResolved {
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
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isTagResolvedList(data: unknown): data is TagResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isTagResolved(item))
  );
}

export function isPaginatedTagResolvedListResponse(data: unknown): data is PaginatedTagResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isTagResolvedList
  )
}

export function isTag(data: unknown): data is Tag {
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
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isTagResponse(data: unknown): data is TagResponse {
  return isCommonResponse(
    data,
    isTag,
    isSimpleError
  )
}

export function isUpdateTagResponse(data: unknown): data is UpdateTagResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteTagResponse(data: unknown): data is DeleteTagResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}