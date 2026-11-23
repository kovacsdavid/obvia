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
import { select_list } from "./slice";

vi.mock("./service", () => ({ select_list: vi.fn() }));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));
vi.mock("@/components/modules/auth/lib/slice", async () => {
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

describe("address select list thunk", () => {
    beforeEach(() => vi.clearAllMocks());

    it("refreshes the token, forwards the signal and notifies errors", async () => {
        const result = { kind: "http_error", status: 500 } as const;
        vi.mocked(api.select_list).mockResolvedValue(result);
        const controller = new AbortController();
        const action = await makeStore().dispatch(
            select_list({ list: "countries" }, { signal: controller.signal }),
        );
        expect(action.payload).toEqual(result);
        expect(api.select_list).toHaveBeenCalledExactlyOnceWith(
            "countries",
            "fresh",
            expect.any(AbortSignal),
        );
        expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(result);
        controller.abort();
        expect(vi.mocked(api.select_list).mock.calls[0][2]?.aborted).toBe(true);
    });

    it("supports silent notifications and rejected requests", async () => {
        const result = { kind: "timeout" } as const;
        vi.mocked(api.select_list)
            .mockResolvedValueOnce(result)
            .mockRejectedValueOnce(new Error("failure"));
        const store = makeStore();
        expect(
            await store
                .dispatch(select_list({ list: "countries", silent: true }))
                .unwrap(),
        ).toEqual(result);
        expect(notifyApiError).not.toHaveBeenCalled();
        await expect(
            store.dispatch(select_list({ list: "countries" })).unwrap(),
        ).rejects.toMatchObject({ message: "failure" });
    });
});
