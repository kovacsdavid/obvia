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
import * as reservations from "./slice";
import type { InventoryReservationUserInput } from "./interface";

vi.mock("./service", () => ({
    create: vi.fn(),
    list: vi.fn(),
    get: vi.fn(),
    get_resolved: vi.fn(),
    deleteItem: vi.fn(),
    select_list: vi.fn(),
    print: vi.fn(),
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));
vi.mock("@/components/modules/auth/lib/slice.ts", () => ({
    refreshAccessToken:
        () => async (dispatch: (action: { type: string }) => void) => {
            dispatch({ type: "test/refreshed" });
        },
}));

const input: InventoryReservationUserInput = {
    id: null,
    inventoryId: "inv",
    quantity: "3",
    referenceType: "",
    referenceId: "",
    reservedUntil: "",
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
        api: api.create,
        argument: input,
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.create({ input, silent })),
    },
    {
        api: api.list,
        argument: { inventoryId: "inv", query: "page=2" },
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(
                reservations.list({
                    inventoryId: "inv",
                    query: "page=2",
                    silent,
                }),
            ),
    },
    {
        api: api.get,
        argument: "id",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.get({ uuid: "id", silent })),
    },
    {
        api: api.get_resolved,
        argument: "id",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.get_resolved({ uuid: "id", silent })),
    },
    {
        api: api.deleteItem,
        argument: "id",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.deleteItem({ uuid: "id", silent })),
    },
    {
        api: api.select_list,
        argument: "inventory",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.select_list({ list: "inventory", silent })),
    },
    {
        api: api.print,
        argument: "id",
        run: (dispatch: Dispatch, silent?: boolean) =>
            dispatch(reservations.print({ uuid: "id", silent })),
    },
];

describe("reservation thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(result);
    });
    it.each(cases)(
        "refreshes and forwards signal, token and notification",
        async ({ api: operation, argument, run }) => {
            const action = await run(makeStore().dispatch);
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
        "suppresses notifications in silent mode",
        async ({ api: operation, argument, run }) => {
            await run(makeStore().dispatch, true);
            expect(operation).toHaveBeenCalledExactlyOnceWith(
                argument,
                "fresh",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).not.toHaveBeenCalled();
        },
    );
});
