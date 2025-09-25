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

import {createAsyncThunk, createSlice} from "@reduxjs/toolkit";
import * as tagsApi from "@/services/tags.ts";
import type {RootState} from "@/store";
import type {CreateTag} from "@/lib/interfaces/tags.ts";

interface TagsState {
  status: "idle" | "loading" | "succeeded" | "failed",
  error: {
    global: string | null,
    fields: Record<string, string | null>
  }
}

const initialState: TagsState = {
  status: "idle",
  error: {
    global: null,
    fields: {}
  }
}

export const create = createAsyncThunk(
  "tags/create",
  async (requestData: CreateTag, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return tagsApi.create(requestData, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const list = createAsyncThunk(
  "tags/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return tagsApi.list(query, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

const tagsSlice = createSlice({
  name: "tags",
  initialState,
  reducers: {},
  extraReducers: (builder) => {
    builder
      .addCase(create.pending, (state) => {
        state.status = "loading";
      })
      .addCase(
        create.fulfilled,
        (
          state,
        ) => {
          state.status = "succeeded";
        })
      .addCase(create.rejected, (state) => {
        state.status = "failed";
      })
    builder
      .addCase(list.pending, (state) => {
        state.status = "loading";
      })
      .addCase(list.fulfilled, (state) => {
        state.status = "succeeded";
      })
      .addCase(list.rejected, (state) => {
        state.status = "failed";
      });
  }
});

export default tagsSlice.reducer;