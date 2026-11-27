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
import * as servicesApi from "./service";
import { isServiceError } from "./guards";
import type { ServiceUserInput } from "./interface";

const input: ServiceUserInput = {
    id: "service-1",
    name: "Installation",
    description: "On site",
    default_price: "42.50",
    default_tax_id: "tax-1",
    currency_code: "EUR",
    status: "active",
    unit_of_measure_id: "unit-1",
    new_unit_of_measure: "hour",
};

const mappedInput = input;

const fields = {
    id: null,
    name: null,
    description: null,
    default_price: null,
    default_tax_id: null,
    currency_code: null,
    status: null,
    unit_of_measure_id: null,
    new_unit_of_measure: null,
};

describe("services API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    let controller: AbortController;

    beforeEach(() => {
        controller = new AbortController();
        fetchMock.mockReset();
        fetchMock.mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
    });

    afterEach(() => {
        vi.unstubAllGlobals();
    });

    it.each([
        [
            "create",
            () => servicesApi.create(input, "token", controller.signal),
            "/api/services/create",
            "POST",
        ],
        [
            "update",
            () => servicesApi.update(input, "token", controller.signal),
            "/api/services/update",
            "PUT",
        ],
        [
            "list",
            () => servicesApi.list(null, "token", controller.signal),
            "/api/services/list",
            "GET",
        ],
        [
            "filtered list",
            () =>
                servicesApi.list(
                    "name=repair&page=2",
                    "token",
                    controller.signal,
                ),
            "/api/services/list?q=name=repair&page=2",
            "GET",
        ],
        [
            "get",
            () => servicesApi.get("service-1", "token", controller.signal),
            "/api/services/get?uuid=service-1",
            "GET",
        ],
        [
            "get resolved",
            () =>
                servicesApi.get_resolved(
                    "service-1",
                    "token",
                    controller.signal,
                ),
            "/api/services/get_resolved?uuid=service-1",
            "GET",
        ],
        [
            "delete",
            () =>
                servicesApi.deleteItem("service-1", "token", controller.signal),
            "/api/services/delete?uuid=service-1",
            "DELETE",
        ],
        [
            "select list",
            () =>
                servicesApi.select_list(
                    "currencies",
                    "token",
                    controller.signal,
                ),
            "/api/services/select_list?list=currencies",
            "GET",
        ],
        [
            "print",
            () => servicesApi.print("service-1", "token", controller.signal),
            "/api/services/print?uuid=service-1",
            "GET",
        ],
    ])(
        "sends %s to the expected endpoint",
        async (_name, invoke, url, method) => {
            await invoke();
            expect(fetchMock).toHaveBeenCalledOnce();
            const [actualUrl, options] = fetchMock.mock.calls[0];
            expect(actualUrl).toBe(url);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            expect(options?.signal).toBeInstanceOf(AbortSignal);
            expect(options?.signal?.aborted).toBe(false);
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
        },
    );

    it.each([
        ["create", servicesApi.create],
        ["update", servicesApi.update],
    ])("maps all %s fields into the JSON body", async (_name, invoke) => {
        await invoke(input, "token");
        const options = fetchMock.mock.calls[0][1];
        expect(options?.headers).toMatchObject({
            "Content-Type": "application/json",
        });
        expect(JSON.parse(options?.body as string)).toEqual(mappedInput);
    });

    it("omits authorization when there is no token", async () => {
        await servicesApi.get("service-1", null);
        expect(fetchMock.mock.calls[0][1]?.headers).not.toHaveProperty(
            "Authorization",
        );
    });

    it.each(["create", "update"] as const)(
        "accepts valid %s field errors",
        async (operation) => {
            const body = {
                error: {
                    message: "Invalid input",
                    fields: { ...fields, name: "Required" },
                },
            };
            fetchMock.mockResolvedValue(
                new Response(JSON.stringify(body), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await servicesApi[operation](input, "token")).toEqual({
                kind: "http_error",
                status: 422,
                body,
            });
        },
    );

    it.each(["create", "update"] as const)(
        "discards malformed %s field errors",
        async (operation) => {
            const body = {
                error: {
                    message: "Invalid input",
                    fields: { ...fields, status: 12 },
                },
            };
            fetchMock.mockResolvedValue(
                new Response(JSON.stringify(body), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await servicesApi[operation](input, "token")).toEqual({
                kind: "http_error",
                status: 422,
                body: undefined,
            });
        },
    );

    it("returns PDF bytes", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70, 45, 49]);
        const response = new Response(bytes, {
            status: 200,
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(response, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValue(response);
        expect(await servicesApi.print("service-1", "token")).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
    });

    it.each([
        [
            "JSON",
            new Response(JSON.stringify({ data: "not bytes" }), {
                headers: { "Content-Type": "application/json" },
            }),
        ],
        [
            "malformed JSON",
            new Response("{", {
                headers: { "Content-Type": "application/json" },
            }),
        ],
        ["empty response", new Response(null, { status: 204 })],
        [
            "unsupported content type",
            new Response("not a PDF", {
                headers: { "Content-Type": "text/plain" },
            }),
        ],
    ])("rejects %s as a PDF response", async (_name, response) => {
        fetchMock.mockResolvedValue(response);
        expect(await servicesApi.print("service-1", "token")).toEqual({
            kind: "invalid_response",
            status: response.status,
        });
    });
});

describe("service form error guard", () => {
    it("accepts null or string for every field", () => {
        expect(isServiceError(fields)).toBe(true);
        for (const field of Object.keys(fields)) {
            expect(isServiceError({ ...fields, [field]: "Invalid" })).toBe(
                true,
            );
        }
    });

    it("rejects missing and incorrectly typed fields", () => {
        expect(isServiceError(null)).toBe(false);
        expect(isServiceError([])).toBe(false);
        expect(isServiceError("error")).toBe(false);
        for (const field of Object.keys(fields)) {
            const missing: Record<string, unknown> = { ...fields };
            delete missing[field];
            expect(isServiceError(missing)).toBe(false);
            expect(isServiceError({ ...fields, [field]: 123 })).toBe(false);
        }
    });
});
