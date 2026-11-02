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

ALTER TABLE units_of_measure ADD COLUMN module VARCHAR(50) DEFAULT 'products' NOT NULL;

ALTER TABLE units_of_measure DROP CONSTRAINT units_of_measure_unit_of_measure_deleted_at_key;

ALTER TABLE units_of_measure
ADD CONSTRAINT units_of_measure_unit_of_measure_module_deleted_at_key
UNIQUE NULLS NOT DISTINCT (unit_of_measure, module, deleted_at);
