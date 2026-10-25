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
import * as ownerProfileApi from "@/components/modules/settings/owner_profile/lib/service.ts";
import type { RootState } from "@/store";
import type {
    OwnerProfileErrors,
    OwnerProfileFullResponse,
    OwnerProfileUserInput,
    UpdateOwnerProfileResponse,
} from "@/components/modules/settings/owner_profile/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface OwnerProfileState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: OwnerProfileState = {
    status: "idle",
};

export const get_full = createAsyncThunk<
    ApiResult<OwnerProfileFullResponse, never>,
    { silent?: boolean } | undefined,
    { state: RootState }
>(
    "owner_profile/get_full",
    async ({ silent = false } = {}, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await ownerProfileApi.get_full(token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const update = createAsyncThunk<
    ApiResult<UpdateOwnerProfileResponse, OwnerProfileErrors>,
    { input: OwnerProfileUserInput; silent?: boolean },
    { state: RootState }
>(
    "owner_profile/update",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        await dispatch(refreshAccessToken());
        const token = getState().auth.login.token;
        const result = await ownerProfileApi.update(input, token, signal);
        if (!silent) notifyApiError(result);
        return result;
    },
);

const ownerProfileSlice = createSlice({
    name: "owner_profile",
    initialState,
    reducers: {},
});

export default ownerProfileSlice.reducer;
