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
import * as productsApi from "@/components/products/service.ts";
import type {RootState} from "@/store";
import type {CreateProduct} from "@/components/products/interface.ts";

interface ProductsState {
  status: "idle" | "loading" | "succeeded" | "failed",
}

const initialState: ProductsState = {
  status: "idle",
}

export const create = createAsyncThunk(
  "products/create",
  async (requestData: CreateProduct, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await productsApi.create(requestData, token);
  }
)

export const list = createAsyncThunk(
  "products/list",
  async (query: string | null, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await productsApi.list(query, token);
  }
)

export const select_list = createAsyncThunk(
  "products/select_list",
  async (list: string, {getState}) => {
    const rootState = getState() as RootState;
    const token = rootState.auth.login.token;
    return await productsApi.select_list(list, token);
  }
)

const productsSlice = createSlice({
  name: "products",
  initialState,
  reducers: {},
});

export default productsSlice.reducer;