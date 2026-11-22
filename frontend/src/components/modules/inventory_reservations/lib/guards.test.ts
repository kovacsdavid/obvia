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
import { isInventoryReservationError } from "./guards";

describe("isInventoryReservationError", () => {
    const fields = {
        id: null,
        inventory_id: null,
        quantity: "Invalid quantity",
        reference_type: null,
        reference_id: null,
        reserved_until: null,
        status: null,
    };

    it("accepts the backend field error shape", () => {
        expect(isInventoryReservationError(fields)).toBe(true);
    });

    it("rejects missing and malformed fields", () => {
        expect(isInventoryReservationError({ ...fields, quantity: 3 })).toBe(
            false,
        );
        const { status: _status, ...withoutStatus } = fields;
        expect(isInventoryReservationError(withoutStatus)).toBe(false);
    });
});
