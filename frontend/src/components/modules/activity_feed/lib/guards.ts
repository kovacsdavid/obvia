/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import type {
  ActivityFeedEntry,
  PaginatedActivityFeedResponse,
  PostCommentResponse,
} from "@/components/modules/activity_feed/lib/interface";
import {
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
} from "@/lib/interfaces/common";

export function isActivityFeedEntry(data: unknown): data is ActivityFeedEntry {
  return (
    typeof data === "object" &&
    data !== null &&
    "activity_type" in data &&
    typeof data.activity_type === "string" &&
    "description" in data &&
    typeof data.description === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string"
  );
}

export function isPostCommentResponse(
  data: unknown,
): data is PostCommentResponse {
  return isCommonResponse(data, isActivityFeedEntry, isFormError);
}

export function isActivityFeedArray(
  data: unknown,
): data is ActivityFeedEntry[] {
  return Array.isArray(data) && data.every((item) => isActivityFeedEntry(item));
}

export function isPaginatedActivityFeedResponse(
  data: unknown,
): data is PaginatedActivityFeedResponse {
  return isPaginatedDataResponse(data, isActivityFeedArray);
}
