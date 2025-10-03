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
import * as worksheetsApi from "@/components/worksheets/service.ts";
import type {RootState} from "@/store";
import type {CreateWorksheet} from "@/components/worksheets/interface.ts";

interface WorksheetsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: WorksheetsState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "worksheets/create",
  async (requestData: CreateWorksheet, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return worksheetsApi.create(requestData, token);
  }
)

export const select_list = createAsyncThunk(
  "worksheets/select_list",
  async (list: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await worksheetsApi.select_list(list, token);
  }
)

export const list = createAsyncThunk(
  "worksheets/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return worksheetsApi.list(query, token);
  }
)

const worksheetsSlice = createSlice({
  name: "worksheets",
  initialState,
  reducers: {},
});

export default worksheetsSlice.reducer;