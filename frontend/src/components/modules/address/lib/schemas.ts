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

import { z } from "zod";

const nullableString = z.string().nullable();

export const addressSchema = z.looseObject({
    id: z.string(),
    type: z.string(),
    country_code: z.string(),
    postal_code: z.string(),
    settlement: z.string(),
    mailbox: nullableString,
    topographic_number: nullableString,
    name_of_public_space: nullableString,
    type_of_public_space: nullableString,
    house_number: nullableString,
    building: nullableString,
    stairway: nullableString,
    floor: nullableString,
    door: nullableString,
});

export const addressErrorsSchema = z.looseObject({
    id: nullableString,
    type: nullableString,
    country_code: nullableString,
    postal_code: nullableString,
    settlement: nullableString,
    mailbox: nullableString,
    topographic_number: nullableString,
    name_of_public_space: nullableString,
    type_of_public_space: nullableString,
    house_number: nullableString,
    building: nullableString,
    stairway: nullableString,
    floor: nullableString,
    door: nullableString,
});
