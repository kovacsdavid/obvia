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
import * as feed from "./slice";

vi.mock("./service", () => ({
    list: vi.fn(),
    post_comment: vi.fn(),
    delete_comment: vi.fn(),
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
const failure = { kind: "http_error", status: 500 } as const;
const cases = [
    {
        name: "list",
        api: api.list,
        args: ["id", "worksheets"],
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(
                feed.list({
                    resourceId: "id",
                    resourceType: "worksheets",
                    silent,
                }),
            ),
    },
    {
        name: "post",
        api: api.post_comment,
        args: ["id", "worksheets", "text"],
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(
                feed.postComment({
                    resourceId: "id",
                    resourceType: "worksheets",
                    comment: "text",
                    silent,
                }),
            ),
    },
    {
        name: "delete",
        api: api.delete_comment,
        args: ["comment-id"],
        run: (dispatch: Dispatch, silent: boolean) =>
            dispatch(feed.deleteComment({ uuid: "comment-id", silent })),
    },
];

describe("activity feed thunks", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        for (const testCase of cases)
            vi.mocked(testCase.api).mockResolvedValue(failure);
    });

    it.each(cases)(
        "refreshes credentials and forwards a signal for $name",
        async ({ api: request, args, run }) => {
            const action = await run(makeStore().dispatch, false);
            expect(action.payload).toEqual(failure);
            expect(request).toHaveBeenCalledExactlyOnceWith(
                ...args,
                "fresh",
                expect.any(AbortSignal),
            );
            expect(notifyApiError).toHaveBeenCalledExactlyOnceWith(failure);
        },
    );

    it.each(cases)("does not notify for silent $name", async ({ run }) => {
        await run(makeStore().dispatch, true);
        expect(notifyApiError).not.toHaveBeenCalled();
    });
});
