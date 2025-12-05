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

import { createAsyncThunk, createSlice } from "@reduxjs/toolkit";
import * as tagsApi from "@/components/modules/tags/lib/service.ts";
import type { RootState } from "@/store";
import type { TagUserInput } from "@/components/modules/tags/lib/interface.ts";

interface TagsState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: TagsState = {
  status: "idle",
};

export const create = createAsyncThunk(
  "tags/create",
  async (requestData: TagUserInput, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return tagsApi.create(requestData, token);
  },
);

export const list = createAsyncThunk(
  "tags/list",
  async (query: string | null, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return tagsApi.list(query, token);
  },
);

export const get_resolved = createAsyncThunk(
  "tags/get_resolved",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tagsApi.get_resolved(uuid, token);
  },
);

export const get = createAsyncThunk(
  "tags/get",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tagsApi.get(uuid, token);
  },
);

export const update = createAsyncThunk(
  "tags/update",
  async (requestData: TagUserInput, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tagsApi.update(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "tags/deleteItem",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tagsApi.deleteItem(uuid, token);
  },
);

const tagsSlice = createSlice({
  name: "tags",
  initialState,
  reducers: {},
});

export default tagsSlice.reducer;
