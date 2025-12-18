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
import * as customersApi from "@/components/modules/customers/lib/service.ts";
import type { RootState } from "@/store";
import type { CustomerUserInput } from "@/components/modules/customers/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

interface CustomersState {
  status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: CustomersState = {
  status: "idle",
};

export const create = createAsyncThunk(
  "customers/create",
  async (requestData: CustomerUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.create(requestData, token);
  },
);

export const deleteItem = createAsyncThunk(
  "customers/deleteItem",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.deleteItem(uuid, token);
  },
);

export const update = createAsyncThunk(
  "customers/update",
  async (requestData: CustomerUserInput, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.update(requestData, token);
  },
);

export const list = createAsyncThunk(
  "customers/list",
  async (query: string | null, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.list(query, token);
  },
);

export const get = createAsyncThunk(
  "customers/get",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.get(uuid, token);
  },
);

export const get_resolved = createAsyncThunk(
  "customers/get_resolved",
  async (uuid: string, { getState, dispatch }) => {
    await dispatch(refreshAccessToken());
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await customersApi.get_resolved(uuid, token);
  },
);

const customersSlice = createSlice({
  name: "customers",
  initialState,
  reducers: {},
});

export default customersSlice.reducer;
