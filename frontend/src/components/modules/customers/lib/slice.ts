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
import type {
    CreateCustomerResponse,
    CustomerErrors,
    CustomerFullResponse,
    CustomerResponse,
    CustomerUserInput,
    DeleteCustomerResponse,
    PaginatedCustomerResolvedListResponse,
    UpdateCustomerResponse,
} from "@/components/modules/customers/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface CustomersState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: CustomersState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateCustomerResponse, CustomerErrors>,
    { input: CustomerUserInput; silent?: boolean },
    { state: RootState }
>(
    "customers/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.create(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteCustomerResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "customers/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.deleteItem(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateCustomerResponse, CustomerErrors>,
    { input: CustomerUserInput; silent?: boolean },
    { state: RootState }
>(
    "customers/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.update(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedCustomerResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "customers/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.list(query, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<CustomerResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "customers/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.get(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_full = createAsyncThunk<
    ApiResult<CustomerFullResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "customers/get_full",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await customersApi.get_full(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const print = createAsyncThunk<
    ApiResult<Uint8Array<ArrayBuffer> | null>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "customers/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const rootState = getState() as RootState;
        const token = rootState.auth.login.token;
        const result = await customersApi.print(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

const customersSlice = createSlice({
    name: "customers",
    initialState,
    reducers: {},
});

export default customersSlice.reducer;
