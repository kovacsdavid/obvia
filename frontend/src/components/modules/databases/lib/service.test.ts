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
import * as api from "./service";

const claims = {
    sub: "user",
    exp: 2,
    iat: 1,
    nbf: 1,
    iss: "issuer",
    aud: "audience",
    jti: "id",
    family_id: null,
    active_tenant: "tenant",
};
const tokenData = { token: "new-token", claims };

describe("database API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    let controller: AbortController;

    beforeEach(() => {
        controller = new AbortController();
        fetchMock
            .mockReset()
            .mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        [
            "create",
            () => api.create({ name: "Company" }, "token", controller.signal),
            "/api/tenants/create",
            "POST",
            { name: "Company" },
        ],
        [
            "list",
            () => api.list(null, "token", controller.signal),
            "/api/tenants/list",
            "GET",
            undefined,
        ],
        [
            "filtered list",
            () => api.list("page=2&limit=10", "token", controller.signal),
            "/api/tenants/list?q=page=2&limit=10",
            "GET",
            undefined,
        ],
        [
            "get",
            () => api.get_resolved("tenant", "token", controller.signal),
            "/api/tenants/get_resolved?uuid=tenant",
            "GET",
            undefined,
        ],
        [
            "activate",
            () => api.activate(null, "token", controller.signal),
            "/api/tenants/activate",
            "POST",
            { uuid: null },
        ],
        [
            "delete",
            () => api.deleteDatabase("tenant", "token", controller.signal),
            "/api/tenants/delete",
            "POST",
            { uuid: "tenant" },
        ],
    ])(
        "sends %s with its original contract and signal",
        async (_name, invoke, url, method, body) => {
            await invoke();
            const [actualUrl, options] = fetchMock.mock.calls[0];
            expect(actualUrl).toBe(url);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            expect(options?.body).toBe(
                body === undefined ? undefined : JSON.stringify(body),
            );
            expect(options?.signal?.aborted).toBe(false);
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
        },
    );

    it("accepts the exact backend validation fields and rejects malformed fields", async () => {
        const body = {
            error: { message: "Invalid", fields: { name: "Required" } },
        };
        fetchMock.mockResolvedValueOnce(Response.json(body, { status: 422 }));
        expect(await api.create({ name: "" }, null)).toEqual({
            kind: "http_error",
            status: 422,
            body,
        });
        fetchMock.mockResolvedValueOnce(
            Response.json(
                { error: { message: "Invalid", fields: { name: 7 } } },
                { status: 422 },
            ),
        );
        expect(await api.create({ name: "" }, null)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it.each([api.activate, api.deleteDatabase])(
        "validates token-bearing responses",
        async (invoke) => {
            fetchMock.mockResolvedValueOnce(Response.json({ data: tokenData }));
            expect(await invoke("tenant", "token")).toEqual({
                kind: "success",
                status: 200,
                data: { data: tokenData },
            });
            fetchMock.mockResolvedValueOnce(
                Response.json({ data: { token: "new-token", claims: {} } }),
            );
            expect(await invoke("tenant", "token")).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        },
    );

    it("classifies rate limits and transport failures", async () => {
        fetchMock.mockResolvedValueOnce(
            new Response(null, {
                status: 429,
                headers: { "Retry-After": "5" },
            }),
        );
        expect(await api.list(null, null)).toEqual({
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: 5,
        });
        fetchMock.mockRejectedValueOnce(new TypeError("network"));
        expect(await api.list(null, null)).toMatchObject({
            kind: "network_error",
        });
    });
});
