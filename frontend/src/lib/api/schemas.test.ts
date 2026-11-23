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

import { describe, expect, it } from "vitest";
import { z } from "zod";
import {
    paginatedSuccessResponseSchema,
    schemaGuard,
    selectOptionListResponseSchema,
    successResponseSchema,
} from "./schemas";

describe("success response schemas", () => {
    const success = schemaGuard(successResponseSchema(z.string()));
    const paginated = schemaGuard(
        paginatedSuccessResponseSchema(z.array(z.string())),
    );
    const meta = { page: 1, limit: 25, total: 0 };

    it("requires valid data and rejects error envelopes", () => {
        expect(success({ meta: null, data: "ok", extra: true })).toBe(true);
        expect(success({ data: "ok" })).toBe(true);
        for (const value of [
            {},
            { data: null },
            { error: { message: "error" } },
            { data: "ok", error: {} },
        ]) {
            expect(success(value)).toBe(false);
        }
    });

    it("requires pagination metadata and valid data", () => {
        expect(paginated({ meta, data: [] })).toBe(true);
        for (const value of [
            {},
            { data: [] },
            { meta: null, data: [] },
            { meta, data: null },
            { meta, data: [], error: {} },
        ]) {
            expect(paginated(value)).toBe(false);
        }
    });

    it("requires data even when the payload schema accepts undefined", () => {
        const looseSuccess = schemaGuard(successResponseSchema(z.unknown()));
        const loosePaginated = schemaGuard(
            paginatedSuccessResponseSchema(z.unknown()),
        );
        for (const data of [
            {},
            { data: undefined },
            { data: "ok", error: undefined },
        ]) {
            expect(looseSuccess(data)).toBe(false);
            expect(loosePaginated({ ...data, meta })).toBe(false);
        }
        expect(looseSuccess({ data: null })).toBe(true);
        expect(loosePaginated({ meta, data: null })).toBe(true);
    });
});

describe("select-list response schema", () => {
    const guard = schemaGuard(selectOptionListResponseSchema);

    it("accepts the non-paginated backend envelope and valid options", () => {
        expect(guard({ meta: null, data: [] })).toBe(true);
        expect(
            guard({ data: [{ value: "id", title: "Name", extra: true }] }),
        ).toBe(true);
    });

    it("rejects malformed options, missing data, errors, and pagination metadata", () => {
        for (const value of [
            {},
            { meta: null, data: null },
            { data: [{ value: 1, title: "Name" }] },
            { data: [{ value: "id" }] },
            { data: [], error: { message: "error" } },
            { error: { message: "error" } },
            { meta: { page: 1, limit: 25, total: 0 }, data: [] },
        ]) {
            expect(guard(value)).toBe(false);
        }
    });
});
