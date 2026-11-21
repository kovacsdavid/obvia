/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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
import * as handoverAndAcceptanceApi from "@/components/modules/handover_and_acceptance/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateHandoverAndAcceptanceResponse,
    DeleteHandoverAndAcceptanceResponse,
    HandoverAndAcceptanceError,
    HandoverAndAcceptanceFullResponse,
    HandoverAndAcceptanceUserInput,
    PaginatedHandoverAndAcceptanceFullListResponse,
    UpdateHandoverAndAcceptanceResponse,
} from "@/components/modules/handover_and_acceptance/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface HandoverAndAcceptanceState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: HandoverAndAcceptanceState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateHandoverAndAcceptanceResponse, HandoverAndAcceptanceError>,
    { input: HandoverAndAcceptanceUserInput; silent?: boolean },
    { state: RootState }
>(
    "handover_and_acceptance/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.create(
            input,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedHandoverAndAcceptanceFullListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "handover_and_acceptance/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.list(
            query,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_full = createAsyncThunk<
    ApiResult<HandoverAndAcceptanceFullResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "handover_and_acceptance/get_full",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.get_full(
            uuid,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateHandoverAndAcceptanceResponse, HandoverAndAcceptanceError>,
    { input: HandoverAndAcceptanceUserInput; silent?: boolean },
    { state: RootState }
>(
    "handover_and_acceptance/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.update(
            input,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteHandoverAndAcceptanceResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "handover_and_acceptance/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.deleteItem(
            uuid,
            token,
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
    "handover_and_acceptance/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await handoverAndAcceptanceApi.print(
            uuid,
            token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const handoverAndAcceptanceSlice = createSlice({
    name: "handover_and_acceptance",
    initialState,
    reducers: {},
});

export default handoverAndAcceptanceSlice.reducer;
