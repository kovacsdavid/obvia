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

CREATE TABLE account_event_log (
    id uuid primary key default uuid_generate_v4(),
    user_id uuid references users(id),
    identifier varchar(255),
    event_type varchar(64) not null,
    status varchar(32) not null,
    ip_address inet,
    user_agent text,
    metadata jsonb,
    created_at timestamptz not null default now(),
);

CREATE INDEX idx_account_event_log_user_id ON account_event_log(user_id);
CREATE INDEX idx_account_event_log_ip_address ON account_event_log(ip_address);
CREATE INDEX idx_account_event_log_event_type ON account_event_log(event_type);
CREATE INDEX idx_account_event_log_created_at ON account_event_log(created_at);
