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
import * as inventoryMovementsApi from "@/components/modules/inventory_movements/lib/service.ts";
import type { RootState } from "@/store";
import type {
    CreateInventoryMovementResponse,
    DeleteInventoryMovementResponse,
    InventoryMovementError,
    InventoryMovementResolvedResponse,
    InventoryMovementResponse,
    InventoryMovementUserInput,
    PaginatedInventoryMovementResolvedListResponse,
} from "@/components/modules/inventory_movements/lib/interface.ts";
import {
    isLogoutInProgress,
    refreshAccessToken,
} from "@/components/modules/auth/lib/slice.ts";
import type { SelectOptionListResponse } from "@/lib/interface";
import type { ApiResult } from "@/lib/api/types";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

interface InventoryMovementsState {
    status: "idle" | "loading" | "succeeded" | "failed";
}

const initialState: InventoryMovementsState = {
    status: "idle",
};

export const get_resolved = createAsyncThunk<
    ApiResult<InventoryMovementResolvedResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory_movements/get_resolved",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.get_resolved(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const get = createAsyncThunk<
    ApiResult<InventoryMovementResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory_movements/get",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.get(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const create = createAsyncThunk<
    ApiResult<CreateInventoryMovementResponse, InventoryMovementError>,
    { input: InventoryMovementUserInput; silent?: boolean },
    { state: RootState }
>(
    "inventory_movements/create",
    async ({ input, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.create(
            input,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const deleteItem = createAsyncThunk<
    ApiResult<DeleteInventoryMovementResponse>,
    { uuid: string; silent?: boolean },
    { state: RootState }
>(
    "inventory_movements/deleteItem",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.deleteItem(
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
    "inventory_movements/select_list",
    async ({ list, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.select_list(
            list,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

export const list = createAsyncThunk<
    ApiResult<PaginatedInventoryMovementResolvedListResponse>,
    { inventoryId: string; query: string | null; silent?: boolean },
    { state: RootState }
>(
    "inventory_movements/list",
    async (
        { inventoryId, query, silent = false },
        { getState, dispatch, signal },
    ) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.list(
            { inventoryId, query },
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
    "inventory_movements/print",
    async ({ uuid, silent = false }, { getState, dispatch, signal }) => {
        const refresh = await dispatch(refreshAccessToken({ silent }));
        if (!refreshAccessToken.fulfilled.match(refresh))
            return { kind: "aborted" };
        if (refresh.payload.kind !== "success") return refresh.payload;
        if (!getState().auth.login.token || isLogoutInProgress())
            return { kind: "aborted" };
        const result = await inventoryMovementsApi.print(
            uuid,
            getState().auth.login.token,
            signal,
        );
        if (!silent) notifyApiError(result);
        return result;
    },
);

const inventoryMovementsSlice = createSlice({
    name: "inventory_movements",
    initialState,
    reducers: {},
});

export default inventoryMovementsSlice.reducer;
