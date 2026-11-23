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

import type { z } from "zod";
import type {
    createWorksheetResponseSchema,
    deleteWorksheetResponseSchema,
    paginatedWorksheetResolvedListResponseSchema,
    updateWorksheetResponseSchema,
    worksheetErrorSchema,
    worksheetResolvedListSchema,
    worksheetResolvedResponseSchema,
    worksheetResolvedSchema,
    worksheetResponseSchema,
    worksheetSchema,
    worksheetUserInputSchema,
} from "./schemas";

export type WorksheetUserInput = z.infer<typeof worksheetUserInputSchema>;
export type Worksheet = z.infer<typeof worksheetSchema>;
export type WorksheetError = z.infer<typeof worksheetErrorSchema>;
export type WorksheetResolved = z.infer<typeof worksheetResolvedSchema>;
export type WorksheetResolvedList = z.infer<typeof worksheetResolvedListSchema>;
export type CreateWorksheetResponse = z.infer<
    typeof createWorksheetResponseSchema
>;
export type UpdateWorksheetResponse = z.infer<
    typeof updateWorksheetResponseSchema
>;
export type DeleteWorksheetResponse = z.infer<
    typeof deleteWorksheetResponseSchema
>;
export type WorksheetResponse = z.infer<typeof worksheetResponseSchema>;
export type WorksheetResolvedResponse = z.infer<
    typeof worksheetResolvedResponseSchema
>;
export type PaginatedWorksheetResolvedListResponse = z.infer<
    typeof paginatedWorksheetResolvedListResponseSchema
>;
