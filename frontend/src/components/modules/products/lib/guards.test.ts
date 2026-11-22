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
import { isProductError } from "./guards";

describe("isProductError", () => {
    const fields = {
        id: null,
        name: "Required",
        description: null,
        unit_of_measure_id: null,
        new_unit_of_measure: "Required",
        status: null,
    };

    it("accepts the backend's nullable product validation fields", () => {
        expect(isProductError(fields)).toBe(true);
    });

    it("rejects missing and malformed fields", () => {
        expect(isProductError({ ...fields, status: undefined })).toBe(false);
        expect(isProductError({ ...fields, new_unit_of_measure: 1 })).toBe(
            false,
        );
        expect(isProductError(null)).toBe(false);
    });
});
