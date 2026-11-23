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

export const worksheetUserInputSchema = z.looseObject({
    id: nullableString,
    description: z.string(),
    customerId: z.string(),
    projectId: z.string(),
    status: z.string(),
});

export const worksheetSchema = z.looseObject({
    id: z.string(),
    display_id: z.string(),
    description: nullableString,
    customer_id: z.string(),
    project_id: nullableString,
    created_by_id: z.string(),
    status: z.string(),
    created_at: z.string(),
    updated_at: z.string(),
    deleted_at: nullableString,
});

export const worksheetResolvedSchema = worksheetSchema.extend({
    customer: z.string(),
    project: nullableString,
    created_by: z.string(),
    net_material_cost: z.string(),
    gross_material_cost: z.string(),
    net_work_cost: z.string(),
    gross_work_cost: z.string(),
});

export const worksheetErrorSchema = z.looseObject({
    id: nullableString,
    description: nullableString,
    customer_id: nullableString,
    project_id: nullableString,
    status: nullableString,
});

export const worksheetResolvedListSchema = z.array(worksheetResolvedSchema);
export const worksheetResponseSchema = successResponseSchema(
    worksheetSchema,
).refine((value) => !("error" in value));
export const worksheetResolvedResponseSchema = successResponseSchema(
    worksheetResolvedSchema,
).refine((value) => !("error" in value));
export const createWorksheetResponseSchema = worksheetResponseSchema;
export const updateWorksheetResponseSchema = worksheetResponseSchema;
export const deleteWorksheetResponseSchema = successResponseSchema(
    z.looseObject({ message: z.string() }),
).refine((value) => !("error" in value));
export const paginatedWorksheetResolvedListResponseSchema =
    paginatedSuccessResponseSchema(worksheetResolvedListSchema).refine(
        (value) => !("error" in value),
    );
