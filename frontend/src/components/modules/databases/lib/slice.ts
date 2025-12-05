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
import * as databasesApi from "@/components/modules/databases/lib/service.ts";
import type { RootState } from "@/store";
import type { CreateDatabase } from "@/components/modules/databases/lib/interface.ts";

interface DatabasesState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: DatabasesState = {
  status: "idle",
};

export const create = createAsyncThunk(
  "databases/create",
  async (requestData: CreateDatabase, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await databasesApi.create(requestData, token);
  },
);

export const list = createAsyncThunk(
  "databases/list",
  async (query: string | null, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await databasesApi.list(query, token);
  },
);

export const activate = createAsyncThunk(
  "databases/activate",
  async (new_tenant_id: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await databasesApi.activate(new_tenant_id, token);
  },
);

export const get = createAsyncThunk(
  "databases/get",
  async (uuid: string, { getState }) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await databasesApi.get_resolved(uuid, token);
  },
);

const databasesSlice = createSlice({
  name: "databases",
  initialState,
  reducers: {},
});

export default databasesSlice.reducer;
