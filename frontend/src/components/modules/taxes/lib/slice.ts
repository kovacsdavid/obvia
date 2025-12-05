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
import * as taxesApi from "@/components/modules/taxes/lib/service.ts";
import type { RootState } from "@/store";
import type { TaxUserInput } from "@/components/modules/taxes/lib/interface.ts";

interface TaxesState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: TaxesState = {
  status: "idle",
};

export const create = createAsyncThunk(
  "taxes/create",
  async (requestData: TaxUserInput, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.create(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "taxes/deleteItem",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.deleteItem(uuid, token);
  },
);

export const update = createAsyncThunk(
  "taxes/update",
  async (requestData: TaxUserInput, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.update(requestData, token);
  },
);

export const list = createAsyncThunk(
  "taxes/list",
  async (query: string | null, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.list(query, token);
  },
);

export const get = createAsyncThunk(
  "taxes/get",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.get(uuid, token);
  },
);

export const get_resolved = createAsyncThunk(
  "taxes/get_resolved",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.get_resolved(uuid, token);
  },
);

export const select_list = createAsyncThunk(
  "taxes/select_list",
  async (list: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await taxesApi.select_list(list, token);
  },
);

const taxesSlice = createSlice({
  name: "taxes",
  initialState,
  reducers: {},
});

export default taxesSlice.reducer;
