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

import { afterEach, expect, it, vi } from "vitest";
import * as api from "./service";
import { isInventoryError } from "./guards";

afterEach(() => vi.unstubAllGlobals());

it("preserves create input and backend field errors", async () => {
    const fields = {
        id: null,
        product_id: "Invalid product",
        warehouse_id: null,
        minimum_stock: null,
        maximum_stock: null,
        currency_code: null,
        status: null,
    };
    const fetchMock = vi.fn().mockResolvedValue(
        new Response(
            JSON.stringify({ error: { message: "Invalid input", fields } }),
            {
                status: 422,
                headers: { "Content-Type": "application/json" },
            },
        ),
    );
    vi.stubGlobal("fetch", fetchMock);
    const result = await api.create(
        {
            id: null,
            product_id: "p",
            warehouse_id: "w",
            minimum_stock: "1",
            maximum_stock: "9",
            currency_code: "EUR",
            status: "active",
        },
        "token",
    );
    expect(JSON.parse(fetchMock.mock.calls[0][1].body)).toEqual({
        id: null,
        product_id: "p",
        warehouse_id: "w",
        minimum_stock: "1",
        maximum_stock: "9",
        currency_code: "EUR",
        status: "active",
    });
    expect(result).toEqual({
        kind: "http_error",
        status: 422,
        body: { error: { message: "Invalid input", fields } },
    });
});

const input = {
    id: null,
    product_id: "p",
    warehouse_id: "w",
    minimum_stock: "1",
    maximum_stock: "9",
    currency_code: "EUR",
    status: "active",
};
const fields = {
    id: null,
    product_id: null,
    warehouse_id: null,
    minimum_stock: null,
    maximum_stock: null,
    currency_code: null,
    status: null,
};

it("keeps response data intact and rejects 204 for JSON and PDF endpoints", async () => {
    const data = {
        id: "i",
        product_id: "p",
        warehouse_id: "w",
        quantity_on_hand: "2",
        quantity_reserved: "1",
        quantity_available: "1",
        minimum_stock: null,
        maximum_stock: null,
        currency_code: "EUR",
        status: "active",
        created_by_id: "u",
        created_at: "date",
        updated_at: "date",
        deleted_at: null,
        extra: "preserved",
    };
    const body = { data, extra: true };
    vi.stubGlobal(
        "fetch",
        vi
            .fn()
            .mockResolvedValueOnce(
                new Response(JSON.stringify(body), {
                    headers: { "Content-Type": "application/json" },
                }),
            )
            .mockImplementation(() =>
                Promise.resolve(new Response(null, { status: 204 })),
            ),
    );
    expect(await api.get("i", null)).toEqual({
        kind: "success",
        status: 200,
        data: body,
    });
    expect(await api.get("i", null)).toEqual({
        kind: "invalid_response",
        status: 204,
    });
    expect(await api.deleteItem("i", null)).toEqual({
        kind: "invalid_response",
        status: 204,
    });
    expect(await api.print("i", null)).toEqual({
        kind: "invalid_response",
        status: 204,
    });
});

it.each([
    [
        "create",
        (signal: AbortSignal) => api.create(input, "token", signal),
        "/api/inventory/create",
        "POST",
    ],
    [
        "update",
        (signal: AbortSignal) => api.update(input, "token", signal),
        "/api/inventory/update",
        "PUT",
    ],
    [
        "list",
        (signal: AbortSignal) => api.list(null, "token", signal),
        "/api/inventory/list",
        "GET",
    ],
    [
        "filtered list",
        (signal: AbortSignal) => api.list("page=2&x=y", "token", signal),
        "/api/inventory/list?q=page=2&x=y",
        "GET",
    ],
    [
        "select",
        (signal: AbortSignal) => api.select_list("products", "token", signal),
        "/api/inventory/select_list?list=products",
        "GET",
    ],
    [
        "get",
        (signal: AbortSignal) => api.get("i", "token", signal),
        "/api/inventory/get?uuid=i",
        "GET",
    ],
    [
        "resolved",
        (signal: AbortSignal) => api.get_resolved("i", "token", signal),
        "/api/inventory/get_resolved?uuid=i",
        "GET",
    ],
    [
        "delete",
        (signal: AbortSignal) => api.deleteItem("i", "token", signal),
        "/api/inventory/delete?uuid=i",
        "DELETE",
    ],
    [
        "print",
        (signal: AbortSignal) => api.print("i", "token", signal),
        "/api/inventory/print?uuid=i",
        "GET",
    ],
])(
    "preserves %s endpoint and forwards cancellation",
    async (_name, invoke, url, method) => {
        const fetchMock = vi
            .fn()
            .mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
        const controller = new AbortController();
        await invoke(controller.signal);
        const [actualUrl, options] = fetchMock.mock.calls[0];
        expect(actualUrl).toBe(url);
        expect(options.method).toBe(method);
        expect(options.headers.Authorization).toBe("Bearer token");
        controller.abort();
        expect(options.signal.aborted).toBe(true);
        if (method === "PUT")
            expect(JSON.parse(options.body)).toEqual({
                id: null,
                product_id: "p",
                warehouse_id: "w",
                minimum_stock: "1",
                maximum_stock: "9",
                currency_code: "EUR",
                status: "active",
            });
    },
);

it.each(["create", "update"] as const)(
    "validates %s backend errors",
    async (operation) => {
        const fetchMock = vi.fn();
        vi.stubGlobal("fetch", fetchMock);
        for (const [validation, valid] of [
            [fields, true],
            [{ ...fields, status: 42 }, false],
        ] as const) {
            const body = { error: { message: "Invalid", fields: validation } };
            fetchMock.mockResolvedValueOnce(
                new Response(JSON.stringify(body), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await api[operation](input, null)).toEqual({
                kind: "http_error",
                status: 422,
                body: valid ? body : undefined,
            });
        }
    },
);

it("checks every DTO error field", () => {
    expect(isInventoryError(fields)).toBe(true);
    for (const key of Object.keys(fields)) {
        const missing: Record<string, unknown> = { ...fields };
        delete missing[key];
        expect(isInventoryError(missing)).toBe(false);
        expect(isInventoryError({ ...fields, [key]: 1 })).toBe(false);
        expect(isInventoryError({ ...fields, [key]: "Invalid" })).toBe(true);
    }
});

it("accepts PDF bytes and rejects a non-PDF response", async () => {
    const bytes = new Uint8Array([37, 80, 68, 70]);
    const response = new Response(bytes, {
        headers: { "Content-Type": "application/pdf" },
    });
    Object.defineProperty(response, "bytes", {
        value: vi.fn().mockResolvedValue(bytes),
    });
    const fetchMock = vi
        .fn()
        .mockResolvedValueOnce(response)
        .mockResolvedValueOnce(
            new Response("bad", { headers: { "Content-Type": "text/plain" } }),
        );
    vi.stubGlobal("fetch", fetchMock);
    expect(await api.print("i", null)).toEqual({
        kind: "success",
        status: 200,
        data: bytes,
    });
    expect(await api.print("i", null)).toEqual({
        kind: "invalid_response",
        status: 200,
    });
});
