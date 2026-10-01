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
import type { OwnerProfileUserInput } from "@/components/modules/settings/owner_profile/lib/interface.ts";
import { refreshAccessToken } from "@/components/modules/auth/lib/slice.ts";

interface OwnerProfileState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: OwnerProfileState = {
    status: "idle",
};

export const get_full = createAsyncThunk(
    "owner_profile/get_full",
    async (_, { getState, dispatch }) => {
        await dispatch(refreshAccessToken());
        const rootState = getState() as RootState;
        const token = rootState.auth.login.token;
        return await ownerProfileApi.get_full(token);
    },
);

export const update = createAsyncThunk(
    "owner_profile/update",
    async (requestData: OwnerProfileUserInput, { getState, dispatch }) => {
        await dispatch(refreshAccessToken());
        const rootState = getState() as RootState;
        const token = rootState.auth.login.token;
        return await ownerProfileApi.update(requestData, token);
    },
);

const ownerProfileSlice = createSlice({
    name: "owner_profile",
    initialState,
    reducers: {},
});

export default ownerProfileSlice.reducer;
