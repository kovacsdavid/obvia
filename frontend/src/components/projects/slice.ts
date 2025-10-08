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
import * as projectsApi from "@/components/projects/service.ts";
import type {RootState} from "@/store";
import type {ProjectUserInput} from "@/components/projects/interface.ts";

interface ProjectsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: ProjectsState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "projects/create",
  async (requestData: ProjectUserInput, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return projectsApi.create(requestData, token);
  }
)

export const list = createAsyncThunk(
  "projects/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return projectsApi.list(query, token);
  }
)

export const get_resolved = createAsyncThunk(
  "projects/get_resolved",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await projectsApi.get_resolved(uuid, token);
  }
)

export const get = createAsyncThunk(
  "projects/get",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await projectsApi.get(uuid, token);
  }
)

export const update = createAsyncThunk(
  "projects/update",
  async (requestData: ProjectUserInput, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await projectsApi.update(requestData, token);
  }
)

export const deleteItem = createAsyncThunk(
  "projects/deleteItem",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await projectsApi.deleteItem(uuid, token);
  }
);

const projectsSlice = createSlice({
  name: "projects",
  initialState,
  reducers: {},
});

export default projectsSlice.reducer;