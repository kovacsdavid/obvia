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

import { refreshAccessToken } from "@/components/modules/auth/lib/slice";
import { createAsyncThunk } from "@reduxjs/toolkit";
import * as addressApi from "@/components/modules/address/lib/service";
import type { RootState } from "@/store";

export const select_list = createAsyncThunk(
    "taxes/select_list",
    async (list: string, { getState, dispatch }) => {
        await dispatch(refreshAccessToken());
        const rootState = getState() as RootState;
        const token = rootState.auth.login.token;
        return await addressApi.select_list(list, token);
    },
);
