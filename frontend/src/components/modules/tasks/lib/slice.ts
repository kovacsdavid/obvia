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
import * as tasksApi from "@/components/modules/tasks/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateTaskResponse,
    DeleteTaskResponse,
    PaginatedTaskResolvedListResponse,
    TaskErrors,
    TaskResolvedResponse,
    TaskResponse,
    TaskUserInput,
    UpdateTaskResponse,
} from "@/components/modules/tasks/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { SelectOptionListResponse } from "@/lib/interface";

interface TasksState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: TasksState = {
    status: "idle",
};

export const create = createAsyncThunk<
    ApiResult<CreateTaskResponse, TaskErrors>,
    { input: TaskUserInput; silent?: boolean },
    { state: RootState }
>(
    "tasks/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.create(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedTaskResolvedListResponse>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "tasks/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.list(query, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const select_list = createAsyncThunk<
    ApiResult<SelectOptionListResponse>,
    { list: string; silent?: boolean },
    { state: RootState }
>(
    "tasks/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.select_list(list, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get_resolved = createAsyncThunk<
    ApiResult<TaskResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "tasks/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.get_resolved(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<TaskResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "tasks/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.get(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateTaskResponse, TaskErrors>,
    { input: TaskUserInput; silent?: boolean },
    { state: RootState }
>(
    "tasks/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.update(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteTaskResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "tasks/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.deleteItem(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const print = createAsyncThunk<
    ApiResult<Uint8Array<ArrayBuffer> | null>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "tasks/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await tasksApi.print(uuid, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

const tasksSlice = createSlice({
    name: "tasks",
    initialState,
    reducers: {},
});

export default tasksSlice.reducer;
