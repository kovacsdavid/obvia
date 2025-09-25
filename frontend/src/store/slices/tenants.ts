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
import * as tenantsApi from "@/services/tenants.ts";
import type {RootState} from "@/store";
import type {CreateTenant} from "@/lib/interfaces/tenants.ts";

interface TenantsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: TenantsState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "tenants/create",
  async (requestData: CreateTenant, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await tenantsApi.create(requestData, token);
    } catch (error: unknown ){
      return rejectWithValue(error);
    }
  }
)

export const list = createAsyncThunk(
  "tenants/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await tenantsApi.list(query, token);
    } catch (error: unknown) {
      rejectWithValue(error)
    }
  }
)

export const activate = createAsyncThunk(
  "tenants/activate",
  async (new_tenant_id: string, {rejectWithValue, getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    try {
      return await tenantsApi.activate(new_tenant_id, token);
    } catch (error: unknown) {
      return rejectWithValue(error)
    }
  }
)

const tenantsSlice = createSlice({
  name: "tenants",
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
    builder
      .addCase(activate.pending, (state) => {
        state.status = "loading";
      })
      .addCase(activate.fulfilled, (state) => {
        state.status = "succeeded";
      })
      .addCase(activate.rejected, (state) => {
        state.status = "failed";
      })
  }
});

export default tenantsSlice.reducer;
