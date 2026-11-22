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
import { isTaxError } from "./guards";

describe("tax validation errors", () => {
    const fields = {
        id: null,
        rate: "Invalid rate",
        description: null,
        country_code: null,
        tax_category: null,
        is_rate_applicable: null,
        legal_text: null,
        reporting_code: null,
        is_default: null,
        status: null,
    };

    it("accepts the backend DTO and rejects missing or invalid fields", () => {
        expect(isTaxError(fields)).toBe(true);
        expect(isTaxError({ ...fields, is_default: true })).toBe(false);
        const { rate: _rate, ...missingRate } = fields;
        expect(isTaxError(missingRate)).toBe(false);
        expect(isTaxError(null)).toBe(false);
    });
});
