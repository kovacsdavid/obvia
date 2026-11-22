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
import * as inventoryApi from "@/components/modules/inventory/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateInventoryResponse,
    DeleteInventoryResponse,
    InventoryError,
    InventoryResolvedResponse,
    InventoryResponse,
    InventoryUserInput,
    PaginatedInventoryResolvedListResponse,
    UpdateInventoryResponse,
} from "@/components/modules/inventory/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { SelectOptionListResponse } from "@/lib/interface";

interface InventoryState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: InventoryState = {
    status: "idle",
};

export const get_resolved = createAsyncThunk<
    ApiResult<InventoryResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.get_resolved(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<InventoryResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.get(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateInventoryResponse, InventoryError>,
    { input: InventoryUserInput; silent?: boolean },
    { state: RootState }
>(
    "inventory/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.update(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteInventoryResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.deleteItem(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const create = createAsyncThunk<
    ApiResult<CreateInventoryResponse, InventoryError>,
    { input: InventoryUserInput; silent?: boolean },
    { state: RootState }
>(
    "inventory/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedInventoryResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "inventory/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.list(
            query,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const select_list = createAsyncThunk<
    ApiResult<SelectOptionListResponse>,
    { list: string; silent?: boolean },
    { state: RootState }
>(
    "inventory/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.select_list(
            list,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const print = createAsyncThunk<
    ApiResult<Uint8Array<ArrayBuffer> | null>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await inventoryApi.print(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const inventorySlice = createSlice({
    name: "inventory",
    initialState,
    reducers: {},
});

export default inventorySlice.reducer;
