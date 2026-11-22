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
import { isInventoryMovementError } from "./guards";

describe("inventory movement validation errors", () => {
    const fields = {
        id: null,
        inventory_id: "Invalid inventory",
        movement_type: null,
        quantity: "Required",
        reference_type: null,
        reference_id: null,
        unit_price: null,
        total_price: null,
        tax_id: null,
    };

    it("accepts the backend's nullable field errors", () => {
        expect(isInventoryMovementError(fields)).toBe(true);
    });

    it("rejects missing and malformed fields", () => {
        const { total_price: _totalPrice, ...missing } = fields;
        expect(isInventoryMovementError(missing)).toBe(false);
        expect(isInventoryMovementError({ ...fields, quantity: 3 })).toBe(
            false,
        );
        expect(isInventoryMovementError(null)).toBe(false);
    });
});
