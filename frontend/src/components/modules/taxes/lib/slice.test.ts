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
import { notifyApiError } from "@/lib/api/notifyApiErrors";
import type { TaxUserInput } from "./interface";
import * as api from "./service";
import * as taxes from "./slice";

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

const input: TaxUserInput = {
    id: null,
    rate: "27",
    description: "VAT",
    country_code: "HU",
    tax_category: "AK",
    is_rate_applicable: true,
    legal_text: "",
    reporting_code: "",
    is_default: false,
    status: "active",
};
const result = { kind: "http_error", status: 500 } as const;
function makeStore() {
    const initialState = { auth: { login: { token: "old" } } } as RootState;
    return configureStore({
        reducer: (state = initialState, action) =>
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
        name: "create",
        api: api.create,
        argument: input,
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.create({ input, silent })),
    },
    {
        name: "update",
        api: api.update,
        argument: input,
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.update({ input, silent })),
    },
    {
        name: "list",
        api: api.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.list({ query: "page=2", silent })),
    },
    {
        name: "get",
        api: api.get,
        argument: "tax-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.get({ uuid: "tax-1", silent })),
    },
    {
        name: "get_resolved",
        api: api.get_resolved,
        argument: "tax-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.get_resolved({ uuid: "tax-1", silent })),
    },
    {
        name: "delete",
        api: api.deleteItem,
        argument: "tax-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.deleteItem({ uuid: "tax-1", silent })),
    },
    {
        name: "select",
        api: api.select_list,
        argument: "countries",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.select_list({ list: "countries", silent })),
    },
    {
        name: "print",
        api: api.print,
        argument: "tax-1",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(taxes.print({ uuid: "tax-1", silent })),
    },
];

describe("tax thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(result);
    });
    it.each(cases)(
        "refreshes and forwards the token and signal for $name",
        async ({ api: request, argument, run }) => {
            const action = await run(makeStore().dispatch);
            expect(action.payload).toEqual(result);
            expect(request).toHaveBeenCalledExactlyOnceWith(
                argument,
                "fresh",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(result);
        },
    );
    it.each(cases)(
        "suppresses notifications for silent $name",
        async ({ run }) => {
            expect((await run(makeStore().dispatch, true)).payload).toEqual(
                result,
            );
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
