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
import * as databasesApi from "@/components/modules/databases/lib/service.ts";
import type { RootState } from "@/store";
import type {
    ActiveDatabaseResponse,
    CreateDatabase,
    CreateDatabaseResponse,
    DatabaseError,
    DatabaseResponse,
    PaginatedDatabaseListResponse,
} from "@/components/modules/databases/lib/interface.ts";
import {
    isLogoutInProgress,
    refreshAccessToken,
} from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface DatabasesState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: DatabasesState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateDatabaseResponse, DatabaseError>,
    { input: CreateDatabase; silent?: boolean },
    { state: RootState }
>(
    "databases/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await databasesApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedDatabaseListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "databases/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await databasesApi.list(
            query,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const activate = createAsyncThunk<
    ApiResult<ActiveDatabaseResponse>,
    { uuid: string | null; silent?: boolean },
    { state: RootState }
>(
    "databases/activate",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await databasesApi.activate(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<DatabaseResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "databases/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await databasesApi.get_resolved(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteDatabase = createAsyncThunk<
    ApiResult<ActiveDatabaseResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "database/deleteDatabase",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await databasesApi.deleteDatabase(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const databasesSlice = createSlice({
    name: "databases",
    initialState,
    reducers: {},
});

export default databasesSlice.reducer;
