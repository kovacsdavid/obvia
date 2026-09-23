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

import { type Address } from "@/components/modules/address/lib/interface";

interface AddressViewProps {
    value: Address | null | undefined;
}

export default function AddressView({ value }: AddressViewProps) {
    const displayAddress = (): string => {
        let address = "";
        if (value !== null && typeof value !== "undefined") {
            address += `${value.country_code}${value.postal_code} ${value.settlement},`;

            if (value.mailbox !== null) {
                address += ` PF. ${value.mailbox}`;
            }

            if (value.topographic_number !== null) {
                address += ` HRSZ.: ${value.topographic_number}`;
            }

            if (value.name_of_public_space !== null) {
                address += ` ${value.name_of_public_space}`;
            }

            if (value.type_of_public_space !== null) {
                address += ` ${value.type_of_public_space}`;
            }

            if (value.house_number !== null) {
                address += ` ${value.house_number}`;
            }

            if (value.building !== null) {
                if (
                    value.stairway === null &&
                    value.floor === null &&
                    value.door === null
                ) {
                    address += ` ${value.building} épület`;
                } else {
                    address += ` ${value.building} épület,`;
                }
            }

            if (value.stairway !== null) {
                if (value.floor === null && value.door === null) {
                    address += ` ${value.stairway} lépcsőház`;
                } else {
                    address += ` ${value.stairway} lépcsőház,`;
                }
            }

            if (value.floor !== null) {
                if (value.door === null) {
                    address += ` ${value.floor} emelet`;
                } else {
                    address += ` ${value.floor} emelet,`;
                }
                address += ` ${value.floor}`;
            }

            if (value.door !== null) {
                address += ` ${value.door} ajtó`;
            }
        }
        if (address === "") {
            return "N/A";
        }
        return address;
    };

    return <>{displayAddress()}</>;
}
