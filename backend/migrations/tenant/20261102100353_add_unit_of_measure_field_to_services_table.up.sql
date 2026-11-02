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

ALTER TABLE services
ADD COLUMN unit_of_measure_id uuid
REFERENCES units_of_measure(id);

WITH new_unit AS (
    INSERT INTO units_of_measure(unit_of_measure, module, created_by_id)
    SELECT 'óra', 'services', id FROM users
    ORDER BY created_at ASC
    LIMIT 1
    RETURNING id
)
UPDATE services SET unit_of_measure_id = new_unit.id
FROM new_unit
WHERE services.unit_of_measure_id IS NULL;

ALTER TABLE services ALTER COLUMN unit_of_measure_id SET NOT NULL;
