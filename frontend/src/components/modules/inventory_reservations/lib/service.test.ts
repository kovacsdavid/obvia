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
import type { InventoryReservationUserInput } from "./interface";

const input: InventoryReservationUserInput = {
    id: null,
    inventoryId: "inventory-1",
    quantity: "3.5",
    referenceType: "worksheets",
    referenceId: "worksheet-1",
    reservedUntil: "2027-01-01",
    status: "active",
};
const fields = {
    id: null,
    inventory_id: null,
    quantity: "Invalid",
    reference_type: null,
    reference_id: null,
    reserved_until: null,
    status: null,
};

describe("inventory reservation API", () => {
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
            "/api/inventory_reservations/create",
            "POST",
        ],
        [
            "list",
            () =>
                api.list(
                    { inventoryId: "inv", query: null },
                    "token",
                    controller.signal,
                ),
            "/api/inventory_reservations/list?inventory_id=inv",
            "GET",
        ],
        [
            "filtered list",
            () =>
                api.list(
                    { inventoryId: "inv", query: "page=2&order=asc" },
                    "token",
                    controller.signal,
                ),
            "/api/inventory_reservations/list?inventory_id=inv&q=page=2&order=asc",
            "GET",
        ],
        [
            "get",
            () => api.get("id", "token", controller.signal),
            "/api/inventory_reservations/get?uuid=id",
            "GET",
        ],
        [
            "get resolved",
            () => api.get_resolved("id", "token", controller.signal),
            "/api/inventory_reservations/get_resolved?uuid=id",
            "GET",
        ],
        [
            "delete",
            () => api.deleteItem("id", "token", controller.signal),
            "/api/inventory_reservations/delete?uuid=id",
            "DELETE",
        ],
        [
            "select",
            () => api.select_list("inventory", "token", controller.signal),
            "/api/inventory_reservations/select_list?list=inventory",
            "GET",
        ],
        [
            "print",
            () => api.print("id", "token", controller.signal),
            "/api/inventory_reservations/print?uuid=id",
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
            expect(options?.signal?.aborted).toBe(false);
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
        },
    );

    it("preserves every create body field, including nullable values", async () => {
        await api.create(input, null);
        const options = fetchMock.mock.calls[0][1];
        expect(options?.headers).not.toHaveProperty("Authorization");
        expect(JSON.parse(options?.body as string)).toEqual({
            id: null,
            inventory_id: "inventory-1",
            quantity: "3.5",
            reference_type: "worksheets",
            reference_id: "worksheet-1",
            reserved_until: "2027-01-01",
            status: "active",
        });
    });

    it("preserves backend validation errors and rejects malformed fields", async () => {
        const body = { error: { message: "Invalid input", fields } };
        fetchMock.mockResolvedValueOnce(
            new Response(JSON.stringify(body), {
                status: 422,
                headers: { "Content-Type": "application/json" },
            }),
        );
        expect(await api.create(input, null)).toEqual({
            kind: "http_error",
            status: 422,
            body,
        });
        fetchMock.mockResolvedValueOnce(
            new Response(
                JSON.stringify({
                    error: {
                        message: "Invalid",
                        fields: { ...fields, quantity: 3 },
                    },
                }),
                {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                },
            ),
        );
        expect(await api.create(input, null)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it("returns PDF bytes and rejects non-PDF responses", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const response = new Response(bytes, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(response, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValueOnce(response).mockResolvedValueOnce(
            new Response("not pdf", {
                headers: { "Content-Type": "text/plain" },
            }),
        );
        expect(await api.print("id", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        expect(await api.print("id", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it("classifies rate limits and transport failures", async () => {
        fetchMock
            .mockResolvedValueOnce(
                new Response(null, {
                    status: 429,
                    headers: { "Retry-After": "5" },
                }),
            )
            .mockRejectedValueOnce(new TypeError("network"));
        expect(await api.get("id", null)).toEqual({
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: 5,
        });
        expect(await api.get("id", null)).toMatchObject({
            kind: "network_error",
        });
    });
});
