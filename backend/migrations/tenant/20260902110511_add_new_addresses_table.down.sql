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

ALTER TABLE customers DROP COLUMN IF EXISTS mailing_address;
ALTER TABLE customers DROP COLUMN IF EXISTS billing_address;

DROP TABLE IF EXISTS address;

CREATE TABLE address
(
    id              uuid primary key      default uuid_generate_v4(),
    street_address  varchar(255) not null,
    city_id         uuid         not null,
    state_id        uuid         not null,
    country_code    varchar(2)   not null,
    additional_info text,
    created_by_id   uuid         not null,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz  not null default now(),
    deleted_at      timestamptz,
    foreign key (created_by_id) references users (id),
    foreign key (city_id) references cities (id),
    foreign key (state_id) references states (id),
    foreign key (country_code) references countries (code)
);

CREATE INDEX idx_address_city_id ON address (city_id);
CREATE INDEX idx_address_state_id ON address (state_id);
CREATE INDEX idx_address_country_code ON address (country_code);
CREATE INDEX idx_address_created_by_id ON address (created_by_id);
CREATE INDEX idx_address_created_at ON address (created_at);
CREATE INDEX idx_address_updated_at ON address (updated_at);
CREATE INDEX idx_address_deleted_at ON address (deleted_at);

CREATE TRIGGER update_updated_at_on_address_table
    BEFORE UPDATE
    ON address
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TABLE address_connect
(
    id               uuid primary key default uuid_generate_v4(),
    address_id       uuid         not null,
    addressable_id   uuid         not null,
    addressable_type varchar(100) not null,
    created_by_id    uuid         not null,
    created_at       timestamptz      default now(),
    updated_at       timestamptz      default now(),
    deleted_at       timestamptz,
    foreign key (address_id) references address (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_address_connect_address_id ON address_connect (address_id);
CREATE INDEX idx_address_connect_addressable_type_id ON address_connect (addressable_type, addressable_id);
CREATE INDEX idx_address_connect_created_by_id ON address_connect (created_by_id);
CREATE INDEX idx_address_connect_created_at ON address_connect (created_at);
CREATE INDEX idx_address_connect_updated_at ON address_connect (updated_at);
CREATE INDEX idx_address_connect_deleted_at ON address_connect (deleted_at);

CREATE TRIGGER update_updated_at_on_address_connect_table
    BEFORE UPDATE
    ON address_connect
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
