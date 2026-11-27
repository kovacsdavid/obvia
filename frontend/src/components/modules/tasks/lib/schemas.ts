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

import { z } from "zod";
import {
    paginatedSuccessResponseSchema,
    successResponseSchema,
} from "@/lib/api/schemas";

const nullableString = z.string().nullable();

export const taskUserInputSchema = z.looseObject({
    id: nullableString,
    worksheet_id: z.string(),
    service_id: z.string(),
    currency_code: z.string(),
    quantity: z.string(),
    price: z.string(),
    tax_id: z.string(),
    status: z.string(),
    priority: nullableString,
    due_date: nullableString,
    description: nullableString,
});

export const taskFormSchema = z
    .object(taskUserInputSchema.shape)
    .omit({ id: true, quantity: true, price: true })
    .extend({ description: z.string() });

export const taskSchema = z.looseObject({
    id: z.string(),
    worksheet_id: z.string(),
    service_id: z.string(),
    currency_code: z.string(),
    quantity: nullableString,
    price: nullableString,
    tax_id: z.string(),
    created_by_id: z.string(),
    status: z.string(),
    priority: nullableString,
    due_date: nullableString,
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
    description: nullableString,
});

export const taskResolvedSchema = taskSchema.extend({
    worksheet: z.string(),
    service: z.string(),
    tax: z.string(),
    created_by: z.string(),
});

export const taskErrorsSchema = z.looseObject({
    id: nullableString,
    worksheet_id: nullableString,
    service_id: nullableString,
    currency_code: nullableString,
    quantity: nullableString,
    price: nullableString,
    tax_id: nullableString,
    status: nullableString,
    priority: nullableString,
    due_date: nullableString,
    description: nullableString,
});

export const taskResolvedListSchema = z.array(taskResolvedSchema);
export const taskResponseSchema = successResponseSchema(taskSchema).refine(
    (value) => !("error" in value),
);
export const taskResolvedResponseSchema = successResponseSchema(
    taskResolvedSchema,
).refine((value) => !("error" in value));
export const createTaskResponseSchema = taskResponseSchema;
export const updateTaskResponseSchema = taskResponseSchema;
export const deleteTaskResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
).refine((value) => !("error" in value));
export const paginatedTaskResolvedListResponseSchema =
    paginatedSuccessResponseSchema(taskResolvedListSchema).refine(
        (value) => !("error" in value),
    );
