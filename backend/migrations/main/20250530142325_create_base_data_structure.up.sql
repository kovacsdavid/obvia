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

-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

create table users (
    id uuid primary key default uuid_generate_v4(),
    email varchar(255) not null unique,
    password_hash varchar(255) not null,
    first_name varchar(255),
    last_name varchar(255),
    phone varchar(32),
    -- possible values e.g.: 'active', 'invited', 'pending', 'suspended', 'disabled', 'locked'
    status varchar(32) not null default 'active',
    last_login_at timestamptz,
    profile_picture_url text,
    locale varchar(16) not null default 'hu-HU',
    invited_by uuid,
    email_verified_at timestamptz,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    deleted_at timestamptz,
    foreign key (invited_by) references users (id)
);

CREATE INDEX idx_users_invited_by ON users (invited_by);
CREATE INDEX idx_users_created_at ON users (created_at);
CREATE INDEX idx_users_updated_at ON users (updated_at);
CREATE INDEX idx_users_deleted_at ON users (deleted_at);

create table tenants
(
    id               uuid primary key      default uuid_generate_v4(),
    name             varchar(255) not null,
    is_self_hosted   boolean      not null,
    db_host          varchar(255) not null,
    db_port          integer      not null default 5432,
    db_name          varchar(255) not null,
    db_user          varchar(255) not null,
    db_password      varchar(255) not null,
    db_max_pool_size integer      not null,
    db_ssl_mode      varchar(255) not null,
    created_by       uuid         not null,
    created_at       timestamptz  not null default now(),
    updated_at       timestamptz  not null default now(),
    deleted_at       timestamptz,
    foreign key (created_by) references users (id)
);

CREATE INDEX idx_tenants_created_by ON tenants (created_by);
CREATE INDEX idx_tenants_created_at ON tenants (created_at);
CREATE INDEX idx_tenants_updated_at ON tenants (updated_at);
CREATE INDEX idx_tenants_deleted_at ON tenants (deleted_at);

create table user_tenants
(
    id             uuid primary key     default uuid_generate_v4(),
    user_id        uuid        not null,
    tenant_id      uuid        not null,
    status         varchar(32) not null default 'pending',
    role           varchar(32) not null default 'member',
    invited_by     uuid,
    last_activated timestamptz not null default now(),
    created_at     timestamptz not null default now(),
    updated_at     timestamptz not null default now(),
    deleted_at     timestamptz,
    foreign key (user_id) references users (id),
    foreign key (invited_by) references users (id),
    foreign key (tenant_id) references tenants (id),
    unique (user_id, tenant_id)
);

CREATE INDEX idx_user_tenants_user_idy ON user_tenants (user_id);
CREATE INDEX idx_user_tenants_invited_by ON user_tenants (invited_by);
CREATE INDEX idx_user_tenants_tenant_idy ON user_tenants (tenant_id);
CREATE INDEX idx_user_tenants_created_at ON user_tenants (created_at);
CREATE INDEX idx_user_tenants_updated_at ON user_tenants (updated_at);
CREATE INDEX idx_user_tenants_deleted_at ON user_tenants (deleted_at);
