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
import * as servicesApi from "@/components/modules/services/lib/service.ts";
import type { RootState } from "@/store";
import type { ServiceUserInput } from "@/components/modules/services/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

interface ServicesState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: ServicesState = {
  status: "idle",
};

export const create = createAsyncThunk(
  "services/create",
  async (requestData: ServiceUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.create(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "services/deleteItem",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.deleteItem(uuid, token);
  },
);

export const update = createAsyncThunk(
  "services/update",
  async (requestData: ServiceUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.update(requestData, token);
  },
);

export const list = createAsyncThunk(
  "services/list",
  async (query: string | null, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.list(query, token);
  },
);

export const get = createAsyncThunk(
  "services/get",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.get(uuid, token);
  },
);

export const get_resolved = createAsyncThunk(
  "services/get_resolved",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.get_resolved(uuid, token);
  },
);

export const select_list = createAsyncThunk(
  "services/select_list",
  async (list: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await servicesApi.select_list(list, token);
  },
);

const servicesSlice = createSlice({
  name: "services",
  initialState,
  reducers: {},
});

export default servicesSlice.reducer;
