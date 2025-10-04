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
import * as tasksApi from "@/components/tasks/service.ts";
import type {RootState} from "@/store";
import type {CreateTask} from "@/components/tasks/interface.ts";

interface TasksState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: TasksState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "tasks/create",
  async (requestData: CreateTask, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return tasksApi.create(requestData, token);
  }
)

export const list = createAsyncThunk(
  "tasks/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return tasksApi.list(query, token);
  }
)

export const select_list = createAsyncThunk(
  "tasks/select_list",
  async (list: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tasksApi.select_list(list, token);
  }
)

export const get = createAsyncThunk(
  "tasks/get",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tasksApi.get_resolved(uuid, token);
  }
)

const tasksSlice = createSlice({
  name: "tasks",
  initialState,
  reducers: {},
});

export default tasksSlice.reducer;