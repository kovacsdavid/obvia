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

DROP TRIGGER IF EXISTS update_inventory_on_reservation ON inventory_reservations;
DROP TRIGGER IF EXISTS update_inventory_on_movement ON inventory_movements;
DROP TRIGGER IF EXISTS update_updated_at_on_inventory_reservations_table ON inventory_reservations;

DROP FUNCTION IF EXISTS cleanup_expired_reservations();
DROP FUNCTION IF EXISTS update_reserved_quantities();
DROP FUNCTION IF EXISTS update_inventory_quantities();

DROP TABLE IF EXISTS inventory_reservations;
DROP TABLE IF EXISTS inventory_movements;
DROP TABLE IF EXISTS inventory;

CREATE TABLE inventory
(
    id            uuid primary key     default uuid_generate_v4(),
    product_id    uuid        not null,
    warehouse_id  uuid        not null,
    quantity      integer     not null default 0,
    price         numeric(15, 2),
    tax_id        uuid        not null,
    currency_code varchar(3)  not null,
    created_by_id uuid        not null,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    deleted_at    timestamptz,
    foreign key (currency_code) references currencies (code),
    foreign key (product_id) references products (id),
    foreign key (warehouse_id) references warehouses (id),
    foreign key (created_by_id) references users (id),
    foreign key (tax_id) references taxes (id)
);

CREATE INDEX idx_inventory_price ON inventory (price);
CREATE INDEX idx_inventory_tax_id ON inventory (tax_id);
CREATE INDEX idx_inventory_currency_code ON inventory (currency_code);
CREATE INDEX idx_inventory_product_id ON inventory (product_id);
CREATE INDEX idx_inventory_warehouse_id ON inventory (warehouse_id);
CREATE INDEX idx_inventory_created_by_id ON inventory (created_by_id);
CREATE INDEX idx_inventory_created_at ON inventory (created_at);
CREATE INDEX idx_inventory_updated_at ON inventory (updated_at);
CREATE INDEX idx_inventory_deleted_at ON inventory (deleted_at);

CREATE TRIGGER update_updated_at_on_inventory_table
    BEFORE UPDATE
    ON inventory
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
