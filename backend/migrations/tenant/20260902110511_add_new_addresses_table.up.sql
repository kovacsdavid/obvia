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


DROP INDEX idx_address_connect_address_id;
DROP INDEX idx_address_connect_addressable_type_id;
DROP INDEX idx_address_connect_created_by_id;
DROP INDEX idx_address_connect_created_at;
DROP INDEX idx_address_connect_updated_at;
DROP INDEX idx_address_connect_deleted_at;

DROP TRIGGER update_updated_at_on_address_connect_table ON address_connect;

DROP TABLE address_connect;

DROP INDEX idx_address_city_id;
DROP INDEX idx_address_state_id;
DROP INDEX idx_address_country_code;
DROP INDEX idx_address_created_by_id;
DROP INDEX idx_address_created_at;
DROP INDEX idx_address_updated_at;
DROP INDEX idx_address_deleted_at;

DROP TRIGGER update_updated_at_on_address_table ON address;

DROP TABLE address;

CREATE TABLE address (
    id                      uuid            primary key default uuid_generate_v4(),
    type                    varchar(50),
    country_code            varchar(2)      not null,
    postal_code             varchar(20)     not null,
    settlement              varchar(255)    not null,
    mailbox                 varchar(100),
    topographic_number      varchar(100),
    name_of_public_space    varchar(255),
    type_of_public_space    varchar(100),
    house_number            varchar(50),
    building                varchar(20),
    stairway                varchar(20),
    floor                   varchar(20),
    door                    varchar(20),
    created_by_id           uuid            not null,
    created_at              timestamptz     not null default now(),
    updated_at              timestamptz     not null default now(),
    deleted_at              timestamptz,
    foreign key (created_by_id) references users (id),
    foreign key (country_code) references countries (code)
);

CREATE INDEX idx_address_created_by_id ON address (created_by_id);
CREATE INDEX idx_address_created_at ON address (created_at);
CREATE INDEX idx_address_updated_at ON address (updated_at);
CREATE INDEX idx_address_deleted_at ON address (deleted_at);

ALTER TABLE customers ADD COLUMN billing_address uuid;
ALTER TABLE customers ADD COLUMN mailing_address uuid;

