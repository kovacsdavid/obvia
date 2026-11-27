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
import {
    paginatedWarehouseResolvedListResponseSchema,
    warehouseResponseSchema,
    warehouseUserInputSchema,
} from "./schemas";

const warehouse = {
    id: "warehouse-1",
    name: "Main",
    contact_name: null,
    contact_phone: null,
    status: "active",
    created_by_id: "user-1",
    created_at: "date",
    updated_at: "date",
    deleted_at: null,
};

describe("warehouse schemas", () => {
    it("accepts nullable backend fields and rejects missing or invalid fields", () => {
        expect(
            warehouseResponseSchema.safeParse({ data: warehouse }).success,
        ).toBe(true);
        expect(
            warehouseResponseSchema.safeParse({
                data: { ...warehouse, contact_phone: 5 },
            }).success,
        ).toBe(false);
        const { deleted_at: _deletedAt, ...incomplete } = warehouse;
        expect(
            warehouseResponseSchema.safeParse({ data: incomplete }).success,
        ).toBe(false);
        expect(warehouseResponseSchema.safeParse({ data: null }).success).toBe(
            false,
        );
        expect(
            warehouseResponseSchema.safeParse({ error: { message: "error" } })
                .success,
        ).toBe(false);
    });

    it("requires pagination metadata and resolved fields", () => {
        const data = [{ ...warehouse, created_by: "User", extra: true }];
        expect(
            paginatedWarehouseResolvedListResponseSchema.safeParse({
                meta: { page: 1, limit: 10, total: 1 },
                data,
            }).success,
        ).toBe(true);
        expect(
            paginatedWarehouseResolvedListResponseSchema.safeParse({ data })
                .success,
        ).toBe(false);
        expect(
            paginatedWarehouseResolvedListResponseSchema.safeParse({
                meta: { page: 1, limit: 10, total: 1 },
                data: [warehouse],
            }).success,
        ).toBe(false);
    });

    it("preserves user input nullability", () => {
        expect(
            warehouseUserInputSchema.safeParse({
                id: null,
                name: "Main",
                contact_name: "",
                contact_phone: "",
                status: "active",
            }).success,
        ).toBe(true);
        expect(
            warehouseUserInputSchema.safeParse({
                id: null,
                name: "Main",
                contact_name: null,
                contact_phone: "",
                status: "active",
            }).success,
        ).toBe(false);
    });
});
