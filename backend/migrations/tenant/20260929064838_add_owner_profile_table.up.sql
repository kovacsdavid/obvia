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

create table owner_profile
(
    id                  uuid primary key    default uuidv7(),
    name                varchar(255)        not null,
    contact_name        varchar(255),
    email               varchar(255)        not null,
    website             varchar(255),
    phone_number        varchar(50),
    owner_profile_type  varchar(50),
    billing_address     uuid,
    mailing_address     uuid,
    created_at    timestamptz               not null default now(),
    updated_at    timestamptz               not null default now()
);

CREATE TRIGGER update_updated_at_on_owner_profile_table
    BEFORE UPDATE
    ON owner_profile
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

INSERT INTO owner_profile(name, contact_name, email, website, phone_number, owner_profile_type)
VALUES ('', '', '', '', '', '');


