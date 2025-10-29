/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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


DROP TABLE IF EXISTS project_assignments CASCADE;
DROP TABLE IF EXISTS task_assignments CASCADE;
DROP TABLE IF EXISTS inventory CASCADE;
DROP TABLE IF EXISTS warehouses CASCADE;
DROP TABLE IF EXISTS product_category_connect CASCADE;
DROP TABLE IF EXISTS product_category CASCADE;
DROP TABLE IF EXISTS products CASCADE;
DROP TABLE IF EXISTS units_of_measure CASCADE;
DROP TABLE IF EXISTS currencies CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;
DROP TABLE IF EXISTS worksheets CASCADE;
DROP TABLE IF EXISTS projects CASCADE;
DROP TABLE IF EXISTS tag_connect CASCADE;
DROP TABLE IF EXISTS tags CASCADE;
DROP TABLE IF EXISTS customer_address_connect CASCADE;
DROP TABLE IF EXISTS address CASCADE;
DROP TABLE IF EXISTS cities CASCADE;
DROP TABLE IF EXISTS postal_codes CASCADE;
DROP TABLE IF EXISTS states CASCADE;
DROP TABLE IF EXISTS countries CASCADE;
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS customers CASCADE;
DROP TABLE IF EXISTS users CASCADE;
