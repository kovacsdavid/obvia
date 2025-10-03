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

export interface CreateTag {
  name: string;
  description: string
}

export type CreateTagResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateTagResponse(data: unknown): data is CreateTagResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export interface TagResolved {
  id: string,
  name: string,
  description: string | null,
  created_by_id: string,
  created_by: string,
  created_at: string,
  deleted_at: string | null,
}

export type TagResolvedList = TagResolved[];

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

export type PaginatedTagResolvedListResponse = PaginatedDataResponse<TagResolvedList, SimpleError>;

export function isPaginatedTagResolvedListResponse(data: unknown): data is PaginatedTagResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isTagResolvedList
  )
}