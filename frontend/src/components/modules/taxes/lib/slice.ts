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
import * as taxesApi from "@/components/modules/taxes/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateTaxResponse,
    DeleteTaxResponse,
    PaginatedTaxResolvedListResponse,
    TaxError,
    TaxResolvedResponse,
    TaxResponse,
    TaxUserInput,
    UpdateTaxResponse,
} from "@/components/modules/taxes/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { SelectOptionListResponse } from "@/lib/interface";

interface TaxesState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: TaxesState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateTaxResponse, TaxError>,
    { input: TaxUserInput; silent?: boolean },
    { state: RootState }
>(
    "taxes/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteTaxResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "taxes/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.deleteItem(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateTaxResponse, TaxError>,
    { input: TaxUserInput; silent?: boolean },
    { state: RootState }
>(
    "taxes/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.update(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedTaxResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "taxes/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.list(
            query,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<TaxResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "taxes/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.get(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_resolved = createAsyncThunk<
    ApiResult<TaxResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "taxes/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.get_resolved(
            uuid,
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
    "taxes/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.select_list(
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
    "taxes/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await taxesApi.print(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const taxesSlice = createSlice({
    name: "taxes",
    initialState,
    reducers: {},
});

export default taxesSlice.reducer;
