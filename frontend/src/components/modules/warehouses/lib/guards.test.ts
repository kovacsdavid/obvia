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
import { isWarehouseError } from "./guards";

describe("warehouse validation errors", () => {
    const fields = {
        id: null,
        name: "Required",
        contact_name: null,
        contact_phone: null,
        status: null,
    };

    it("accepts the backend DTO's nullable fields", () => {
        expect(isWarehouseError(fields)).toBe(true);
    });

    it("rejects missing or incorrectly typed fields", () => {
        expect(isWarehouseError({ ...fields, status: 1 })).toBe(false);
        const { contact_phone: _contactPhone, ...incomplete } = fields;
        expect(isWarehouseError(incomplete)).toBe(false);
        expect(isWarehouseError(null)).toBe(false);
    });
});
