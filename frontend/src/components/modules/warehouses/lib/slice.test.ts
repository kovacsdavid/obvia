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

import { configureStore } from "@reduxjs/toolkit";
import { beforeEach, describe, expect, it, vi } from "vitest";
import type { RootState } from "@/store";
import * as api from "./service";
import * as thunks from "./slice";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

vi.mock("./service", () => ({
    create: vi.fn(),
    update: vi.fn(),
    list: vi.fn(),
    get: vi.fn(),
    get_resolved: vi.fn(),
    deleteItem: vi.fn(),
    print: vi.fn(),
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));
vi.mock("@/components/modules/auth/lib/slice.ts", async () => {
    const { createAsyncThunk } = await import("@reduxjs/toolkit");
    return {
        isLogoutInProgress: () => false,
        refreshAccessToken: createAsyncThunk(
            "auth/refreshAccessToken",
            async (_, { dispatch }) => {
                dispatch({ type: "refreshed" });
                return { kind: "success" as const };
            },
        ),
    };
});

const input = {
    id: null,
    name: "Main",
    contact_name: "",
    contact_phone: "",
    status: "active",
};
const result = { kind: "http_error", status: 500 } as const;
function makeStore() {
    const initialState = { auth: { login: { token: "old" } } } as RootState;
    return configureStore({
        reducer: (state = initialState, action) =>
            action.type === "refreshed"
                ? {
                      ...state,
                      auth: {
                          ...state.auth,
                          login: { ...state.auth.login, token: "fresh" },
                      },
                  }
                : state,
    });
}
type Dispatch = ReturnType<typeof makeStore>["dispatch"];
const cases = [
    {
        api: api.create,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.create({ input, silent })),
    },
    {
        api: api.update,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.update({ input, silent })),
    },
    {
        api: api.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.list({ query: "page=2", silent })),
    },
    {
        api: api.get,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.get({ uuid: "id", silent })),
    },
    {
        api: api.get_resolved,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.get_resolved({ uuid: "id", silent })),
    },
    {
        api: api.deleteItem,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.deleteItem({ uuid: "id", silent })),
    },
    {
        api: api.print,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.print({ uuid: "id", silent })),
    },
];

describe("warehouse thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(result);
    });
    it.each(cases)(
        "refreshes credentials and forwards signals for $api",
        async ({ api: request, argument, run }) => {
            const action = await run(makeStore().dispatch, false);
            expect(action.payload).toEqual(result);
            expect(request).toHaveBeenCalledWith(
                argument,
                "fresh",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(result);
        },
    );
    it.each(cases)(
        "honors silent mode for $api",
        async ({ api: request, run }) => {
            await run(makeStore().dispatch, true);
            expect(request).toHaveBeenCalledOnce();
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
