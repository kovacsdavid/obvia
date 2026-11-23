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
import { beforeEach, expect, it, vi } from "vitest";
import type { RootState } from "@/store";
import * as api from "./service";
import * as inventory from "./slice";
import { notifyApiError } from "@/lib/api/notifyApiErrors";

vi.mock("./service", () => ({
    create: vi.fn(),
    update: vi.fn(),
    list: vi.fn(),
    get: vi.fn(),
    get_resolved: vi.fn(),
    deleteItem: vi.fn(),
    select_list: vi.fn(),
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
    productId: "p",
    warehouseId: "w",
    minimumStock: "",
    maximumStock: "",
    currencyCode: "HUF",
    status: "active",
};
const error = { kind: "http_error", status: 500 } as const;
function makeStore() {
    const initial = { auth: { login: { token: "old" } } } as RootState;
    return configureStore({
        reducer: (state = initial, action) =>
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
        name: "create",
        api: api.create,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.create({ input, silent })),
    },
    {
        name: "update",
        api: api.update,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.update({ input, silent })),
    },
    {
        name: "list",
        api: api.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.list({ query: "page=2", silent })),
    },
    {
        name: "get",
        api: api.get,
        argument: "i",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.get({ uuid: "i", silent })),
    },
    {
        name: "resolved",
        api: api.get_resolved,
        argument: "i",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.get_resolved({ uuid: "i", silent })),
    },
    {
        name: "delete",
        api: api.deleteItem,
        argument: "i",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.deleteItem({ uuid: "i", silent })),
    },
    {
        name: "select",
        api: api.select_list,
        argument: "products",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.select_list({ list: "products", silent })),
    },
    {
        name: "print",
        api: api.print,
        argument: "i",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(inventory.print({ uuid: "i", silent })),
    },
];

beforeEach(() => {
    vi.clearAllMocks();
    for (const testCase of cases)
        vi.mocked(testCase.api).mockResolvedValue(error);
});

it.each(cases)(
    "refreshes credentials and forwards signal for $name",
    async ({ api: method, argument, run }) => {
        const action = await run(makeStore().dispatch, false);
        expect(action.payload).toEqual(error);
        expect(method).toHaveBeenCalledExactlyOnceWith(
            argument,
            "fresh",
            expect.any(AbortSignal),
        );
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(error);
    },
);

it.each(cases)("suppresses notifications for silent $name", async ({ run }) => {
    expect((await run(makeStore().dispatch, true)).payload).toEqual(error);
    expect(notifyApiError).not.toHaveBeenCalled();
});
