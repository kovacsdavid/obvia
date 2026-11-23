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
import * as productsApi from "./service";
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import * as products from "./slice";

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
                dispatch({ type: "test/refreshed" });
                return { kind: "success" as const };
            },
        ),
    };
});

const input = {
    id: null,
    name: "Product",
    description: "",
    unitOfMeasureId: "unit",
    newUnitOfMeasure: "",
    status: "active",
};
const result = { kind: "http_error", status: 500 } as const;
function makeStore() {
    const initial = { auth: { login: { token: "old" } } } as RootState;
    return configureStore({
        reducer: (state = initial, action) =>
            action.type === "test/refreshed"
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
        api: productsApi.create,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.create({ input, silent })),
    },
    {
        api: productsApi.update,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.update({ input, silent })),
    },
    {
        api: productsApi.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.list({ query: "page=2", silent })),
    },
    {
        api: productsApi.get,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.get({ uuid: "id", silent })),
    },
    {
        api: productsApi.get_resolved,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.get_resolved({ uuid: "id", silent })),
    },
    {
        api: productsApi.deleteItem,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.deleteItem({ uuid: "id", silent })),
    },
    {
        api: productsApi.select_list,
        argument: "units_of_measure",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(
                products.select_list({ list: "units_of_measure", silent }),
            ),
    },
    {
        api: productsApi.print,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(products.print({ uuid: "id", silent })),
    },
];
describe("product thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        cases.forEach(({ api }) => vi.mocked(api).mockResolvedValue(result));
    });
    it.each(cases)(
        "refreshes and forwards token and signal for $argument",
        async ({ api, argument, run }) => {
            expect((await run(makeStore().dispatch, false)).payload).toEqual(
                result,
            );
            expect(api).toHaveBeenCalledWith(
                argument,
                "fresh",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).toHaveBeenCalledWith(result);
        },
    );
    it.each(cases)(
        "supports silent requests for $argument",
        async ({ api, run }) => {
            await run(makeStore().dispatch, true);
            expect(api).toHaveBeenCalledOnce();
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
