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

import {
  type ProcessedResponse,
  ProcessResponse,
} from "@/lib/interfaces/common.ts";
import {
  globalRequestTimeout,
  unexpectedError,
  unexpectedFormError,
} from "@/services/utils/consts.ts";
import type {
  PaginatedActivityFeedResponse,
  PostCommentResponse,
} from "@/components/modules/activity_feed/lib/interface";
import {
  isPaginatedActivityFeedResponse,
  isPostCommentResponse,
} from "./guards";

export async function list(
  resourceId: string,
  resourceType: string,
  token: string | null,
): Promise<ProcessedResponse<PaginatedActivityFeedResponse>> {
  const uri = `/api/activity_feed/list?resource_id=${resourceId}&resource_type=${resourceType}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isPaginatedActivityFeedResponse)) ??
      unexpectedError
    );
  });
}

export async function post_comment(
  resourceId: string,
  resourceType: string,
  comment: string,
  token: string | null,
): Promise<ProcessedResponse<PostCommentResponse>> {
  return await fetch(`/api/comments/post`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      commentable_id: resourceId,
      commentable_type: resourceType,
      comment,
    }),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isPostCommentResponse)) ??
      unexpectedFormError
    );
  });
}
