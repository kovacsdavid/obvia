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

CREATE TABLE refresh_tokens (
    id              uuid        primary key default uuid_generate_v4(),
    user_id         uuid        not null,
    family_id       uuid        not null,
    jti             uuid        not null unique,
    iat             timestamptz not null,
    exp             timestamptz not null,
    replaced_by     uuid,
    consumed_at     timestamptz,
    revoked_at      timestamptz,
    foreign key (user_id) references users (id),
    foreign key (replaced_by) references refresh_tokens (jti)
);

CREATE INDEX idx_refresh_tokens_jti ON refresh_tokens (jti);
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens (user_id);
CREATE INDEX idx_refresh_tokens_family_id ON refresh_tokens (family_id);
CREATE INDEX idx_refresh_tokens_exp ON refresh_tokens (exp);

