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
import * as servicesApi from "@/components/modules/services/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateServiceResponse,
    DeleteServiceResponse,
    PaginatedServiceResolvedListResponse,
    ServiceError,
    ServiceResolvedResponse,
    ServiceResponse,
    ServiceUserInput,
    UpdateServiceResponse,
} from "@/components/modules/services/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { SelectOptionListResponse } from "@/lib/interface";

interface ServicesState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: ServicesState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateServiceResponse, ServiceError>,
    { input: ServiceUserInput; silent?: boolean },
    { state: RootState }
>(
    "services/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.create(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteServiceResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "services/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.deleteItem(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateServiceResponse, ServiceError>,
    { input: ServiceUserInput; silent?: boolean },
    { state: RootState }
>(
    "services/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.update(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedServiceResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "services/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.list(query, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<ServiceResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "services/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.get(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_resolved = createAsyncThunk<
    ApiResult<ServiceResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "services/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.get_resolved(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const select_list = createAsyncThunk<
    ApiResult<SelectOptionListResponse>,
    { list: string; silent?: boolean },
    { state: RootState }
>(
    "services/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.select_list(list, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const print = createAsyncThunk<
    ApiResult<Uint8Array<ArrayBuffer> | null>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "services/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await servicesApi.print(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

const servicesSlice = createSlice({
    name: "services",
    initialState,
    reducers: {},
});

export default servicesSlice.reducer;
