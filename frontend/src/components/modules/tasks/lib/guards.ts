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

import { schemaGuard } from "@/lib/api/schemas";
import {
    createTaskResponseSchema,
    deleteTaskResponseSchema,
    paginatedTaskResolvedListResponseSchema,
    taskErrorsSchema,
    taskResolvedListSchema,
    taskResolvedResponseSchema,
    taskResolvedSchema,
    taskResponseSchema,
    taskSchema,
    updateTaskResponseSchema,
} from "./schemas";

export const isCreateTaskResponse = schemaGuard(createTaskResponseSchema);
export const isUpdateTaskResponse = schemaGuard(updateTaskResponseSchema);
export const isDeleteTaskResponse = schemaGuard(deleteTaskResponseSchema);
export const isTask = schemaGuard(taskSchema);
export const isTaskErrors = schemaGuard(taskErrorsSchema);
export const isTaskResolved = schemaGuard(taskResolvedSchema);
export const isTaskResolvedList = schemaGuard(taskResolvedListSchema);
export const isTaskResponse = schemaGuard(taskResponseSchema);
export const isTaskResolvedResponse = schemaGuard(taskResolvedResponseSchema);
export const isPaginatedTaskResolvedListResponse = schemaGuard(
    paginatedTaskResolvedListResponseSchema,
);
