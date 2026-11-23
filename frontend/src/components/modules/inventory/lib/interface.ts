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
    inventoryUserInputSchema,
    inventoryErrorSchema,
    inventorySchema,
    inventoryResolvedSchema,
    inventoryResolvedListSchema,
    inventoryResponseSchema,
    inventoryResolvedResponseSchema,
    createInventoryResponseSchema,
    updateInventoryResponseSchema,
    deleteInventoryResponseSchema,
    paginatedInventoryResolvedListResponseSchema,
} from "./schemas";

export type InventoryUserInput = z.infer<typeof inventoryUserInputSchema>;
export type InventoryError = z.infer<typeof inventoryErrorSchema>;
export type Inventory = z.infer<typeof inventorySchema>;
export type InventoryResolved = z.infer<typeof inventoryResolvedSchema>;
export type InventoryResolvedList = z.infer<typeof inventoryResolvedListSchema>;
export type InventoryResponse = z.infer<typeof inventoryResponseSchema>;
export type InventoryResolvedResponse = z.infer<
    typeof inventoryResolvedResponseSchema
>;
export type CreateInventoryResponse = z.infer<
    typeof createInventoryResponseSchema
>;
export type UpdateInventoryResponse = z.infer<
    typeof updateInventoryResponseSchema
>;
export type DeleteInventoryResponse = z.infer<
    typeof deleteInventoryResponseSchema
>;
export type PaginatedInventoryResolvedListResponse = z.infer<
    typeof paginatedInventoryResolvedListResponseSchema
>;
