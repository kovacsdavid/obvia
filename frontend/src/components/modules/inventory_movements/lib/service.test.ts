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
import type { InventoryMovementUserInput } from "./interface";

const input: InventoryMovementUserInput = {
    id: null,
    inventoryId: "inventory-1",
    movementType: "out",
    quantity: "2",
    referenceType: "worksheets",
    referenceId: "reference-1",
    unitPrice: "10",
    totalPrice: "20",
    taxId: "tax-1",
};
const fields = {
    id: null,
    inventory_id: null,
    movement_type: null,
    quantity: "Required",
    reference_type: null,
    reference_id: null,
    unit_price: null,
    total_price: null,
    tax_id: null,
};

describe("inventory movement API", () => {
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
            () => api.create(input, "token", controller.signal),
            "/api/inventory_movements/create",
            "POST",
        ],
        [
            "list",
            () =>
                api.list(
                    { inventoryId: "inventory-1", query: null },
                    "token",
                    controller.signal,
                ),
            "/api/inventory_movements/list?inventory_id=inventory-1",
            "GET",
        ],
        [
            "filtered list",
            () =>
                api.list(
                    {
                        inventoryId: "inventory-1",
                        query: "page=2&sort=quantity",
                    },
                    "token",
                    controller.signal,
                ),
            "/api/inventory_movements/list?inventory_id=inventory-1&q=page=2&sort=quantity",
            "GET",
        ],
        [
            "get",
            () => api.get("movement-1", "token", controller.signal),
            "/api/inventory_movements/get?uuid=movement-1",
            "GET",
        ],
        [
            "get resolved",
            () => api.get_resolved("movement-1", "token", controller.signal),
            "/api/inventory_movements/get_resolved?uuid=movement-1",
            "GET",
        ],
        [
            "delete",
            () => api.deleteItem("movement-1", "token", controller.signal),
            "/api/inventory_movements/delete?uuid=movement-1",
            "DELETE",
        ],
        [
            "select list",
            () => api.select_list("taxes", "token", controller.signal),
            "/api/inventory_movements/select_list?list=taxes",
            "GET",
        ],
        [
            "print",
            () => api.print("movement-1", "token", controller.signal),
            "/api/inventory_movements/print?uuid=movement-1",
            "GET",
        ],
    ])(
        "sends %s with the original route, method, token and signal",
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

    it("preserves every create body field", async () => {
        await api.create(input, null);
        const options = fetchMock.mock.calls[0][1];
        expect(options?.headers).not.toHaveProperty("Authorization");
        expect(JSON.parse(options?.body as string)).toEqual({
            id: null,
            inventory_id: "inventory-1",
            movement_type: "out",
            quantity: "2",
            reference_type: "worksheets",
            reference_id: "reference-1",
            unit_price: "10",
            total_price: "20",
            tax_id: "tax-1",
        });
    });

    it("retains valid backend field errors and rejects malformed ones", async () => {
        for (const [errorFields, expected] of [
            [fields, true],
            [{ ...fields, quantity: 2 }, false],
        ] as const) {
            const body = {
                error: { message: "Invalid input", fields: errorFields },
            };
            fetchMock.mockResolvedValueOnce(
                new Response(JSON.stringify(body), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await api.create(input, "token")).toEqual({
                kind: "http_error",
                status: 422,
                body: expected ? body : undefined,
            });
        }
    });

    it("returns PDF bytes and classifies a rate limit", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const response = new Response(bytes, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(response, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValueOnce(response);
        expect(await api.print("movement-1", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        fetchMock.mockResolvedValueOnce(
            new Response(null, {
                status: 429,
                headers: { "Retry-After": "5" },
            }),
        );
        expect(await api.get("movement-1", null)).toEqual({
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: 5,
        });
    });
});
