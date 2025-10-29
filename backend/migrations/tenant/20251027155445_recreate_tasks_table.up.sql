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

DROP TABLE IF EXISTS tasks CASCADE;

create table tasks
(
    id            uuid primary key        default uuid_generate_v4(),
    worksheet_id  uuid           not null,
    service_id    uuid           not null,
    currency_code varchar(3)     not null,
    quantity      numeric(15, 2) not null,
    price         numeric(15, 2),
    tax_id        uuid           not null,
    created_by_id uuid           not null,
    status        varchar(50)    not null default 'pending',
    priority      varchar(50),
    due_date      timestamptz,
    created_at    timestamptz    not null default now(),
    updated_at    timestamptz    not null default now(),
    deleted_at    timestamptz,
    description   text,
    foreign key (worksheet_id) references worksheets (id),
    foreign key (service_id) references services (id),
    foreign key (created_by_id) references users (id),
    foreign key (tax_id) references taxes (id),
    foreign key (currency_code) references currencies (code)
);

CREATE INDEX idx_tasks_worksheet_id ON tasks (worksheet_id);
CREATE INDEX idx_tasks_service_id ON tasks (service_id);
CREATE INDEX idx_tasks_currency_code ON tasks (currency_code);
CREATE INDEX idx_tasks_quantity ON tasks (quantity);
CREATE INDEX idx_tasks_tax_id ON tasks (tax_id);
CREATE INDEX idx_tasks_created_by_id ON tasks (created_by_id);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
CREATE INDEX idx_tasks_updated_at ON tasks (updated_at);
CREATE INDEX idx_tasks_deleted_at ON tasks (deleted_at);

CREATE TRIGGER update_updated_at_on_tasks_table
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
