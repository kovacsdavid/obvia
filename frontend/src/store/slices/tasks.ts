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
import * as tasksApi from "@/services/tasks.ts";
import type {RootState} from "@/store";

interface TasksState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: TasksState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "tasks/create",
  async (requestData: tasksApi.CreateTask, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return tasksApi.create(requestData, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const list = createAsyncThunk(
  "tasks/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return tasksApi.list(query, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const select_list = createAsyncThunk(
  "tasks/select_list",
  async (list: string, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await tasksApi.select_list(list, token);
    } catch (error: unknown) {
      return rejectWithValue(error)
    }
  }
)

const tasksSlice = createSlice({
  name: "tasks",
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

export default tasksSlice.reducer;