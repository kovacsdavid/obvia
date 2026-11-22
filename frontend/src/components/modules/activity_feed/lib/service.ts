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

import { apiRequest } from "@/lib/api/request";
import type { ApiResult } from "@/lib/api/types";
import type {
    CommentError,
    DeleteCommentResponse,
    PaginatedActivityFeedResponse,
    PostCommentResponse,
} from "@/components/modules/activity_feed/lib/interface";
import {
    isPaginatedActivityFeedResponse,
    isPostCommentResponse,
    isDeleteCommentResponse,
    isCommentError,
} from "./guards";

export async function list(
    resourceId: string,
    resourceType: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PaginatedActivityFeedResponse>> {
    const query = new URLSearchParams({
        resource_id: resourceId,
        resource_type: resourceType,
    });
    return apiRequest(`/api/activity_feed/list?${query}`, {
        method: "GET",
        token,
        signal,
        guard: isPaginatedActivityFeedResponse,
    });
}

export async function post_comment(
    resourceId: string,
    resourceType: string,
    comment: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<PostCommentResponse, CommentError>> {
    return apiRequest(`/api/comments/post`, {
        method: "POST",
        token,
        signal,
        guard: isPostCommentResponse,
        formErrorGuard: isCommentError,
        body: {
            commentable_id: resourceId,
            commentable_type: resourceType,
            comment,
        },
    });
}

export async function delete_comment(
    uuid: string,
    token: string | null,
    signal?: AbortSignal,
): Promise<ApiResult<DeleteCommentResponse>> {
    const query = new URLSearchParams({ uuid });
    return apiRequest(`/api/comments/delete?${query}`, {
        method: "DELETE",
        token,
        signal,
        guard: isDeleteCommentResponse,
    });
}
