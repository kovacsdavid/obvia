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

import {
    cleanup,
    fireEvent,
    render,
    screen,
    waitFor,
} from "@testing-library/react";
import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import ActivityFeed from "./ActivityFeed";

const mocks = vi.hoisted(() => ({
    dispatch: vi.fn(),
    list: vi.fn(),
    post: vi.fn(),
    remove: vi.fn(),
    abort: vi.fn(),
    setPage: vi.fn(),
    setLimit: vi.fn(),
    setTotal: vi.fn(),
}));

vi.mock("@/store/hooks.ts", () => ({ useAppDispatch: () => mocks.dispatch }));
vi.mock("@/hooks/use_auth", () => ({ useAuth: () => ({ claims: null }) }));
vi.mock("@/hooks/use_data_display_common.ts", () => ({
    useDataDisplayCommon: () => ({
        setPage: mocks.setPage,
        setLimit: mocks.setLimit,
        setTotal: mocks.setTotal,
    }),
}));
vi.mock("@/components/modules/activity_feed/lib/slice.ts", () => {
    const thunk = (kind: string) =>
        Object.assign((payload: unknown) => ({ kind, payload }), {
            fulfilled: {
                match: (action: { type: string }) =>
                    action.type === `${kind}/fulfilled`,
            },
        });
    return {
        list: thunk("list"),
        postComment: thunk("post"),
        deleteComment: thunk("remove"),
    };
});

const entry = {
    id: "comment-id",
    resource_id: "resource-id",
    resource_type: "worksheets",
    activity_type: "comment",
    content: "Existing comment",
    created_by_id: "user-id",
    created_by: "Anna",
    created_at: "2026-01-01T00:00:00Z",
    updated_at: "2026-01-01T00:00:00Z",
    deleted_at: null,
};

describe("activity feed", () => {
    beforeEach(() => {
        vi.clearAllMocks();
        mocks.list.mockResolvedValue({
            kind: "success",
            data: { meta: { page: 2, limit: 10, total: 31 }, data: [entry] },
        });
        mocks.post.mockResolvedValue({ kind: "success", data: {} });
        mocks.remove.mockResolvedValue({ kind: "success", data: {} });
        mocks.dispatch.mockImplementation(
            (action: {
                kind: "list" | "post" | "remove";
                payload: unknown;
            }) => {
                const promise = mocks[action.kind](action.payload).then(
                    (payload: unknown) => ({
                        type: `${action.kind}/fulfilled`,
                        payload,
                    }),
                );
                return Object.assign(promise, { abort: mocks.abort });
            },
        );
    });
    afterEach(cleanup);

    it("loads entries, preserves pagination metadata and aborts on unmount", async () => {
        const view = render(
            <ActivityFeed resourceId="resource-id" resourceType="worksheets" />,
        );
        expect(await screen.findByText("Existing comment")).toBeInTheDocument();
        expect(mocks.list).toHaveBeenCalledWith({
            resourceId: "resource-id",
            resourceType: "worksheets",
        });
        expect(mocks.setPage).toHaveBeenCalledWith(2);
        expect(mocks.setLimit).toHaveBeenCalledWith(10);
        expect(mocks.setTotal).toHaveBeenCalledWith(31);
        view.unmount();
        expect(mocks.abort).toHaveBeenCalledOnce();
    });

    it("shows validation errors and retains the comment without refreshing", async () => {
        mocks.post.mockResolvedValue({
            kind: "http_error",
            status: 422,
            body: {
                error: {
                    message: "Invalid",
                    fields: {
                        comment: "Required",
                        commentable_id: "Bad resource",
                    },
                },
            },
        });
        render(
            <ActivityFeed resourceId="resource-id" resourceType="worksheets" />,
        );
        await screen.findByText("Existing comment");
        const input = screen.getByLabelText("Megjegyzés");
        fireEvent.change(input, { target: { value: "draft" } });
        fireEvent.click(screen.getByRole("button", { name: "Küldés" }));
        expect(await screen.findByText("Required")).toBeInTheDocument();
        expect(screen.getByText("Bad resource")).toBeInTheDocument();
        expect(input).toHaveValue("draft");
        expect(input).toHaveAttribute("aria-invalid", "true");
        expect(mocks.list).toHaveBeenCalledOnce();
    });

    it("clears the comment and refreshes after posting", async () => {
        render(
            <ActivityFeed resourceId="resource-id" resourceType="worksheets" />,
        );
        await screen.findByText("Existing comment");
        const input = screen.getByLabelText("Megjegyzés");
        fireEvent.change(input, { target: { value: "New comment" } });
        fireEvent.click(screen.getByRole("button", { name: "Küldés" }));
        await waitFor(() => expect(mocks.list).toHaveBeenCalledTimes(2));
        expect(mocks.post).toHaveBeenCalledWith({
            resourceId: "resource-id",
            resourceType: "worksheets",
            comment: "New comment",
        });
        expect(input).toHaveValue("");
    });

    it.each([true, false])(
        "refreshes after deletion only on success: %s",
        async (success) => {
            if (!success)
                mocks.remove.mockResolvedValue({
                    kind: "network_error",
                    offline: false,
                });
            render(
                <ActivityFeed
                    resourceId="resource-id"
                    resourceType="worksheets"
                />,
            );
            await screen.findByText("Existing comment");
            fireEvent.pointerDown(
                screen.getByRole("button", { name: "Menü megnyitása" }),
                { button: 0, ctrlKey: false },
            );
            fireEvent.click(
                await screen.findByRole("menuitem", { name: "Törlés" }),
            );
            await waitFor(() =>
                expect(mocks.remove).toHaveBeenCalledWith({
                    uuid: "comment-id",
                }),
            );
            await waitFor(() =>
                expect(mocks.list).toHaveBeenCalledTimes(success ? 2 : 1),
            );
        },
    );
});
