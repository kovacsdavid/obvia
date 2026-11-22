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
import * as users from "./slice";

vi.mock("./service", () => ({
    create: vi.fn(),
    list: vi.fn(),
    enableOtp: vi.fn(),
    verifyOtp: vi.fn(),
    disableOtp: vi.fn(),
}));
vi.mock("@/lib/api/notifyApiErrors", () => ({ notifyApiError: vi.fn() }));
vi.mock("@/components/modules/auth/lib/slice.ts", () => ({
    refreshAccessToken:
        () => async (dispatch: (action: { type: string }) => void) => {
            dispatch({ type: "test/refreshed" });
        },
}));

const input = {
    email: "a@b.test",
    lastName: "Last",
    firstName: "First",
    phone: "123",
    status: "active",
};
const result = { kind: "http_error", status: 404 } as const;

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
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(users.create({ input, silent })),
    },
    {
        name: "list",
        api: api.list,
        argument: "page=2",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(users.list({ query: "page=2", silent })),
    },
    {
        name: "enable",
        api: api.enableOtp,
        argument: undefined,
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(users.enableOtp({ silent })),
    },
    {
        name: "verify",
        api: api.verifyOtp,
        argument: "123456",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(users.verifyOtp({ otp: "123456", silent })),
    },
    {
        name: "disable",
        api: api.disableOtp,
        argument: "123456",
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(users.disableOtp({ otp: "123456", silent })),
    },
];

describe("users thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(result);
    });

    it.each(cases)(
        "refreshes and forwards signal for $name",
        async ({ api: method, argument, run }) => {
            const action = await run(makeStore().dispatch, false);
            expect(action.payload).toEqual(result);
            const args =
                argument === undefined
                    ? ["fresh", expect.any(AbortSignal)]
                    : [argument, "fresh", expect.any(AbortSignal)];
            expect(method).toHaveBeenCalledExactlyOnceWith(...args);
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(result);
        },
    );

    it.each(cases)("allows silent $name", async ({ run }) => {
        await run(makeStore().dispatch, true);
        expect(notifyApiError).not.toHaveBeenCalled();
    });

    it("supports create and list screen arguments", async () => {
        const store = makeStore();
        await store.dispatch(users.create({ input }));
        await store.dispatch(users.list({ query: null }));
        expect(api.create).toHaveBeenCalledWith(
            input,
            "fresh",
            expect.any(AbortSignal),
        );
        expect(api.list).toHaveBeenCalledWith(
            null,
            "fresh",
            expect.any(AbortSignal),
        );
    });

    it("marks an API error as failed rather than succeeded", () => {
        const state = users.default(
            undefined,
            users.create.fulfilled(result, "request", { input }),
        );
        expect(state.status).toBe("failed");
    });
});
