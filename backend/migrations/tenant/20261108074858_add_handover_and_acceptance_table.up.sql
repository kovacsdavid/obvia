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

create table item_identifier_types (
    id          uuid            primary key default uuidv7(),
    name        varchar(100)    not null unique,
    is_active   boolean         not null default true,
    created_at  timestamptz     not null default now()
);

create table item_makes (
    id          uuid            primary key default uuidv7(),
    name        varchar(100)    not null unique,
    is_active   boolean         not null default true,
    created_at  timestamptz     not null default now()
);

create table item_models (
    id          uuid            primary key default uuidv7(),
    make_id     uuid            not null,
    name        varchar(100)    not null unique,
    is_active   boolean         not null default true,
    created_at  timestamptz     not null default now(),
    foreign key (make_id) references item_makes(id)
);

create index idx_item_models_item_makes ON item_models (make_id);

create table handover_and_acceptance 
(
    id                          uuid            primary key default uuidv7(),
    display_id                  varchar(255)    not null,
    display_year                int             not null,
    display_seq                 int             not null,
    customer_id                 uuid            not null,
    item_make_id                uuid,
    item_model_id               uuid,
    item_identifier_type_id     uuid            not null,
    item_identifier_value       varchar(100)    not null,
    reported_issue              text,
    condition_at_receipt        text,
    included_accessories        text[],
    authorized_work             text,
    repair_limit                numeric(15, 2),
    currency_code               varchar(3),
    terms_version               varchar(100),
    accepted_by                 uuid            not null,
    received_at                 timestamptz     not null,
    created_at                  timestamptz     not null default now(),
    updated_at                  timestamptz     not null default now(),
    deleted_at                  timestamptz,
    foreign key (customer_id) references customers (id),
    foreign key (item_make_id) references item_makes (id),
    foreign key (item_model_id) references  item_models (id),
    foreign key (item_identifier_type_id) references item_identifier_types (id),
    foreign key (currency_code) references currencies (code),
    foreign key (accepted_by) references users (id)
);

create index idx_handover_and_acceptance_customer_id ON handover_and_acceptance (customer_id);
create index idx_handover_and_acceptance_item_make_id ON handover_and_acceptance (item_make_id);
create index idx_handover_and_acceptance_item_model_id ON handover_and_acceptance (item_model_id);
create index idx_handover_and_acceptance_item_identifier_type_id 
ON handover_and_acceptance (item_identifier_type_id);
create index idx_handover_and_acceptance_currency_code ON handover_and_acceptance (currency_code);
create index idx_handover_and_acceptance_accepted_by ON handover_and_acceptance (accepted_by);


CREATE TRIGGER update_updated_at_on_handover_and_acceptance_table
    BEFORE UPDATE
    ON handover_and_acceptance
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();
