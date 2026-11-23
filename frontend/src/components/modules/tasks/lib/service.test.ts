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
    isCreateTaskResponse,
    isDeleteTaskResponse,
    isPaginatedTaskResolvedListResponse,
    isTaskErrors,
    isTaskResolved,
    isTaskResolvedResponse,
    isTaskResponse,
    isUpdateTaskResponse,
} from "./guards";
import type { TaskUserInput } from "./interface";

const task = {
    id: "task",
    worksheet_id: "worksheet",
    service_id: "service",
    currency_code: "EUR",
    quantity: null,
    price: "10",
    tax_id: "tax",
    created_by_id: "user",
    status: "open",
    priority: null,
    due_date: null,
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
    description: null,
};
const resolved = {
    ...task,
    worksheet: "Worksheet",
    service: "Service",
    tax: "Tax",
    created_by: "User",
};
const fields = {
    id: null,
    worksheet_id: null,
    service_id: null,
    currency_code: null,
    quantity: null,
    price: "Invalid",
    tax_id: null,
    status: null,
    priority: null,
    due_date: null,
    description: null,
};
const input: TaskUserInput = {
    id: null,
    worksheetId: "worksheet",
    serviceId: "service",
    currencyCode: "EUR",
    quantity: "1",
    price: "10",
    taxId: "tax",
    status: "open",
    priority: null,
    dueDate: null,
    description: null,
};
const meta = { page: 1, limit: 25, total: 1 };
const json = (data: unknown, status = 200) =>
    new Response(JSON.stringify(data), {
        status,
        headers: { "Content-Type": "application/json" },
    });

describe("task guards", () => {
    it("checks resolved fields, error fields, and success envelopes", () => {
        expect(isTaskResolved({ ...resolved, extra: true })).toBe(true);
        expect(isTaskResolved({ ...resolved, tax: null })).toBe(false);
        expect(isTaskErrors(fields)).toBe(true);
        expect(isTaskErrors({ ...fields, price: undefined })).toBe(false);
        expect(isTaskResponse({ data: task, meta: { arbitrary: true } })).toBe(
            false,
        );
        expect(isTaskResponse({ data: task, error: undefined })).toBe(false);
        expect(isDeleteTaskResponse({ data: { message: "Deleted" } })).toBe(
            true,
        );
        expect(isDeleteTaskResponse({ data: {} })).toBe(false);
        expect(
            isPaginatedTaskResolvedListResponse({ meta, data: [resolved] }),
        ).toBe(true);
        expect(isPaginatedTaskResolvedListResponse({ meta, data: [] })).toBe(
            true,
        );
        expect(isPaginatedTaskResolvedListResponse({ data: [] })).toBe(false);
        expect(
            isPaginatedTaskResolvedListResponse({ meta, data: [task] }),
        ).toBe(false);
    });

    it.each([
        ["create", isCreateTaskResponse, task],
        ["update", isUpdateTaskResponse, task],
        ["get", isTaskResponse, task],
        ["get resolved", isTaskResolvedResponse, resolved],
        ["delete", isDeleteTaskResponse, { message: "Deleted" }],
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
                isPaginatedTaskResolvedListResponse({
                    data: [],
                    meta: invalidMeta,
                }),
            ).toBe(false);
        }
    });
});

describe("task service", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        ["create", () => api.create(input, null), task],
        ["update", () => api.update(input, null), task],
        ["get", () => api.get("task", null), task],
        ["get resolved", () => api.get_resolved("task", null), resolved],
        ["delete", () => api.deleteItem("task", null), { message: "Deleted" }],
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
            expect(url).toBe(`/api/tasks/${name}`);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            expect(JSON.parse(options?.body as string)).toEqual({
                id: null,
                worksheet_id: "worksheet",
                service_id: "service",
                currency_code: "EUR",
                quantity: "1",
                price: "10",
                tax_id: "tax",
                status: "open",
                priority: null,
                due_date: null,
                description: null,
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
        fetchMock.mockResolvedValueOnce(json({ data: task, extra: true }));
        expect(await api.get("task", null)).toEqual({
            kind: "success",
            status: 200,
            data: { data: task, extra: true },
        });
        fetchMock.mockResolvedValueOnce(json({ data: null }));
        expect(await api.get("task", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(json({ data: [] }));
        expect(await api.list(null, null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await api.deleteItem("task", null)).toEqual({
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
            .mockResolvedValueOnce(json({ data: task }))
            .mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await api.print("task", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        expect(await api.print("task", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        expect(await api.print("task", null)).toEqual({
            kind: "invalid_response",
            status: 204,
        });
    });
});
