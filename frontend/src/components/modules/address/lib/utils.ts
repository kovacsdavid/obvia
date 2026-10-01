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

import type { Address } from "@/components/modules/address/lib/interface";

export function normalizeAddress(address: Address | null): Address | null {
    if (address === null) return null;

    return {
        ...address,
        mailbox: address.mailbox ?? "",
        topographic_number: address.topographic_number ?? "",
        name_of_public_space: address.name_of_public_space ?? "",
        type_of_public_space: address.type_of_public_space ?? "",
        house_number: address.house_number ?? "",
        building: address.building ?? "",
        stairway: address.stairway ?? "",
        floor: address.floor ?? "",
        door: address.door ?? "",
    };
}

export const defaultAddress = () => ({
    id: "",
    type: "full_address",
    country_code: "HU",
    postal_code: "",
    settlement: "",
    mailbox: "",
    topographic_number: "",
    name_of_public_space: "",
    type_of_public_space: "",
    house_number: "",
    building: "",
    stairway: "",
    floor: "",
    door: "",
});
