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

create table task_assignments
(
    id            uuid primary key default uuidv7(),
    user_id       uuid                           not null,
    task_id       uuid                           not null,
    created_by_id uuid                           not null,
    created_at    timestamptz      default now() not null,
    deleted_at    timestamptz,
    unique nulls not distinct (user_id, task_id, deleted_at),
    foreign key (user_id) references users (id),
    foreign key (task_id) references tasks (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_task_assignments_user_id ON task_assignments (user_id);
CREATE INDEX idx_task_assignments_task_idd ON task_assignments (task_id);
CREATE INDEX idx_task_assignments_created_by_id ON task_assignments (created_by_id);
CREATE INDEX idx_task_assignments_created_at ON task_assignments (created_at);
CREATE INDEX idx_task_assignments_deleted_at ON task_assignments (deleted_at);

create table project_assignments
(
    id            uuid primary key default uuidv7(),
    user_id       uuid                           not null,
    project_id    uuid                           not null,
    created_by_id uuid                           not null,
    created_at    timestamptz      default now() not null,
    deleted_at    timestamptz,
    unique nulls not distinct (user_id, project_id, deleted_at),
    foreign key (user_id) references users (id),
    foreign key (project_id) references tasks (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_project_assignments_user_id ON project_assignments (user_id);
CREATE INDEX idx_project_assignments_project_id ON project_assignments (project_id);
CREATE INDEX idx_project_assignments_created_by_id ON project_assignments (created_by_id);
CREATE INDEX idx_project_assignments_created_at ON project_assignments (created_at);
CREATE INDEX idx_project_assignments_deleted_at ON project_assignments (deleted_at);

ALTER TABLE tasks
DROP CONSTRAINT IF EXISTS tasks_worksheet_id_fkey;
