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
import * as inventoryMovementsApi from "@/components/modules/inventory_movements/lib/service.ts";
import type {RootState} from "@/store";
import type {InventoryMovementUserInput} from "@/components/modules/inventory_movements/lib/interface.ts";

interface InventoryMovementsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: InventoryMovementsState = {
  status: "idle",
}

export const get_resolved = createAsyncThunk(
  "inventory_movements/get_resolved",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.get_resolved(uuid, token);
  }
)

export const get = createAsyncThunk(
  "inventory_movements/get",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.get(uuid, token);
  }
)

export const create = createAsyncThunk(
  "inventory_movements/create",
  async (requestData: InventoryMovementUserInput, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.create(requestData, token);
  }
)

export const deleteItem = createAsyncThunk(
  "inventory_movements/deleteItem",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.deleteItem(uuid, token);
  }
);


export const select_list = createAsyncThunk(
  "inventory_movements/select_list",
  async (list: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.select_list(list, token);
  }
)

export const list = createAsyncThunk(
  "inventory_movements/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await inventoryMovementsApi.list(query, token);
  }
)


const inventoryMovementsSlice = createSlice({
  name: "inventory_movements",
  initialState,
  reducers: {},
});

export default inventoryMovementsSlice.reducer;
