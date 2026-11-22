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

import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import * as activityFeedApi from "@/components/modules/activity_feed/lib/service";
import type { RootState } from "@/store";
import {
    isLogoutInProgress,
    refreshAccessToken,
} from "@/components/modules/auth/lib/slice.ts";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { ApiResult } from "@/lib/api/types";
import type {
    CommentError,
    DeleteCommentResponse,
    PaginatedActivityFeedResponse,
    PostCommentResponse,
} from "./interface";

type ActivityFeedState = object;

const initialState: ActivityFeedState = {};

export const list = createAsyncThunk<
    ApiResult<PaginatedActivityFeedResponse>,
    { resourceId: string; resourceType: string; silent?: boolean },
    { state: RootState }
>(
    "activity_feed/list",
    async (
        { resourceId, resourceType, silent = false },
        { getState, dispatch, signal },
    ) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const token = getState().auth.login.token;
        const result = await activityFeedApi.list(
            resourceId,
            resourceType,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const postComment = createAsyncThunk<
    ApiResult<PostCommentResponse, CommentError>,
    {
        resourceId: string;
        resourceType: string;
        comment: string;
        silent?: boolean;
    },
    { state: RootState }
>(
    "activity_feed/post_comment",
    async (
        { resourceId, resourceType, comment, silent = false },
        { getState, dispatch, signal },
    ) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const token = getState().auth.login.token;
        const result = await activityFeedApi.post_comment(
            resourceId,
            resourceType,
            comment,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteComment = createAsyncThunk<
    ApiResult<DeleteCommentResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "activity_feed/delete_comment",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const token = getState().auth.login.token;
        const result = await activityFeedApi.delete_comment(
            uuid,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const activityFeedSlice = createSlice({
    name: "activity_feed",
    initialState,
    reducers: {},
});

export default activityFeedSlice.reducer;
