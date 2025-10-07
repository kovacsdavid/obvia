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
import * as warehousesApi from "@/components/warehouses/service.ts";
import type {RootState} from "@/store";
import type {WarehouseUserInput} from "@/components/warehouses/interface.ts";

interface WarehousesState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: WarehousesState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "warehouses/create",
  async (requestData: WarehouseUserInput, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return warehousesApi.create(requestData, token);
  }
)

export const list = createAsyncThunk(
  "warehouses/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return warehousesApi.list(query, token);
  }
)

export const get_resolved = createAsyncThunk(
  "warehouses/get_resolved",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await warehousesApi.get_resolved(uuid, token);
  }
)

export const get = createAsyncThunk(
  "warehouses/get",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await warehousesApi.get(uuid, token);
  }
)

export const update = createAsyncThunk(
  "warehouses/update",
  async (requestData: WarehouseUserInput, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await warehousesApi.update(requestData, token);
  }
)

export const deleteWarehouse = createAsyncThunk(
  "warehouses/deleteWarehouse",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await warehousesApi.deleteWarehouse(uuid, token);
  }
);

const warehousesSlice = createSlice({
  name: "warehouses",
  initialState,
  reducers: {},
});

export default warehousesSlice.reducer;