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

import { z } from "zod";
import {
    paginatedSuccessResponseSchema,
    successResponseSchema,
} from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const activityFeedResolvedEntrySchema = z.looseObject({
    id: z.string(),
    resource_id: z.string(),
    resource_type: z.string(),
    activity_type: z.string(),
    content: z.string(),
    created_by_id: z.string(),
    created_by: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const commentSchema = z.looseObject({
    id: z.string(),
    commentable_type: z.string(),
    commentable_id: z.string(),
    comment: z.string(),
    created_by_id: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const commentErrorSchema = z
    .object({
        id: nullableString.optional(),
        commentable_type: nullableString.optional(),
        commentable_id: nullableString.optional(),
        comment: nullableString.optional(),
    })
    .catchall(nullableString);

export const activityFeedArraySchema = z.array(activityFeedResolvedEntrySchema);
export const postCommentResponseSchema = successResponseSchema(commentSchema);
export const deleteCommentResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
);
export const paginatedActivityFeedResponseSchema =
    paginatedSuccessResponseSchema(activityFeedArraySchema);
