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
import * as api from "./service";
import * as thunks from "./slice";

vi.mock("./service", () => ({
    create: vi.fn(),
    list: vi.fn(),
    activate: vi.fn(),
    get_resolved: vi.fn(),
    deleteDatabase: vi.fn(),
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
const input = { name: "Company" };
const result = { kind: "http_error", status: 500 } as const;
const cases = [
    {
        name: "create",
        api: api.create,
        argument: input,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.create({ input, silent })),
    },
    {
        name: "list",
        api: api.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.list({ query: "page=2", silent })),
    },
    {
        name: "activate",
        api: api.activate,
        argument: null,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.activate({ uuid: null, silent })),
    },
    {
        name: "get",
        api: api.get_resolved,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.get({ uuid: "id", silent })),
    },
    {
        name: "delete",
        api: api.deleteDatabase,
        argument: "id",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(thunks.deleteDatabase({ uuid: "id", silent })),
    },
];

describe("database thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(result);
    });
    it.each(cases)(
        "refreshes and forwards the token and signal for $name",
        async ({ api: operation, argument, run }) => {
            const action = await run(makeStore().dispatch, false);
            expect(action.payload).toEqual(result);
            expect(operation).toHaveBeenCalledExactlyOnceWith(
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
            await run(makeStore().dispatch, true);
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
