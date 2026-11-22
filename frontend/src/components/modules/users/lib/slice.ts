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
import * as usersApi from "@/components/modules/users/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateUser,
    OtpFields,
} from "@/components/modules/users/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { EnableOtpResponse } from "@/components/modules/auth/lib/interface";
import type { SimpleMessageData } from "@/lib/interface";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface UsersState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: UsersState = {
    status: "idle",
};

export const enableOtp = createAsyncThunk<
    ApiResult<EnableOtpResponse>,
    { silent?: boolean } | void,
    { state: RootState }
>("users/enableOtp", async (options, { getState, dispatch, signal }) => {
    await dispatch(refreshAccessToken());
    const result = await usersApi.enableOtp(
        getState().auth.login.token,
        signal,
    );
    if (!options?.silent) notifyApiError(result);
    return result;
});

export const verifyOtp = createAsyncThunk<
    ApiResult<{ data: SimpleMessageData }, OtpFields>,
    { otp: string; silent?: boolean },
    { state: RootState }
>(
    "users/verifyOtp",
    async ({ otp, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await usersApi.verifyOtp(
            otp,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const disableOtp = createAsyncThunk<
    ApiResult<{ data: SimpleMessageData }, OtpFields>,
    { otp: string; silent?: boolean },
    { state: RootState }
>(
    "users/disableOtp",
    async ({ otp, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await usersApi.disableOtp(
            otp,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const create = createAsyncThunk<
    ApiResult<never>,
    { input: CreateUser; silent?: boolean },
    { state: RootState }
>(
    "users/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await usersApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<never>,
    { query: string | null; silent?: boolean },
    { state: RootState }
>(
    "users/list",
    async ({ query, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const result = await usersApi.list(
            query,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const usersSlice = createSlice({
    name: "users",
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(create.pending, (state) => {
                state.status = "loading";
            })
            .addCase(create.fulfilled, (state, action) => {
                state.status =
                    action.payload.kind === "success" ? "succeeded" : "failed";
            })
            .addCase(create.rejected, (state) => {
                state.status = "failed";
            });
        builder
            .addCase(list.pending, (state) => {
                state.status = "loading";
            })
            .addCase(list.fulfilled, (state, action) => {
                state.status =
                    action.payload.kind === "success" ? "succeeded" : "failed";
            })
            .addCase(list.rejected, (state) => {
                state.status = "failed";
            });
    },
});

export default usersSlice.reducer;
