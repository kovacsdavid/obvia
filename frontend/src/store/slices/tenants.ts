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

import {createAsyncThunk, createSlice, type PayloadAction} from "@reduxjs/toolkit";
import * as tenantsApi from "@/services/tenants.ts";
import {isTenantsResponse} from "@/services/tenants.ts";
import type {RootState} from "@/store";

interface TenantsState {
  status: "idle" | "loading" | "succeeded" | "failed",
  error: {
    global: string | null,
    fields: Record<string, string | null>
  }
}

const initialState: TenantsState = {
  status: "idle",
  error: {
    global: null,
    fields: {}
  }
}

export const create = createAsyncThunk(
  "tenants/create",
  async (requestData: tenantsApi.TenantsRequest, {rejectWithValue, getState}) => {
    try {
      const rootState = getState() as RootState;
      const token = rootState.auth.login.token;
      const response = await tenantsApi.create(requestData, token);
      if (response.success) {
        return response;
      } else {
        return rejectWithValue(response);
      }
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const list = createAsyncThunk(
  "tenants/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    try {
      const rootState = getState() as RootState;
      const token = rootState.auth.login.token;
      const response = await tenantsApi.list(query, token);
      if (response.success) {
        return response;
      } else {
        return rejectWithValue(response);
      }
    } catch (error: unknown) {
      return rejectWithValue(error);
    }
  }
)

export const activate = createAsyncThunk(
  "tenants/activate",
  async (new_tenant_id: string, {rejectWithValue, getState}) => {
    try {
      const rootState = getState() as RootState;
      const token = rootState.auth.login.token;
      const response = await tenantsApi.activate(new_tenant_id, token);
      if (response.success) {
        return response;
      } else {
        return rejectWithValue(response)
      }
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
        state.error = {global: null, fields: {}};
      })
      .addCase(
        create.fulfilled,
        (
          state,
          action: PayloadAction<tenantsApi.TenantsResponse>
        ) => {
          state.status = "succeeded";
          console.log(action);
          state.error = {global: null, fields: {}};
        })
      .addCase(create.rejected, (state, action) => {
        state.status = "failed";
        if (isTenantsResponse(action.payload) && typeof action.payload?.error !== "undefined") {
          state.error = action.payload.error;
        } else {
          state.error = {global: "Váratlan hiba történt a kommunikáció során", fields: {}};
        }
      })
    builder
      .addCase(list.pending, (state) => {
        state.status = "loading";
        state.error = {
          global: null,
          fields: {}
        };
      })
      .addCase(list.fulfilled, (state) => {
        state.status = "succeeded";
        state.error = {
          global: null,
          fields: {}
        };
      })
      .addCase(list.rejected, (state) => {
        state.status = "failed";
        state.error = {global: "Váratlan hiba történt a kommunikáció során", fields: {}};
      });
    builder
      .addCase(activate.pending, (state) => {
        state.status = "loading";
        state.error = {
          global: null,
          fields: {}
        };
      })
      .addCase(activate.fulfilled, (state) => {
        state.status = "succeeded";
        state.error = {
          global: null,
          fields: {}
        };
      })
      .addCase(activate.rejected, (state) => {
        state.status = "failed";
        state.error = {global: "Váratlan hiba történt a kommunikáció során", fields: {}};
      })
  }
});

export default tenantsSlice.reducer;
