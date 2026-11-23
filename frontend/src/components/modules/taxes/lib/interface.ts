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
    createTaxResponseSchema,
    deleteTaxResponseSchema,
    paginatedTaxResolvedListResponseSchema,
    taxErrorSchema,
    taxResolvedListSchema,
    taxResolvedResponseSchema,
    taxResolvedSchema,
    taxResponseSchema,
    taxSchema,
    taxUserInputSchema,
    updateTaxResponseSchema,
} from "./schemas";

export type TaxUserInput = z.infer<typeof taxUserInputSchema>;
export type TaxError = z.infer<typeof taxErrorSchema>;
export type Tax = z.infer<typeof taxSchema>;
export type TaxResolved = z.infer<typeof taxResolvedSchema>;
export type CreateTaxResponse = z.infer<typeof createTaxResponseSchema>;
export type UpdateTaxResponse = z.infer<typeof updateTaxResponseSchema>;
export type DeleteTaxResponse = z.infer<typeof deleteTaxResponseSchema>;
export type TaxResolvedList = z.infer<typeof taxResolvedListSchema>;
export type TaxResponse = z.infer<typeof taxResponseSchema>;
export type TaxResolvedResponse = z.infer<typeof taxResolvedResponseSchema>;
export type PaginatedTaxResolvedListResponse = z.infer<
    typeof paginatedTaxResolvedListResponseSchema
>;
