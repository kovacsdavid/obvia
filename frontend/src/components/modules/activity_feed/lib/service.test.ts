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
import { delete_comment, list, post_comment } from "./service";

afterEach(() => vi.unstubAllGlobals());

describe("post_comment", () => {
    it("accepts the backend comment record and sends the expected request", async () => {
        const comment = {
            id: "comment-id",
            commentable_type: "worksheets",
            commentable_id: "resource-id",
            comment: "Hello",
            created_by_id: "user-id",
            created_at: "2026-01-01",
            updated_at: "2026-01-01",
            deleted_at: null,
        };
        const fetchMock = vi.fn().mockResolvedValue(
            new Response(JSON.stringify({ meta: null, data: comment }), {
                status: 201,
                headers: { "Content-Type": "application/json" },
            }),
        );
        vi.stubGlobal("fetch", fetchMock);
        const result = await post_comment(
            "resource-id",
            "worksheets",
            "Hello",
            "token",
        );
        expect(result).toMatchObject({
            kind: "success",
            status: 201,
            data: { data: comment },
        });
        expect(fetchMock).toHaveBeenCalledWith(
            "/api/comments/post",
            expect.objectContaining({
                method: "POST",
                body: JSON.stringify({
                    commentable_id: "resource-id",
                    commentable_type: "worksheets",
                    comment: "Hello",
                }),
                headers: expect.objectContaining({
                    Authorization: "Bearer token",
                }),
            }),
        );
    });

    it("preserves backend validation fields", async () => {
        const fields = {
            id: null,
            commentable_type: null,
            commentable_id: null,
            comment: "Required",
        };
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(
                new Response(
                    JSON.stringify({ error: { message: "Invalid", fields } }),
                    {
                        status: 422,
                        headers: { "Content-Type": "application/json" },
                    },
                ),
            ),
        );
        expect(await post_comment("id", "worksheets", "", null)).toMatchObject({
            kind: "http_error",
            status: 422,
            body: { error: { fields } },
        });
    });
});

describe("activity feed requests", () => {
    it("loads paginated entries and encodes resource parameters", async () => {
        const data = { meta: { page: 2, limit: 10, total: 15 }, data: [] };
        const fetchMock = vi.fn().mockResolvedValue(
            new Response(JSON.stringify(data), {
                status: 200,
                headers: { "Content-Type": "application/json" },
            }),
        );
        vi.stubGlobal("fetch", fetchMock);
        expect(await list("id & more", "worksheets", "token")).toMatchObject({
            kind: "success",
            data,
        });
        expect(fetchMock).toHaveBeenCalledWith(
            "/api/activity_feed/list?resource_id=id+%26+more&resource_type=worksheets",
            expect.objectContaining({ method: "GET" }),
        );
    });

    it("deletes a comment and validates the response", async () => {
        const fetchMock = vi.fn().mockResolvedValue(
            new Response(
                JSON.stringify({ meta: null, data: { message: "Deleted" } }),
                {
                    status: 200,
                    headers: { "Content-Type": "application/json" },
                },
            ),
        );
        vi.stubGlobal("fetch", fetchMock);
        expect(await delete_comment("id & more", null)).toMatchObject({
            kind: "success",
            status: 200,
        });
        expect(fetchMock).toHaveBeenCalledWith(
            "/api/comments/delete?uuid=id+%26+more",
            expect.objectContaining({ method: "DELETE" }),
        );
    });

    it("rejects malformed success and validation responses", async () => {
        const fetchMock = vi
            .fn()
            .mockResolvedValueOnce(
                new Response(JSON.stringify({ data: { id: "incomplete" } }), {
                    status: 201,
                    headers: { "Content-Type": "application/json" },
                }),
            )
            .mockResolvedValueOnce(
                new Response(
                    JSON.stringify({
                        error: { message: "Invalid", fields: { comment: 123 } },
                    }),
                    {
                        status: 422,
                        headers: { "Content-Type": "application/json" },
                    },
                ),
            );
        vi.stubGlobal("fetch", fetchMock);
        expect(await post_comment("id", "worksheets", "text", null)).toEqual({
            kind: "invalid_response",
            status: 201,
        });
        expect(await post_comment("id", "worksheets", "text", null)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it("rejects successful responses without the required payload", async () => {
        vi.stubGlobal(
            "fetch",
            vi.fn().mockImplementation(() =>
                Promise.resolve(
                    new Response(JSON.stringify({ meta: null }), {
                        status: 200,
                        headers: { "Content-Type": "application/json" },
                    }),
                ),
            ),
        );
        expect(await list("id", "worksheets", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await post_comment("id", "worksheets", "text", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await delete_comment("id", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it("propagates caller cancellation", async () => {
        const controller = new AbortController();
        vi.stubGlobal(
            "fetch",
            vi.fn(
                (_url: string, options: RequestInit) =>
                    new Promise((_resolve, reject) => {
                        options.signal?.addEventListener("abort", () =>
                            reject(new DOMException("Aborted", "AbortError")),
                        );
                    }),
            ),
        );
        const request = list("id", "worksheets", null, controller.signal);
        controller.abort();
        expect(await request).toEqual({ kind: "aborted" });
    });
});
