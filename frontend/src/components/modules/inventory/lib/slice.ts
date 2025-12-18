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
import * as inventoryApi from "@/components/modules/inventory/lib/service.ts";
import type { RootState } from "@/store";
import type { InventoryUserInput } from "@/components/modules/inventory/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

interface InventoryState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: InventoryState = {
  status: "idle",
};

export const get_resolved = createAsyncThunk(
  "inventory/get_resolved",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.get_resolved(uuid, token);
  },
);

export const get = createAsyncThunk(
  "inventory/get",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.get(uuid, token);
  },
);

export const update = createAsyncThunk(
  "inventory/update",
  async (requestData: InventoryUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.update(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "inventory/deleteItem",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.deleteItem(uuid, token);
  },
);

export const create = createAsyncThunk(
  "inventory/create",
  async (requestData: InventoryUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.create(requestData, token);
  },
);

export const list = createAsyncThunk(
  "inventory/list",
  async (query: string | null, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.list(query, token);
  },
);

export const select_list = createAsyncThunk(
  "inventory/select_list",
  async (list: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryApi.select_list(list, token);
  },
);

const inventorySlice = createSlice({
  name: "inventory",
  initialState,
  reducers: {},
});

export default inventorySlice.reducer;
