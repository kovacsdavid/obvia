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

DROP TABLE IF EXISTS worksheets CASCADE;

create table worksheets
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    customer_id   uuid,
    project_id    uuid,
    created_by_id uuid         not null,
    status        varchar(50)  not null default 'draft',
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    description   text,
    foreign key (customer_id) references customers (id),
    foreign key (project_id) references projects (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_worksheets_customer_id ON worksheets (customer_id);
CREATE INDEX idx_worksheets_project_id ON worksheets (project_id);
CREATE INDEX idx_worksheets_created_by_id ON worksheets (created_by_id);
CREATE INDEX idx_worksheets_created_at ON worksheets (created_at);
CREATE INDEX idx_worksheets_updated_at ON worksheets (updated_at);
CREATE INDEX idx_worksheets_deleted_at ON worksheets (deleted_at);

CREATE TRIGGER update_updated_at_on_worksheets_table
    BEFORE UPDATE
    ON worksheets
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
