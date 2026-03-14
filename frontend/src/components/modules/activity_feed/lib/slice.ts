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
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

type ActivityFeedState = object;

const initialState: ActivityFeedState = {};

export const list = createAsyncThunk(
  "activity_feed/list",
  async (
    {
      resourceId,
      resourceType,
    }: {
      resourceId: string;
      resourceType: string;
    },
    { getState, dispatch },
  ) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await activityFeedApi.list(resourceId, resourceType, token);
  },
);

export const postComment = createAsyncThunk(
  "activity_feed/post_comment",
  async (
    {
      resourceId,
      resourceType,
      comment,
    }: {
      resourceId: string;
      resourceType: string;
      comment: string;
    },
    { getState, dispatch },
  ) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await activityFeedApi.post_comment(
      resourceId,
      resourceType,
      comment,
      token,
    );
  },
);

const activityFeedSlice = createSlice({
  name: "activity_feed",
  initialState,
  reducers: {},
});

export default activityFeedSlice.reducer;
