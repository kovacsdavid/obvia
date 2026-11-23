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

import { afterEach, describe, expect, it, vi } from "vitest";
import {
    isActivityFeedEntry,
    isCommentError,
    isDeleteCommentResponse,
    isPaginatedActivityFeedResponse,
} from "./guards";
import { delete_comment, list, post_comment } from "./service";

afterEach(() => vi.unstubAllGlobals());

const entry = {
    id: "id",
    resource_id: "resource",
    resource_type: "worksheets",
    activity_type: "comment",
    content: "Hello",
    created_by_id: "user",
    created_by: "User",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
};

describe("activity feed schemas", () => {
    it("validates payloads and paginated envelopes", () => {
        const data = {
            meta: { page: 1, limit: 25, total: 1 },
            data: [{ ...entry, extra: true }],
        };
        expect(isActivityFeedEntry(data.data[0])).toBe(true);
        expect(isActivityFeedEntry({ ...entry, deleted_at: 1 })).toBe(false);
        expect(isPaginatedActivityFeedResponse(data)).toBe(true);
        expect(isPaginatedActivityFeedResponse({ data: [] })).toBe(false);
        expect(isPaginatedActivityFeedResponse({ ...data, meta: null })).toBe(
            false,
        );
        expect(isPaginatedActivityFeedResponse({ ...data, data: null })).toBe(
            false,
        );
        expect(
            isPaginatedActivityFeedResponse({
                ...data,
                error: { message: "Error" },
            }),
        ).toBe(false);
        expect(isDeleteCommentResponse({ data: { message: "Deleted" } })).toBe(
            true,
        );
        expect(isDeleteCommentResponse({ data: {} })).toBe(false);
    });

    it("accepts extensible string-or-null validation fields", () => {
        expect(isCommentError({})).toBe(true);
        expect(isCommentError({ comment: "Required", extra: null })).toBe(true);
        expect(isCommentError({ comment: 123 })).toBe(false);
        expect(isCommentError({ extra: 123 })).toBe(false);
    });

    it("returns the original list payload including extra fields", async () => {
        const body = {
            meta: { page: 1, limit: 25, total: 1 },
            data: [{ ...entry, extra: true }],
            trace: "request",
        };
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                new Response(JSON.stringify(body), {
                    headers: { "Content-Type": "application/json" },
                }),
            ),
        );
        expect(await list("resource", "worksheets", null)).toEqual({
            kind: "success",
            status: 200,
            data: body,
        });
    });

    it.each([
        () => list("id", "worksheets", null),
        () => post_comment("id", "worksheets", "text", null),
        () => delete_comment("id", null),
    ])("rejects non-JSON success responses", async (operation) => {
        const pdf = new Response("pdf", {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(pdf, "bytes", {
            value: async () => new Uint8Array([1, 2]),
        });
        vi.stubGlobal(
            "fetch",
            vi
                .fn()
                .mockResolvedValueOnce(new Response(null, { status: 204 }))
                .mockResolvedValueOnce(
                    new Response("text", {
                        headers: { "Content-Type": "text/plain" },
                    }),
                )
                .mockResolvedValueOnce(pdf),
        );
        expect(await operation()).toEqual({
            kind: "invalid_response",
            status: 204,
        });
        expect(await operation()).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await operation()).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });
});
