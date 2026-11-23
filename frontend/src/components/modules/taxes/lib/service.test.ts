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
import type { TaxUserInput } from "./interface";

const input: TaxUserInput = {
    id: "tax-1",
    rate: "27",
    description: "VAT",
    countryCode: "HU",
    taxCategory: "AK",
    isRateApplicable: true,
    legalText: "text",
    reportingCode: "code",
    isDefault: false,
    status: "active",
};
const body = {
    id: "tax-1",
    rate: "27",
    description: "VAT",
    country_code: "HU",
    tax_category: "AK",
    is_rate_applicable: true,
    legal_text: "text",
    reporting_code: "code",
    is_default: false,
    status: "active",
};
const fields = {
    id: null,
    rate: "Invalid",
    description: null,
    country_code: null,
    tax_category: null,
    is_rate_applicable: null,
    legal_text: null,
    reporting_code: null,
    is_default: null,
    status: null,
};

describe("tax API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock
            .mockReset()
            .mockResolvedValue(new Response(null, { status: 204 }));
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        [
            "create",
            (signal: AbortSignal) => api.create(input, "token", signal),
            "/api/taxes/create",
            "POST",
        ],
        [
            "update",
            (signal: AbortSignal) => api.update(input, "token", signal),
            "/api/taxes/update",
            "PUT",
        ],
        [
            "list",
            (signal: AbortSignal) => api.list(null, "token", signal),
            "/api/taxes/list",
            "GET",
        ],
        [
            "filtered list",
            (signal: AbortSignal) => api.list("page=2", "token", signal),
            "/api/taxes/list?q=page=2",
            "GET",
        ],
        [
            "get",
            (signal: AbortSignal) => api.get("tax-1", "token", signal),
            "/api/taxes/get?uuid=tax-1",
            "GET",
        ],
        [
            "resolved",
            (signal: AbortSignal) => api.get_resolved("tax-1", "token", signal),
            "/api/taxes/get_resolved?uuid=tax-1",
            "GET",
        ],
        [
            "delete",
            (signal: AbortSignal) => api.deleteItem("tax-1", "token", signal),
            "/api/taxes/delete?uuid=tax-1",
            "DELETE",
        ],
        [
            "select",
            (signal: AbortSignal) =>
                api.select_list("countries", "token", signal),
            "/api/taxes/select_list?list=countries",
            "GET",
        ],
        [
            "print",
            (signal: AbortSignal) => api.print("tax-1", "token", signal),
            "/api/taxes/print?uuid=tax-1",
            "GET",
        ],
    ])(
        "sends %s with the expected route, method, token and signal",
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
        "maps the %s request body",
        async (operation) => {
            await api[operation](input, null);
            expect(
                JSON.parse(fetchMock.mock.calls[0][1]?.body as string),
            ).toEqual(body);
            expect(fetchMock.mock.calls[0][1]?.headers).not.toHaveProperty(
                "Authorization",
            );
        },
    );

    it.each(["create", "update"] as const)(
        "recognizes valid and malformed %s validation errors",
        async (operation) => {
            const error = { error: { message: "Invalid", fields } };
            fetchMock.mockResolvedValueOnce(
                new Response(JSON.stringify(error), {
                    status: 422,
                    headers: { "Content-Type": "application/json" },
                }),
            );
            expect(await api[operation](input, null)).toEqual({
                kind: "http_error",
                status: 422,
                body: error,
            });
            fetchMock.mockResolvedValueOnce(
                new Response(
                    JSON.stringify({
                        error: {
                            ...error.error,
                            fields: { ...fields, is_default: true },
                        },
                    }),
                    {
                        status: 422,
                        headers: { "Content-Type": "application/json" },
                    },
                ),
            );
            expect(await api[operation](input, null)).toEqual({
                kind: "http_error",
                status: 422,
                body: undefined,
            });
        },
    );

    it("returns PDF bytes and rejects non-PDF content", async () => {
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
        expect(await api.print("tax-1", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        expect(await api.print("tax-1", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it("validates paginated responses and preserves 204 behavior", async () => {
        const data = [
            {
                id: "tax-1",
                rate: null,
                description: "VAT",
                country_code: "HU",
                country: "Hungary",
                tax_category: "AK",
                is_rate_applicable: false,
                legal_text: null,
                reporting_code: null,
                is_default: false,
                status: "active",
                created_by_id: "user-1",
                created_by: "User",
                created_at: "date",
                updated_at: "date",
                deleted_at: null,
                extra: "retained",
            },
        ];
        const meta = { page: 1, limit: 10, total: 1 };
        fetchMock
            .mockResolvedValueOnce(
                new Response(JSON.stringify({ meta, data }), {
                    headers: { "Content-Type": "application/json" },
                }),
            )
            .mockResolvedValueOnce(
                new Response(JSON.stringify({ data }), {
                    headers: { "Content-Type": "application/json" },
                }),
            );
        expect(await api.list(null, null)).toEqual({
            kind: "success",
            status: 200,
            data: { meta, data },
        });
        expect(await api.list(null, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await api.deleteItem("tax-1", null)).toEqual({
            kind: "invalid_response",
            status: 204,
        });
        expect(await api.print("tax-1", null)).toEqual({
            kind: "invalid_response",
            status: 204,
        });
    });
});
