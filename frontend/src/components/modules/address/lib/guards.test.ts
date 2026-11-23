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

import { describe, expect, expectTypeOf, it } from "vitest";
import { isAddress, isAddressError } from "./guards";
import type { Address, AddressErrors } from "./interface";

const address = {
    id: "address-id",
    type: "billing",
    country_code: "HU",
    postal_code: "1111",
    settlement: "Budapest",
    mailbox: null,
    topographic_number: null,
    name_of_public_space: null,
    type_of_public_space: null,
    house_number: null,
    building: null,
    stairway: null,
    floor: null,
    door: null,
} satisfies Address;

const addressErrors = {
    id: null,
    type: null,
    country_code: null,
    postal_code: null,
    settlement: null,
    mailbox: null,
    topographic_number: null,
    name_of_public_space: null,
    type_of_public_space: null,
    house_number: null,
    building: null,
    stairway: null,
    floor: null,
    door: null,
} satisfies AddressErrors;

describe("address guards", () => {
    it("accepts backend address and error fields, including nullable values and extra fields", () => {
        expect(isAddress({ ...address, house_number: "12", extra: true })).toBe(
            true,
        );
        expect(isAddress(address)).toBe(true);
        expect(
            isAddressError({
                ...addressErrors,
                house_number: "Required",
                extra: true,
            }),
        ).toBe(true);
        expect(isAddressError(addressErrors)).toBe(true);
    });

    it.each([
        ["address", isAddress, address],
        ["address error", isAddressError, addressErrors],
    ])(
        "requires every %s field, including house_number",
        (_name, guard, value) => {
            for (const field of Object.keys(value)) {
                const incomplete = Object.fromEntries(
                    Object.entries(value).filter(([key]) => key !== field),
                );
                expect(guard(incomplete), `missing ${field}`).toBe(false);
                expect(
                    guard({ ...value, [field]: 123 }),
                    `invalid ${field}`,
                ).toBe(false);
            }
            expect(guard(null)).toBe(false);
            expect(guard([])).toBe(false);
        },
    );

    it("requires the inferred house_number fields to be nullable strings", () => {
        expectTypeOf<Pick<Address, "house_number">>().toEqualTypeOf<{
            house_number: string | null;
        }>();
        expectTypeOf<Pick<AddressErrors, "house_number">>().toEqualTypeOf<{
            house_number: string | null;
        }>();
    });
});
