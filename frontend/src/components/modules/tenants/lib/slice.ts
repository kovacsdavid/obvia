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
import * as tenantsApi from "@/components/modules/tenants/lib/service.ts";
import type {RootState} from "@/store";
import type {CreateTenant} from "@/components/modules/tenants/lib/interface.ts";

interface TenantsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: TenantsState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "tenants/create",
  async (requestData: CreateTenant, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tenantsApi.create(requestData, token);
  }
)

export const list = createAsyncThunk(
  "tenants/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tenantsApi.list(query, token);
  }
)

export const activate = createAsyncThunk(
  "tenants/activate",
  async (new_tenant_id: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tenantsApi.activate(new_tenant_id, token);
  }
)

export const get = createAsyncThunk(
  "tenants/get",
  async (uuid: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await tenantsApi.get_resolved(uuid, token);
  }
)

const tenantsSlice = createSlice({
  name: "tenants",
  initialState,
  reducers: {},
});

export default tenantsSlice.reducer;
