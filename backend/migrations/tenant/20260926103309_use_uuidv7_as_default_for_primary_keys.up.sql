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

ALTER TABLE address ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE cities ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE comments ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE customers ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE inventory ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE inventory_movements ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE inventory_reservations ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE postal_codes ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE product_category ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE product_category_connect ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE products ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE project_assignments ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE projects ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE services ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE states ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE tag_connect ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE tags ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE task_assignments ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE tasks ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE taxes ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE units_of_measure ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE users ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE warehouses ALTER COLUMN id SET DEFAULT uuidv7();
ALTER TABLE worksheets ALTER COLUMN id SET DEFAULT uuidv7();
