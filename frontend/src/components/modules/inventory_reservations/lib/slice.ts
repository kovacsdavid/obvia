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
import * as InventoryReservationsApi from "@/components/modules/inventory_reservations/lib/service.ts";
import type { RootState } from "@/store";
import type { InventoryReservationUserInput } from "@/components/modules/inventory_reservations/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

interface InventoryReservationsState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: InventoryReservationsState = {
  status: "idle",
};

export const get_resolved = createAsyncThunk(
  "inventory_reservations/get_resolved",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.get_resolved(uuid, token);
  },
);

export const get = createAsyncThunk(
  "inventory_reservations/get",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.get(uuid, token);
  },
);

export const create = createAsyncThunk(
  "inventory_reservations/create",
  async (
    requestData: InventoryReservationUserInput,
    { getState, dispatch },
  ) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.create(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "inventory_reservations/deleteItem",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.deleteItem(uuid, token);
  },
);

export const select_list = createAsyncThunk(
  "inventory_reservations/select_list",
  async (list: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.select_list(list, token);
  },
);

export const list = createAsyncThunk(
  "inventory_reservations/list",
  async (query: string | null, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await InventoryReservationsApi.list(query, token);
  },
);

const InventoryReservationsSlice = createSlice({
  name: "inventory_reservations",
  initialState,
  reducers: {},
});

export default InventoryReservationsSlice.reducer;
