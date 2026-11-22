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
import * as warehousesApi from "@/components/modules/warehouses/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateWarehouseResponse,
    UpdateWarehouseResponse,
    DeleteWarehouseResponse,
    PaginatedWarehouseResolvedListResponse,
    WarehouseResponse,
    WarehouseResolvedResponse,
    WarehouseUserInput,
    WarehouseError,
} from "@/components/modules/warehouses/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface WarehousesState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: WarehousesState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateWarehouseResponse, WarehouseError>,
    { input: WarehouseUserInput; silent?: boolean },
    { state: RootState }
>(
    "warehouses/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedWarehouseResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "warehouses/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.list(
            query,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_resolved = createAsyncThunk<
    ApiResult<WarehouseResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "warehouses/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.get_resolved(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<WarehouseResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "warehouses/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.get(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateWarehouseResponse, WarehouseError>,
    { input: WarehouseUserInput; silent?: boolean },
    { state: RootState }
>(
    "warehouses/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.update(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteWarehouseResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "warehouses/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.deleteItem(
            uuid,
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
    "warehouses/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await warehousesApi.print(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const warehousesSlice = createSlice({
    name: "warehouses",
    initialState,
    reducers: {},
});

export default warehousesSlice.reducer;
