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
    inventoryMovementUserInputSchema,
    inventoryMovementErrorSchema,
    inventoryMovementSchema,
    inventoryMovementResolvedSchema,
    inventoryMovementResolvedListSchema,
    inventoryMovementResponseSchema,
    inventoryMovementResolvedResponseSchema,
    createInventoryMovementResponseSchema,
    deleteInventoryMovementResponseSchema,
    paginatedInventoryMovementResolvedListResponseSchema,
} from "./schemas";

export type InventoryMovementUserInput = z.infer<
    typeof inventoryMovementUserInputSchema
>;
export type InventoryMovementError = z.infer<
    typeof inventoryMovementErrorSchema
>;
export type InventoryMovement = z.infer<typeof inventoryMovementSchema>;
export type InventoryMovementResolved = z.infer<
    typeof inventoryMovementResolvedSchema
>;
export type InventoryMovementResolvedList = z.infer<
    typeof inventoryMovementResolvedListSchema
>;
export type InventoryMovementResponse = z.infer<
    typeof inventoryMovementResponseSchema
>;
export type InventoryMovementResolvedResponse = z.infer<
    typeof inventoryMovementResolvedResponseSchema
>;
export type CreateInventoryMovementResponse = z.infer<
    typeof createInventoryMovementResponseSchema
>;
export type DeleteInventoryMovementResponse = z.infer<
    typeof deleteInventoryMovementResponseSchema
>;
export type PaginatedInventoryMovementResolvedListResponse = z.infer<
    typeof paginatedInventoryMovementResolvedListResponseSchema
>;
