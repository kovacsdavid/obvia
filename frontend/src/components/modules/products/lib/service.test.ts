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
import * as productsApi from "./service";

const input = {
    id: "product-1",
    name: "Product",
    description: "Description",
    unitOfMeasureId: "other",
    newUnitOfMeasure: "box",
    status: "active",
};
const fields = {
    id: null,
    name: "Required",
    description: null,
    unit_of_measure_id: null,
    new_unit_of_measure: "Required",
    status: null,
};

describe("products API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock.mockReset();
        fetchMock.mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        [
            "create",
            (signal: AbortSignal) => productsApi.create(input, "token", signal),
            "/api/products/create",
            "POST",
        ],
        [
            "update",
            (signal: AbortSignal) => productsApi.update(input, "token", signal),
            "/api/products/update",
            "PUT",
        ],
        [
            "list",
            (signal: AbortSignal) => productsApi.list(null, "token", signal),
            "/api/products/list",
            "GET",
        ],
        [
            "filtered list",
            (signal: AbortSignal) =>
                productsApi.list("page=2&name=box", "token", signal),
            "/api/products/list?q=page=2&name=box",
            "GET",
        ],
        [
            "get",
            (signal: AbortSignal) =>
                productsApi.get("product-1", "token", signal),
            "/api/products/get?uuid=product-1",
            "GET",
        ],
        [
            "resolved",
            (signal: AbortSignal) =>
                productsApi.get_resolved("product-1", "token", signal),
            "/api/products/get_resolved?uuid=product-1",
            "GET",
        ],
        [
            "select",
            (signal: AbortSignal) =>
                productsApi.select_list("units_of_measure", "token", signal),
            "/api/products/select_list?list=units_of_measure",
            "GET",
        ],
        [
            "delete",
            (signal: AbortSignal) =>
                productsApi.deleteItem("product-1", "token", signal),
            "/api/products/delete?uuid=product-1",
            "DELETE",
        ],
        [
            "print",
            (signal: AbortSignal) =>
                productsApi.print("product-1", "token", signal),
            "/api/products/print?uuid=product-1",
            "GET",
        ],
    ])(
        "sends %s with authentication and a cancellable signal",
        async (_name, invoke, url, method) => {
            const controller = new AbortController();
            await invoke(controller.signal);
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

    it.each(["create", "update"] as const)(
        "maps %s body and preserves validation errors",
        async (operation) => {
            const body = { error: { message: "Invalid input", fields } };
            fetchMock.mockResolvedValue(
                new Response(JSON.stringify(body), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await productsApi[operation](input, null)).toEqual({
                kind: "http_error",
                status: 422,
                body,
            });
            const options = fetchMock.mock.calls[0][1];
            expect(JSON.parse(options?.body as string)).toEqual({
                id: input.id,
                name: input.name,
                description: input.description,
                unit_of_measure_id: input.unitOfMeasureId,
                new_unit_of_measure: input.newUnitOfMeasure,
                status: input.status,
            });
            expect(options?.headers).not.toHaveProperty("Authorization");
        },
    );

    it("reports malformed successful data and rate limits", async () => {
        fetchMock.mockResolvedValueOnce(
            new Response(JSON.stringify({ data: {} }), {
                status: 200,
                headers: { "Content-Type": "application/json" },
            }),
        );
        expect(await productsApi.get("product-1", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(
            new Response(null, {
                status: 429,
                headers: { "Retry-After": "5" },
            }),
        );
        expect(await productsApi.list(null, null)).toEqual({
            kind: "rate_limited",
            status: 429,
            retryAfterSeconds: 5,
        });
    });
});
