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

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';


create table users
(
    id                  uuid primary key,
    email               varchar(255) not null unique,
    first_name          varchar(255),
    last_name           varchar(255),
    phone               varchar(32),
    status              varchar(50)  not null default 'active',
    profile_picture_url text,
    locale              varchar(16)           default 'hu-HU',
    invited_by          uuid,
    email_verified_at   timestamptz,
    created_at          timestamptz  not null default now(),
    updated_at          timestamptz  not null default now(),
    deleted_at          timestamptz,
    foreign key (invited_by) references users (id)
);

CREATE INDEX idx_users_invited_by ON users (invited_by);
CREATE INDEX idx_users_created_at ON users (created_at);
CREATE INDEX idx_users_updated_at ON users (updated_at);
CREATE INDEX idx_users_deleted_at ON users (deleted_at);

CREATE TRIGGER update_updated_at_on_users_table
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table customers
(
    id           uuid primary key      default uuid_generate_v4(),
    name         varchar(255) not null,
    contact_name varchar(255),
    email        varchar(255) not null unique,
    phone_number varchar(50),
    status       varchar(50)  not null,
    type         varchar(50)  not null,
    created_by   uuid         not null,
    created_at   timestamptz  not null default now(),
    updated_at   timestamptz  not null default now(),
    deleted_at   timestamptz,
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_customers_created_by ON customers (created_by);
CREATE INDEX idx_customers_created_at ON customers (created_at);
CREATE INDEX idx_customers_updated_at ON customers (updated_at);
CREATE INDEX idx_customers_deleted_at ON customers (deleted_at);

CREATE TRIGGER update_updated_at_on_customers_table
    BEFORE UPDATE
    ON customers
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table comments
(
    id               uuid         not null primary key,
    commentable_type varchar(255) not null,
    commentable_id   uuid         not null,
    comment          text,
    created_by       uuid         not null,
    created_at       timestamptz  not null default now(),
    updated_at       timestamptz  not null default now(),
    deleted_at       timestamptz,
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_comments_created_by ON comments (created_by);
CREATE INDEX idx_comments_created_at ON comments (created_at);
CREATE INDEX idx_comments_updated_at ON comments (updated_at);
CREATE INDEX idx_comments_deleted_at ON comments (deleted_at);

CREATE TRIGGER update_updated_at_on_comments_table
    BEFORE UPDATE
    ON comments
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table countries -- Entry can only be deleted if no data relies on it!
(
    id         uuid         not null primary key,
    name       varchar(100) not null,
    created_by uuid         not null,
    created_at timestamptz  not null default now(),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_countries_created_by ON countries (created_by);
CREATE INDEX idx_countries_created_at ON countries (created_at);

CREATE TRIGGER update_updated_at_on_countries_table
    BEFORE UPDATE
    ON countries
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table states -- Entry can only be deleted if no data relies on it!
(
    id         uuid         not null primary key,
    name       varchar(100) not null,
    created_by uuid         not null,
    created_at timestamptz  not null default now(),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_states_created_by ON states (created_by);
CREATE INDEX idx_states_created_at ON states (created_at);

CREATE TRIGGER update_updated_at_on_states_table
    BEFORE UPDATE
    ON states
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table postal_codes -- Entry can only be deleted if no data relies on it!
(
    id          uuid        not null primary key,
    postal_code varchar(20) not null,
    created_by  uuid        not null,
    created_at  timestamptz not null default now(),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_postal_codes_created_by ON postal_codes (created_by);
CREATE INDEX idx_postal_codes_created_at ON postal_codes (created_at);

create table cities -- Entry can only be deleted if no data relies on it!
(
    id          uuid         not null primary key,
    postal_code uuid         not null,
    name        varchar(100) not null,
    created_by  uuid         not null,
    created_at  timestamptz  not null default now(),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_cities_created_by ON cities (created_by);
CREATE INDEX idx_cities_created_at ON cities (created_at);

create table address
(
    id              uuid primary key      default uuid_generate_v4(),
    street_address  varchar(255) not null,
    city_id         uuid         not null,
    state_id        uuid         not null,
    country_id      uuid         not null,
    additional_info text,
    created_by      uuid         not null,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz  not null default now(),
    deleted_at      timestamptz,
    foreign key (created_by) references users (id),
    foreign key (city_id) references cities (id),
    foreign key (state_id) references states (id),
    foreign key (country_id) references countries (id)
);

CREATE INDEX idx_address_city_id ON address (city_id);
CREATE INDEX idx_address_state_id ON address (state_id);
CREATE INDEX idx_address_country_id ON address (country_id);
CREATE INDEX idx_address_created_by ON address (created_by);
CREATE INDEX idx_address_created_at ON address (created_at);
CREATE INDEX idx_address_updated_at ON address (updated_at);
CREATE INDEX idx_address_deleted_at ON address (deleted_at);

CREATE TRIGGER update_updated_at_on_address_table
    BEFORE UPDATE
    ON address
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table address_connect
(
    id               uuid primary key default uuid_generate_v4(),
    address_id       uuid         not null,
    addressable_id   uuid         not null,
    addressable_type varchar(100) not null,
    created_by       uuid         not null,
    created_at       timestamptz      default now(),
    updated_at       timestamptz      default now(),
    deleted_at       timestamptz,
    foreign key (address_id) references address (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_address_connect_address_id ON address_connect (address_id);
CREATE INDEX idx_address_connect_addressable_type_id ON address_connect (addressable_type, addressable_id);
CREATE INDEX idx_address_connect_created_by ON address_connect (created_by);
CREATE INDEX idx_address_connect_created_at ON address_connect (created_at);
CREATE INDEX idx_address_connect_updated_at ON address_connect (updated_at);
CREATE INDEX idx_address_connect_deleted_at ON address_connect (deleted_at);

CREATE TRIGGER update_updated_at_on_address_connect_table
    BEFORE UPDATE
    ON address_connect
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table tags -- Entry can only be deleted if no data relies on it!
(
    id          uuid primary key      default uuid_generate_v4(),
    name        varchar(255) not null,
    description text,
    created_by  uuid         not null,
    created_at  timestamptz  not null default now(),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_tags_created_by ON tags (created_by);
CREATE INDEX idx_tags_created_at ON tags (created_at);

create table tag_connect
(
    id            uuid         not null primary key,
    taggable_id   uuid         not null,
    taggable_type varchar(255) not null,
    tag_id        uuid,
    created_by    uuid         not null,
    created_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (tag_id) references tags (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_tag_connect_tag_id ON tag_connect (tag_id);
CREATE INDEX idx_tag_connect_taggable_type_id ON tag_connect (taggable_type, taggable_id);
CREATE INDEX idx_tag_connect_created_by ON tag_connect (created_by);
CREATE INDEX idx_tag_connect_created_at ON tag_connect (created_at);
CREATE INDEX idx_tag_connect_deleted_at ON tag_connect (deleted_at);

create table projects
(
    id          uuid         not null primary key,
    name        varchar(255) not null,
    description text,
    created_by  uuid         not null,
    status      varchar(50)  not null default 'planning',
    start_date  timestamptz,
    end_date    timestamptz,
    created_at  timestamptz  not null default now(),
    updated_at  timestamptz  not null default now(),
    deleted_at  timestamptz,
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_projects_created_by ON projects (created_by);
CREATE INDEX idx_projects_created_at ON projects (created_at);
CREATE INDEX idx_projects_updated_at ON projects (updated_at);
CREATE INDEX idx_projects_deleted_at ON projects (deleted_at);

CREATE TRIGGER update_updated_at_on_projects_table
    BEFORE UPDATE
    ON projects
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table worksheets
(
    id          uuid         not null primary key,
    name        varchar(255) not null,
    description text,
    project_id  uuid         not null,
    created_by  uuid         not null,
    status      varchar(50)  not null default 'draft',
    created_at  timestamptz  not null default now(),
    updated_at  timestamptz  not null default now(),
    deleted_at  timestamptz,
    foreign key (project_id) references projects (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_worksheets_project_id ON worksheets (project_id);
CREATE INDEX idx_worksheets_created_by ON worksheets (created_by);
CREATE INDEX idx_worksheets_created_at ON worksheets (created_at);
CREATE INDEX idx_worksheets_updated_at ON worksheets (updated_at);
CREATE INDEX idx_worksheets_deleted_at ON worksheets (deleted_at);

CREATE TRIGGER update_updated_at_on_worksheets_table
    BEFORE UPDATE
    ON worksheets
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table tasks
(
    id           uuid         not null primary key,
    worksheet_id uuid         not null,
    title        varchar(255) not null,
    description  text,
    created_by   uuid         not null,
    status       varchar(50)  not null default 'pending',
    priority     integer,
    due_date     timestamptz,
    created_at   timestamptz  not null default now(),
    updated_at   timestamptz  not null default now(),
    deleted_at   timestamptz,
    foreign key (worksheet_id) references worksheets (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_tasks_worksheet_id ON tasks (worksheet_id);
CREATE INDEX idx_tasks_created_by ON tasks (created_by);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
CREATE INDEX idx_tasks_updated_at ON tasks (updated_at);
CREATE INDEX idx_tasks_deleted_at ON tasks (deleted_at);

CREATE TRIGGER update_updated_at_on_tasks_table
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table currencies
(
    id         uuid primary key     default uuid_generate_v4(),
    currency   varchar(3)  not null,
    created_by uuid        not null,
    created_at timestamptz not null default now(),
    deleted_at timestamptz,
    foreign key (created_by) references users (id),
    unique (currency)
);

CREATE INDEX idx_currencies_created_by ON currencies (created_by);
CREATE INDEX idx_currencies_created_at ON currencies (created_at);

create table units_of_measure
(
    id              uuid primary key     default uuid_generate_v4(),
    unit_of_measure varchar(50) not null,
    created_by      uuid        not null,
    created_at      timestamptz not null default now(),
    deleted_at      timestamptz,
    foreign key (created_by) references users (id),
    unique (unit_of_measure)
);

CREATE INDEX idx_units_of_measure_created_by ON units_of_measure (created_by);
CREATE INDEX idx_units_of_measure_created_at ON units_of_measure (created_at);

create table products
(
    id                 uuid primary key      default uuid_generate_v4(),
    name               varchar(255) not null,
    description        text,
    unit_of_measure_id uuid         not null,
    status             varchar(50)  not null default 'active',
    created_by         uuid         not null,
    created_at         timestamptz           default now(),
    updated_at         timestamptz           default now(),
    deleted_at         timestamptz,
    foreign key (unit_of_measure_id) references units_of_measure (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_products_unit_of_measure_id ON products (unit_of_measure_id);
CREATE INDEX idx_products_created_by ON products (created_by);
CREATE INDEX idx_products_created_at ON products (created_at);
CREATE INDEX idx_products_updated_at ON products (updated_at);
CREATE INDEX idx_products_deleted_at ON products (deleted_at);

CREATE TRIGGER update_updated_at_on_products_table
    BEFORE UPDATE
    ON products
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table product_category
(
    id          uuid primary key      default uuid_generate_v4(),
    name        varchar(255) not null,
    description text,
    parent_id   uuid,
    created_by  uuid         not null,
    created_at  timestamptz  not null default now(),
    foreign key (parent_id) references product_category (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_product_category_parent_id ON product_category (parent_id);
CREATE INDEX idx_product_category_created_by ON product_category (created_by);
CREATE INDEX idx_product_category_created_at ON product_category (created_at);

create table product_category_connect
(
    id                  uuid primary key     default uuid_generate_v4(),
    product_id          uuid        not null,
    product_category_id uuid        not null,
    created_by          uuid        not null,
    created_at          timestamptz not null default now(),
    deleted_at          timestamptz,
    foreign key (product_id) references products (id),
    foreign key (product_category_id) references product_category (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_product_category_connect_product_id ON product_category_connect (product_id);
CREATE INDEX idx_product_category_connect_product_category_id ON product_category_connect (product_category_id);
CREATE INDEX idx_product_category_connect_created_by ON product_category_connect (created_by);
CREATE INDEX idx_product_category_connect_created_at ON product_category_connect (created_at);
CREATE INDEX idx_product_category_connect_deleted_at ON product_category_connect (deleted_at);

create table warehouses
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    contact_name  varchar(255),
    contact_phone varchar(50),
    status        varchar(50)  not null default 'active',
    created_by    uuid         not null,
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_warehouses_created_by ON warehouses (created_by);
CREATE INDEX idx_warehouses_created_at ON warehouses (created_at);
CREATE INDEX idx_warehouses_updated_at ON warehouses (updated_at);
CREATE INDEX idx_warehouses_deleted_at ON warehouses (deleted_at);

CREATE TRIGGER update_updated_at_on_warehouses_table
    BEFORE UPDATE
    ON warehouses
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table inventory
(
    id           uuid primary key     default uuid_generate_v4(),
    product_id   uuid        not null,
    warehouse_id uuid        not null,
    quantity     integer     not null default 0,
    price        numeric(15, 2),
    cost         numeric(15, 2),
    currency_id  uuid        not null,
    created_by   uuid        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz not null default now(),
    deleted_at   timestamptz,
    foreign key (currency_id) references currencies (id),
    foreign key (product_id) references products (id),
    foreign key (warehouse_id) references warehouses (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_inventory_price ON inventory (price);
CREATE INDEX idx_inventory_cost ON inventory (cost);
CREATE INDEX idx_inventory_currency_id ON inventory (currency_id);
CREATE INDEX idx_inventory_product_id ON inventory (product_id);
CREATE INDEX idx_inventory_warehouse_id ON inventory (warehouse_id);
CREATE INDEX idx_inventory_created_by ON inventory (created_by);
CREATE INDEX idx_inventory_created_at ON inventory (created_at);
CREATE INDEX idx_inventory_updated_at ON inventory (updated_at);
CREATE INDEX idx_inventory_deleted_at ON inventory (deleted_at);

CREATE TRIGGER update_updated_at_on_inventory_table
    BEFORE UPDATE
    ON inventory
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table task_assignments
(
    id         uuid primary key default uuid_generate_v4(),
    user_id    uuid                           not null,
    task_id    uuid                           not null,
    created_by uuid                           not null,
    created_at timestamptz      default now() not null,
    deleted_at timestamptz,
    unique (user_id, task_id),
    foreign key (user_id) references users (id),
    foreign key (task_id) references tasks (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_task_assignments_user_id ON task_assignments (user_id);
CREATE INDEX idx_task_assignments_task_idd ON task_assignments (task_id);
CREATE INDEX idx_task_assignments_created_by ON task_assignments (created_by);
CREATE INDEX idx_task_assignments_created_at ON task_assignments (created_at);
CREATE INDEX idx_task_assignments_deleted_at ON task_assignments (deleted_at);

create table project_assignments
(
    id         uuid primary key default uuid_generate_v4(),
    user_id    uuid                           not null,
    project_id uuid                           not null,
    created_by uuid                           not null,
    created_at timestamptz      default now() not null,
    deleted_at timestamptz,
    unique (user_id, project_id),
    foreign key (user_id) references users (id),
    foreign key (project_id) references tasks (id),
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_project_assignments_user_id ON project_assignments (user_id);
CREATE INDEX idx_project_assignments_project_id ON project_assignments (project_id);
CREATE INDEX idx_project_assignments_created_by ON project_assignments (created_by);
CREATE INDEX idx_project_assignments_created_at ON project_assignments (created_at);
CREATE INDEX idx_project_assignments_deleted_at ON project_assignments (deleted_at);
