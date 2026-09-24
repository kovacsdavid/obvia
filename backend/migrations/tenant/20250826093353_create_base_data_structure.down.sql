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

DROP TRIGGER IF EXISTS update_updated_at_on_inventory_table ON inventory;
DROP TRIGGER IF EXISTS update_updated_at_on_warehouses_table ON warehouses;
DROP TRIGGER IF EXISTS update_updated_at_on_products_table ON products;
DROP TRIGGER IF EXISTS update_updated_at_on_tasks_table ON tasks;
DROP TRIGGER IF EXISTS update_updated_at_on_services_table ON services;
DROP TRIGGER IF EXISTS update_updated_at_on_worksheets_table ON worksheets;
DROP TRIGGER IF EXISTS update_updated_at_on_taxes_table ON taxes;
DROP TRIGGER IF EXISTS update_updated_at_on_projects_table ON projects;
DROP TRIGGER IF EXISTS update_updated_at_on_address_connect_table ON address_connect;
DROP TRIGGER IF EXISTS update_updated_at_on_address_table ON address;
DROP TRIGGER IF EXISTS update_updated_at_on_states_table ON states;
DROP TRIGGER IF EXISTS update_updated_at_on_comments_table ON comments;
DROP TRIGGER IF EXISTS update_updated_at_on_customers_table ON customers;
DROP TRIGGER IF EXISTS update_updated_at_on_users_table ON users;

DROP TABLE IF EXISTS project_assignments;
DROP TABLE IF EXISTS task_assignments;
DROP TABLE IF EXISTS inventory;
DROP TABLE IF EXISTS warehouses;
DROP TABLE IF EXISTS product_category_connect;
DROP TABLE IF EXISTS product_category;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS units_of_measure;
DROP TABLE IF EXISTS tasks;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS currencies;
DROP TABLE IF EXISTS taxes;
DROP TABLE IF EXISTS worksheets;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS tag_connect;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS address_connect;
DROP TABLE IF EXISTS address;
DROP TABLE IF EXISTS cities;
DROP TABLE IF EXISTS postal_codes;
DROP TABLE IF EXISTS states;
DROP TABLE IF EXISTS countries;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS customers;
DROP TABLE IF EXISTS users;

DROP FUNCTION IF EXISTS update_updated_at();

DROP EXTENSION IF EXISTS "uuid-ossp";
