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

import type {
    Address,
    AddressErrors,
} from "@/components/modules/address/lib/interface";

export const isAddress = (data: unknown): data is Address => {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "type" in data &&
        typeof data.type === "string" &&
        "country_code" in data &&
        typeof data.country_code === "string" &&
        "postal_code" in data &&
        typeof data.postal_code === "string" &&
        "settlement" in data &&
        typeof data.settlement === "string" &&
        "mailbox" in data &&
        (data.mailbox === null || typeof data.mailbox === "string") &&
        "topographic_number" in data &&
        (data.topographic_number === null ||
            typeof data.topographic_number === "string") &&
        "name_of_public_space" in data &&
        (data.name_of_public_space === null ||
            typeof data.name_of_public_space === "string") &&
        "type_of_public_space" in data &&
        (data.type_of_public_space === null ||
            typeof data.type_of_public_space === "string") &&
        "house_number" in data &&
        (data.house_number === null || typeof data.house_number === "string") &&
        "building" in data &&
        (data.building === null || typeof data.building === "string") &&
        "stairway" in data &&
        (data.stairway === null || typeof data.stairway === "string") &&
        "floor" in data &&
        (data.floor === null || typeof data.floor === "string") &&
        "door" in data &&
        (data.door === null || typeof data.door === "string")
    );
};

export const isAddressError = (data: unknown): data is AddressErrors => {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        (data.id === null || typeof data.id === "string") &&
        "type" in data &&
        (data.type === null || typeof data.type === "string") &&
        "country_code" in data &&
        (data.country_code === null || typeof data.country_code === "string") &&
        "postal_code" in data &&
        (data.postal_code === null || typeof data.postal_code === "string") &&
        "settlement" in data &&
        (data.settlement === null || typeof data.settlement === "string") &&
        "mailbox" in data &&
        (data.mailbox === null || typeof data.mailbox === "string") &&
        "topographic_number" in data &&
        (data.topographic_number === null ||
            typeof data.topographic_number === "string") &&
        "name_of_public_space" in data &&
        (data.name_of_public_space === null ||
            typeof data.name_of_public_space === "string") &&
        "type_of_public_space" in data &&
        (data.type_of_public_space === null ||
            typeof data.type_of_public_space === "string") &&
        (!("house_number" in data) ||
            data.house_number === null ||
            typeof data.house_number === "string") &&
        "building" in data &&
        (data.building === null || typeof data.building === "string") &&
        "stairway" in data &&
        (data.stairway === null || typeof data.stairway === "string") &&
        "floor" in data &&
        (data.floor === null || typeof data.floor === "string") &&
        "door" in data &&
        (data.door === null || typeof data.door === "string")
    );
};
