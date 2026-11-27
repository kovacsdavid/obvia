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
    isCreateTaxResponse,
    isDeleteTaxResponse,
    isPaginatedTaxResolvedListResponse,
    isTaxResolved,
    isUpdateTaxResponse,
} from "./guards";
import { taxUserInputSchema } from "./schemas";

const tax = {
    id: "tax-1",
    rate: null,
    description: "VAT",
    country_code: "HU",
    tax_category: "AK",
    is_rate_applicable: false,
    legal_text: null,
    reporting_code: null,
    is_default: false,
    status: "active",
    created_by_id: "user-1",
    created_at: "date",
    updated_at: "date",
    deleted_at: null,
};
const resolved = { ...tax, country: "Hungary", created_by: "User" };

describe("tax schemas", () => {
    it("checks success payloads, required fields and envelope shape", () => {
        for (const guard of [isCreateTaxResponse, isUpdateTaxResponse]) {
            expect(guard({ data: { ...tax, extra: true } })).toBe(true);
            expect(guard({ data: { ...tax, rate: 27 } })).toBe(false);
            expect(guard({ data: { ...tax, is_default: null } })).toBe(false);
            expect(guard({ data: null })).toBe(false);
            expect(guard({ error: { message: "error" } })).toBe(false);
        }
        expect(isDeleteTaxResponse({ data: { message: "Deleted" } })).toBe(
            true,
        );
        expect(isDeleteTaxResponse({ data: {} })).toBe(false);
    });

    it("requires resolved fields and list metadata", () => {
        const meta = { page: 1, limit: 10, total: 1 };
        expect(isTaxResolved(resolved)).toBe(true);
        expect(isTaxResolved(tax)).toBe(false);
        expect(
            isPaginatedTaxResolvedListResponse({ meta, data: [resolved] }),
        ).toBe(true);
        expect(isPaginatedTaxResolvedListResponse({ data: [resolved] })).toBe(
            false,
        );
        expect(isPaginatedTaxResolvedListResponse({ meta, data: [tax] })).toBe(
            false,
        );
    });

    it("preserves nullable user input fields", () => {
        const input = {
            id: null,
            rate: null,
            description: "VAT",
            country_code: "HU",
            tax_category: "AK",
            is_rate_applicable: null,
            legal_text: null,
            reporting_code: null,
            is_default: false,
            status: "active",
        };
        expect(taxUserInputSchema.safeParse(input).success).toBe(true);
        expect(
            taxUserInputSchema.safeParse({ ...input, is_default: null })
                .success,
        ).toBe(false);
    });
});
