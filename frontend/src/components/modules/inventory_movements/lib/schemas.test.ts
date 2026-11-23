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
    isInventoryMovement,
    isInventoryMovementResolved,
    isInventoryMovementResponse,
    isPaginatedInventoryMovementResolvedListResponse,
    isDeleteInventoryMovementResponse,
} from "./guards";

const movement = {
    id: "movement",
    inventory_id: "inventory",
    movement_type: "out",
    quantity: "2",
    reference_type: null,
    reference_id: null,
    unit_price: null,
    total_price: "20",
    tax_id: "tax",
    movement_date: "date",
    created_by_id: "user",
    created_at: "date",
};
const resolved = { ...movement, tax: null, created_by: "User" };

describe("inventory movement schema guards", () => {
    it("validates nullable prices and resolved fields", () => {
        expect(isInventoryMovement({ ...movement, extra: true })).toBe(true);
        expect(isInventoryMovement({ ...movement, unit_price: 10 })).toBe(
            false,
        );
        expect(isInventoryMovementResolved(resolved)).toBe(true);
        const { tax: _tax, ...missing } = resolved;
        expect(isInventoryMovementResolved(missing)).toBe(false);
    });

    it("validates success and pagination envelopes", () => {
        expect(
            isInventoryMovementResponse({ data: movement, extra: true }),
        ).toBe(true);
        expect(isInventoryMovementResponse({})).toBe(false);
        expect(isInventoryMovementResponse({ data: null })).toBe(false);
        expect(
            isInventoryMovementResponse({
                data: movement,
                error: { message: "error" },
            }),
        ).toBe(false);
        expect(
            isDeleteInventoryMovementResponse({ data: { message: "Deleted" } }),
        ).toBe(true);
        expect(
            isPaginatedInventoryMovementResolvedListResponse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [resolved],
            }),
        ).toBe(true);
        expect(
            isPaginatedInventoryMovementResolvedListResponse({
                data: [resolved],
            }),
        ).toBe(false);
        expect(
            isPaginatedInventoryMovementResolvedListResponse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [movement],
            }),
        ).toBe(false);
    });
});
