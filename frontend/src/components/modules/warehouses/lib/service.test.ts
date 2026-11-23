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
import type { WarehouseUserInput } from "./interface";

const input: WarehouseUserInput = {
    id: "warehouse-1",
    name: "Main",
    contactName: "Contact",
    contactPhone: "123",
    status: "active",
};
const fields = {
    id: null,
    name: "Required",
    contact_name: null,
    contact_phone: null,
    status: null,
};

describe("warehouse API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    let controller: AbortController;

    beforeEach(() => {
        controller = new AbortController();
        fetchMock.mockReset();
        fetchMock.mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        [
            "create",
            () => api.create(input, "token", controller.signal),
            "/api/warehouses/create",
            "POST",
        ],
        [
            "update",
            () => api.update(input, "token", controller.signal),
            "/api/warehouses/update",
            "PUT",
        ],
        [
            "list",
            () => api.list(null, "token", controller.signal),
            "/api/warehouses/list",
            "GET",
        ],
        [
            "filtered list",
            () => api.list("page=2&name=main", "token", controller.signal),
            "/api/warehouses/list?q=page=2&name=main",
            "GET",
        ],
        [
            "get",
            () => api.get("warehouse-1", "token", controller.signal),
            "/api/warehouses/get?uuid=warehouse-1",
            "GET",
        ],
        [
            "resolved",
            () => api.get_resolved("warehouse-1", "token", controller.signal),
            "/api/warehouses/get_resolved?uuid=warehouse-1",
            "GET",
        ],
        [
            "delete",
            () => api.deleteItem("warehouse-1", "token", controller.signal),
            "/api/warehouses/delete?uuid=warehouse-1",
            "DELETE",
        ],
        [
            "print",
            () => api.print("warehouse-1", "token", controller.signal),
            "/api/warehouses/print?uuid=warehouse-1",
            "GET",
        ],
    ])(
        "sends %s with authentication and cancellation",
        async (_name, invoke, url, method) => {
            await invoke();
            const [actualUrl, options] = fetchMock.mock.calls[0];
            expect(actualUrl).toBe(url);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
        },
    );

    it.each([
        ["create", api.create],
        ["update", api.update],
    ])("maps %s body and parses validation errors", async (_name, invoke) => {
        fetchMock.mockResolvedValue(
            new Response(
                JSON.stringify({ error: { message: "Invalid", fields } }),
                {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                },
            ),
        );
        const result = await invoke(input, null);
        expect(JSON.parse(fetchMock.mock.calls[0][1]?.body as string)).toEqual({
            id: input.id,
            name: input.name,
            contact_name: input.contactName,
            contact_phone: input.contactPhone,
            status: input.status,
        });
        expect(fetchMock.mock.calls[0][1]?.headers).not.toHaveProperty(
            "Authorization",
        );
        expect(result).toEqual({
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid", fields } },
        });
    });

    it("returns PDF bytes and rejects malformed successes", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const pdf = new Response(bytes, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(pdf, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValueOnce(pdf);
        expect(await api.print("warehouse-1", null)).toEqual({
            kind: "success",
            status: 200,
            data: new Uint8Array([37, 80, 68, 70]),
        });
        fetchMock.mockResolvedValueOnce(
            new Response(JSON.stringify({ data: {} }), {
                headers: { "Content-Type": "application/json" },
            }),
        );
        expect(await api.get("warehouse-1", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });
});
