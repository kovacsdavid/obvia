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

export interface CreateTag {
  name: string;
  description: string
}

export interface Tag {
  id: string,
  name: string,
  description: string | null,
  created_by_id: string,
  created_at: string,
  deleted_at: string | null,
}

export type TagList = Tag[];

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

export function isTagList(data: unknown): data is TagList {
  return (
    Array.isArray(data) &&
    data.every(item => isTag(item))
  );
}

export function isPaginatedTagListResponse(data: unknown): data is PaginatedDataResponse<TagList> {
  return isPaginatedDataResponse(
    data,
    isTagList
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

export function isPaginatedTagResolvedListResponse(data: unknown): data is PaginatedDataResponse<TagResolvedList> {
  return isPaginatedDataResponse(
    data,
    isTagResolvedList
  )
}