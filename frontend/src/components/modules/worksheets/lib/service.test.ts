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
import {
    isCreateWorksheetResponse,
    isDeleteWorksheetResponse,
    isPaginatedWorksheetResolvedListResponse,
    isUpdateWorksheetResponse,
    isWorksheetError,
    isWorksheetResolved,
    isWorksheetResolvedResponse,
    isWorksheetResponse,
} from "./guards";
import type { WorksheetUserInput } from "./interface";

const worksheet = {
    id: "worksheet",
    display_id: "WS-1",
    description: null,
    customer_id: "customer",
    project_id: null,
    created_by_id: "user",
    status: "open",
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
};
const resolved = {
    ...worksheet,
    customer: "Customer",
    project: null,
    created_by: "User",
    net_material_cost: "0",
    gross_material_cost: "0",
    net_work_cost: "10",
    gross_work_cost: "12",
};
const fields = {
    id: null,
    description: null,
    customer_id: "Required",
    project_id: null,
    status: null,
};
const input: WorksheetUserInput = {
    id: null,
    description: "Work",
    customer_id: "customer",
    project_id: "",
    status: "open",
};
const meta = { page: 1, limit: 25, total: 1 };
const json = (data: unknown, status = 200) =>
    new Response(JSON.stringify(data), {
        status,
        headers: { "Content-Type": "application/json" },
    });

describe("worksheet guards", () => {
    it("checks resolved fields, error fields, and success envelopes", () => {
        expect(isWorksheetResolved({ ...resolved, extra: true })).toBe(true);
        expect(isWorksheetResolved({ ...resolved, project: 1 })).toBe(false);
        expect(isWorksheetError(fields)).toBe(true);
        expect(isWorksheetError({ ...fields, status: undefined })).toBe(false);
        expect(
            isWorksheetResponse({ data: worksheet, meta: { arbitrary: true } }),
        ).toBe(false);
        expect(isWorksheetResponse({ data: worksheet, error: undefined })).toBe(
            false,
        );
        expect(
            isDeleteWorksheetResponse({ data: { message: "Deleted" } }),
        ).toBe(true);
        expect(isDeleteWorksheetResponse({ data: {} })).toBe(false);
        expect(
            isPaginatedWorksheetResolvedListResponse({
                meta,
                data: [resolved],
            }),
        ).toBe(true);
        expect(
            isPaginatedWorksheetResolvedListResponse({ meta, data: [] }),
        ).toBe(true);
        expect(isPaginatedWorksheetResolvedListResponse({ data: [] })).toBe(
            false,
        );
        expect(
            isPaginatedWorksheetResolvedListResponse({
                meta,
                data: [worksheet],
            }),
        ).toBe(false);
    });

    it.each([
        ["create", isCreateWorksheetResponse, worksheet],
        ["update", isUpdateWorksheetResponse, worksheet],
        ["get", isWorksheetResponse, worksheet],
        ["get resolved", isWorksheetResolvedResponse, resolved],
        ["delete", isDeleteWorksheetResponse, { message: "Deleted" }],
    ])("validates %s metadata", (_name, guard, data) => {
        expect(guard({ data })).toBe(true);
        expect(guard({ data, meta: null })).toBe(true);
        for (const invalidMeta of [{}, meta, "invalid", 1, false, []]) {
            expect(guard({ data, meta: invalidMeta })).toBe(false);
        }
    });

    it("requires valid pagination metadata", () => {
        for (const invalidMeta of [null, {}, { ...meta, page: "1" }]) {
            expect(
                isPaginatedWorksheetResolvedListResponse({
                    data: [],
                    meta: invalidMeta,
                }),
            ).toBe(false);
        }
    });
});

describe("worksheet service", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        ["create", () => api.create(input, null), worksheet],
        ["update", () => api.update(input, null), worksheet],
        ["get", () => api.get("worksheet", null), worksheet],
        ["get resolved", () => api.get_resolved("worksheet", null), resolved],
        [
            "delete",
            () => api.deleteItem("worksheet", null),
            { message: "Deleted" },
        ],
    ])(
        "%s accepts null metadata and rejects invalid metadata",
        async (_name, invoke, data) => {
            fetchMock.mockResolvedValueOnce(json({ data, meta: null }));
            expect(await invoke()).toMatchObject({
                kind: "success",
                data: { data, meta: null },
            });
            fetchMock.mockResolvedValueOnce(
                json({ data, meta: { arbitrary: true } }),
            );
            expect(await invoke()).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        },
    );

    it("accepts paginated list responses and rejects invalid metadata", async () => {
        fetchMock.mockResolvedValueOnce(json({ data: [resolved], meta }));
        expect(await api.list(null, null)).toMatchObject({
            kind: "success",
            data: { data: [resolved], meta },
        });
        fetchMock.mockResolvedValueOnce(json({ data: [], meta: null }));
        expect(await api.list(null, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it.each([
        ["create", api.create, "POST"],
        ["update", api.update, "PUT"],
    ])(
        "%s maps input and preserves validation errors",
        async (name, invoke, method) => {
            fetchMock.mockResolvedValue(
                json({ error: { message: "Invalid", fields } }, 422),
            );
            expect(await invoke(input, "token")).toEqual({
                kind: "http_error",
                status: 422,
                body: { error: { message: "Invalid", fields } },
            });
            const [url, options] = fetchMock.mock.calls[0];
            expect(url).toBe(`/api/worksheets/${name}`);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            expect(JSON.parse(options?.body as string)).toEqual({
                id: null,
                description: "Work",
                customer_id: "customer",
                project_id: "",
                status: "open",
            });
        },
    );

    it("handles valid and invalid error and success responses", async () => {
        fetchMock.mockResolvedValueOnce(
            json({ error: { message: "Invalid" } }, 422),
        );
        expect(await api.create(input, null)).toMatchObject({
            kind: "http_error",
            body: { error: { message: "Invalid" } },
        });
        fetchMock.mockResolvedValueOnce(
            json({ error: { message: "Invalid", fields: {} } }, 422),
        );
        expect(await api.update(input, null)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
        fetchMock.mockResolvedValueOnce(json({ data: worksheet, extra: true }));
        expect(await api.get("worksheet", null)).toEqual({
            kind: "success",
            status: 200,
            data: { data: worksheet, extra: true },
        });
        fetchMock.mockResolvedValueOnce(json({ data: null }));
        expect(await api.get("worksheet", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(json({ data: [] }));
        expect(await api.list(null, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await api.deleteItem("worksheet", null)).toEqual({
            kind: "invalid_response",
            status: 204,
        });
    });

    it("accepts PDF bytes and rejects JSON and 204 print responses", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const pdf = new Response(null, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(pdf, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock
            .mockResolvedValueOnce(pdf)
            .mockResolvedValueOnce(json({ data: worksheet }))
            .mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await api.print("worksheet", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        expect(await api.print("worksheet", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await api.print("worksheet", null)).toEqual({
            kind: "invalid_response",
            status: 204,
        });
    });
});
