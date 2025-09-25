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
import * as inventoryApi from "@/services/inventory.ts";
import type {RootState} from "@/store";
import type {CreateInventory} from "@/lib/interfaces/inventory.ts";

interface InventoryState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: InventoryState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "inventory/create",
  async (requestData: CreateInventory, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await inventoryApi.create(requestData, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const list = createAsyncThunk(
  "inventory/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await inventoryApi.list(query, token);
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const select_list = createAsyncThunk(
  "inventory/select_list",
  async (list: string, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await inventoryApi.select_list(list, token);
    } catch (error: unknown) {
      return rejectWithValue(error)
    }
  }
)

const inventorySlice = createSlice({
  name: "inventory",
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

export default inventorySlice.reducer;