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
    inventoryReservationErrorSchema,
    inventoryReservationResponseSchema,
    paginatedInventoryReservationResolvedListResponseSchema,
} from "./schemas";
import {
    isInventoryReservation,
    isInventoryReservationResolved,
    isDeleteInventoryReservationResponse,
} from "./guards";

const reservation = {
    id: "reservation",
    inventory_id: "inventory",
    quantity: "3",
    reference_type: null,
    reference_id: null,
    reserved_until: null,
    status: "active",
    created_by_id: "user",
    created_at: "date",
    updated_at: "date",
};
const resolved = { ...reservation, created_by: "User" };

describe("inventory reservation schemas", () => {
    it("requires all nullable validation fields and accepts extra fields", () => {
        const fields = {
            id: null,
            inventory_id: null,
            quantity: "Invalid",
            reference_type: null,
            reference_id: null,
            reserved_until: null,
            status: null,
            extra: true,
        };
        expect(inventoryReservationErrorSchema.safeParse(fields).success).toBe(
            true,
        );
        expect(
            inventoryReservationErrorSchema.safeParse({
                ...fields,
                quantity: 3,
            }).success,
        ).toBe(false);
        const { status: _status, ...missing } = fields;
        expect(inventoryReservationErrorSchema.safeParse(missing).success).toBe(
            false,
        );
    });

    it("requires success data and valid pagination metadata", () => {
        expect(isInventoryReservation({ ...reservation, extra: true })).toBe(
            true,
        );
        expect(
            isInventoryReservation({ ...reservation, reserved_until: 3 }),
        ).toBe(false);
        expect(isInventoryReservationResolved(resolved)).toBe(true);
        expect(isInventoryReservationResolved(reservation)).toBe(false);
        expect(
            inventoryReservationResponseSchema.safeParse({
                data: reservation,
                extra: true,
            }).success,
        ).toBe(true);
        expect(
            inventoryReservationResponseSchema.safeParse({
                data: reservation,
                error: { message: "error" },
            }).success,
        ).toBe(false);
        expect(
            isDeleteInventoryReservationResponse({
                data: { message: "Deleted" },
            }),
        ).toBe(true);
        expect(inventoryReservationResponseSchema.safeParse({}).success).toBe(
            false,
        );
        expect(
            inventoryReservationResponseSchema.safeParse({ data: null })
                .success,
        ).toBe(false);
        expect(
            paginatedInventoryReservationResolvedListResponseSchema.safeParse({
                data: [],
            }).success,
        ).toBe(false);
        expect(
            paginatedInventoryReservationResolvedListResponseSchema.safeParse({
                meta: { page: 1, limit: 25, total: 0 },
                data: [],
            }).success,
        ).toBe(true);
        expect(
            paginatedInventoryReservationResolvedListResponseSchema.safeParse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [resolved],
            }).success,
        ).toBe(true);
        expect(
            paginatedInventoryReservationResolvedListResponseSchema.safeParse({
                meta: { page: 1, limit: 25, total: 1 },
                data: [reservation],
            }).success,
        ).toBe(false);
    });
});
