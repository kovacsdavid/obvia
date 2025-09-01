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
import * as worksheetsApi from "@/services/worksheets.ts";

interface WorksheetsState {
  status: "idle" | "loading" | "succeeded" | "failed",
  error: {
    global: string | null,
    fields: Record<string, string | null>
  }
}

const initialState: WorksheetsState = {
  status: "idle",
  error: {
    global: null,
    fields: {}
  }
}

export const create = createAsyncThunk(
  "worksheets/create",
  async (requestData: worksheetsApi.CreateWorksheet, {rejectWithValue, getState}) => {
    console.log(requestData, rejectWithValue, getState);
    // TODO
  }
)

export const list = createAsyncThunk(
  "worksheets/list",
  async (query: string | null, {rejectWithValue, getState}) => {
    console.log(query, rejectWithValue, getState);
    // TODO
  }
)

const worksheetsSlice = createSlice({
  name: "worksheets",
  initialState,
  reducers: {},
  extraReducers: () => {
    // TODO: extraReducers
  }
});

export default worksheetsSlice.reducer;