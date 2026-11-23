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
    isInventory,
    isInventoryResolved,
    isInventoryResponse,
    isPaginatedInventoryResolvedListResponse,
    isDeleteInventoryResponse,
} from "./guards";

const inventory = {
    id: "inventory",
    product_id: "product",
    warehouse_id: "warehouse",
    quantity_on_hand: "2",
    quantity_reserved: "1",
    quantity_available: "1",
    minimum_stock: null,
    maximum_stock: "10",
    currency_code: "EUR",
    status: "active",
    created_by_id: "user",
    created_at: "date",
    updated_at: "date",
    deleted_at: null,
};
const resolved = {
    ...inventory,
    product: "Product",
    warehouse: "Warehouse",
    currency: "Euro",
    created_by: "User",
};

describe("inventory schema guards", () => {
    it("validates base and resolved payloads without rejecting extra fields", () => {
        expect(isInventory({ ...inventory, extra: true })).toBe(true);
        expect(isInventory({ ...inventory, minimum_stock: 3 })).toBe(false);
        expect(isInventoryResolved(resolved)).toBe(true);
        const { currency: _currency, ...missing } = resolved;
        expect(isInventoryResolved(missing)).toBe(false);
    });

    it("validates success and pagination envelopes", () => {
        expect(isInventoryResponse({ data: inventory, extra: true })).toBe(
            true,
        );
        expect(isInventoryResponse({})).toBe(false);
        expect(isInventoryResponse({ data: null })).toBe(false);
        expect(
            isInventoryResponse({
                data: inventory,
                error: { message: "error" },
            }),
        ).toBe(false);
        expect(
            isDeleteInventoryResponse({ data: { message: "Deleted" } }),
        ).toBe(true);
        expect(
            isPaginatedInventoryResolvedListResponse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [resolved],
            }),
        ).toBe(true);
        expect(
            isPaginatedInventoryResolvedListResponse({ data: [resolved] }),
        ).toBe(false);
        expect(
            isPaginatedInventoryResolvedListResponse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [inventory],
            }),
        ).toBe(false);
    });
});
