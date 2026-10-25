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
import * as worksheetsApi from "@/components/modules/worksheets/lib/service.ts";
import type { RootState } from "@/store";
import type {
    WorksheetUserInput,
    WorksheetError,
    CreateWorksheetResponse,
    PaginatedWorksheetResolvedListResponse,
    WorksheetResolvedResponse,
    WorksheetResponse,
    UpdateWorksheetResponse,
    DeleteWorksheetResponse,
} from "@/components/modules/worksheets/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { SelectOptionListResponse } from "@/lib/interface";

interface WorksheetsState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: WorksheetsState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateWorksheetResponse, WorksheetError>,
    { input: WorksheetUserInput; silent?: boolean },
    { state: RootState }
>(
    "worksheets/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.create(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const select_list = createAsyncThunk<
    ApiResult<SelectOptionListResponse>,
    { list: string; silent?: boolean },
    { state: RootState }
>(
    "worksheets/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.select_list(list, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedWorksheetResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "worksheets/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.list(query, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_resolved = createAsyncThunk<
    ApiResult<WorksheetResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "worksheets/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.get_resolved(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<WorksheetResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "worksheets/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.get(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateWorksheetResponse, WorksheetError>,
    { input: WorksheetUserInput; silent?: boolean },
    { state: RootState }
>(
    "worksheets/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.update(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteWorksheetResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "worksheets/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.deleteItem(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const print = createAsyncThunk<
    ApiResult<Uint8Array<ArrayBuffer> | null>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "worksheets/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await worksheetsApi.print(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

const worksheetsSlice = createSlice({
    name: "worksheets",
    initialState,
    reducers: {},
});

export default worksheetsSlice.reducer;
