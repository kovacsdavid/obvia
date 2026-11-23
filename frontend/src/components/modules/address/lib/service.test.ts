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

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { select_list } from "./service";

describe("address select list service", () => {
    const fetchMock = vi.fn<typeof fetch>();

    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it("forwards the request, token and abort signal and validates the response", async () => {
        const controller = new AbortController();
        const data = { data: [{ value: "HU", title: "Hungary" }] };
        fetchMock.mockResolvedValueOnce(
            new Response(JSON.stringify(data), {
                headers: { "Content-Type": "application/json" },
            }),
        );

        expect(
            await select_list("countries", "token", controller.signal),
        ).toEqual({
            kind: "success",
            status: 200,
            data,
        });
        const [url, options] = fetchMock.mock.calls[0];
        expect(url).toBe("/api/address/select_list?list=countries");
        expect(options?.method).toBe("GET");
        expect(options?.headers).toMatchObject({
            Authorization: "Bearer token",
        });
        controller.abort();
        expect(options?.signal?.aborted).toBe(true);
    });

    it("rejects malformed success data and classifies HTTP failures", async () => {
        fetchMock
            .mockResolvedValueOnce(
                new Response(JSON.stringify({ data: [{ value: 1 }] }), {
                    headers: { "Content-Type": "application/json" },
                }),
            )
            .mockResolvedValueOnce(
                new Response(JSON.stringify({ error: { message: "Failed" } }), {
                    status: 500,
                    headers: { "Content-Type": "application/json" },
                }),
            );
        expect(await select_list("countries", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await select_list("countries", null)).toEqual({
            kind: "http_error",
            status: 500,
            body: { error: { message: "Failed" } },
        });
        expect(fetchMock.mock.calls[0][1]?.headers).not.toHaveProperty(
            "Authorization",
        );
    });
});
