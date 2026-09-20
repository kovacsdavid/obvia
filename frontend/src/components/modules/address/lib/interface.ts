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

export interface Address {
    id: string;
    type: string;
    country_code: string;
    postal_code: string;
    settlement: string;
    mailbox: string | null;
    topographic_number: string | null;
    name_of_public_space: string | null;
    type_of_public_space: string | null;
    house_number: string | null;
    building: string | null;
    stairway: string | null;
    floor: string | null;
    door: string | null;
}

export interface AddressErrors {
    id: string | null;
    type: string | null;
    country_code: string | null;
    postal_code: string | null;
    settlement: string | null;
    mailbox: string | null;
    topographic_number: string | null;
    name_of_public_space: string | null;
    type_of_public_space: string | null;
    house_number: string | null;
    building: string | null;
    stairway: string | null;
    floor: string | null;
    door: string | null;
}
