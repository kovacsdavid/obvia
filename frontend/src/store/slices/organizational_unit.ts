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

import {createAsyncThunk, createSlice, type PayloadAction} from "@reduxjs/toolkit";
import * as organizationalUnitApi from "@/services/organizational_unit";
import {isOrganizationalUnitResponse} from "@/services/organizational_unit";
import type {RootState} from "@/store";

interface OrganizationalUnitState {
    status: "idle" | "loading" | "succeeded" | "failed",
    error: {
        global: string | null,
        fields: Record<string, string | null>
    }
}

const initialState: OrganizationalUnitState = {
    status: "idle",
    error: {
        global: null,
        fields: {}
    }
}

export const create = createAsyncThunk(
    "organizational_unit/create",
    async (requestData: organizationalUnitApi.OrganizationalUnitRequest, { rejectWithValue, getState }) => {
        try {
            const rootState = getState() as RootState;
            const token =  rootState.auth.login.token;
            const response = await organizationalUnitApi.create(requestData, token);
            if (response.success) {
                return response;
            } else {
                return rejectWithValue(response);
            }
        } catch (error: unknown) {
            return rejectWithValue(error);
        }
    }
)

const organizationalUnitSlice = createSlice({
    name: "organizational_unit",
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(create.pending, (state) => {
                state.status = "loading";
                state.error = { global: null, fields: {}};
            })
            .addCase(
                create.fulfilled,
                (
                    state,
                    action: PayloadAction<organizationalUnitApi.OrganizationalUnitResponse>
                ) => {
                state.status = "succeeded";
                console.log(action);
                state.error = { global: null, fields: {}};
            })
            .addCase(create.rejected, (state, action) => {
                state.status = "failed";
                if (isOrganizationalUnitResponse(action.payload) && typeof action.payload?.error !== "undefined") {
                    state.error = action.payload.error;
                } else {
                    state.error = {global: "Váratlan hiba történt a kommunikáció során", fields: {}};
                }
            })
    }
});

export default organizationalUnitSlice.reducer;
