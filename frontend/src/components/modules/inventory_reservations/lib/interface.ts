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
    inventoryReservationUserInputSchema,
    inventoryReservationErrorSchema,
    inventoryReservationSchema,
    inventoryReservationResolvedSchema,
    inventoryReservationResolvedListSchema,
    inventoryReservationResponseSchema,
    inventoryReservationResolvedResponseSchema,
    createInventoryReservationResponseSchema,
    deleteInventoryReservationResponseSchema,
    paginatedInventoryReservationResolvedListResponseSchema,
} from "./schemas";

export type InventoryReservationUserInput = z.infer<
    typeof inventoryReservationUserInputSchema
>;
export type InventoryReservationError = z.infer<
    typeof inventoryReservationErrorSchema
>;
export type InventoryReservation = z.infer<typeof inventoryReservationSchema>;
export type InventoryReservationResolved = z.infer<
    typeof inventoryReservationResolvedSchema
>;
export type InventoryReservationResolvedList = z.infer<
    typeof inventoryReservationResolvedListSchema
>;
export type InventoryReservationResponse = z.infer<
    typeof inventoryReservationResponseSchema
>;
export type InventoryReservationResolvedResponse = z.infer<
    typeof inventoryReservationResolvedResponseSchema
>;
export type CreateInventoryReservationResponse = z.infer<
    typeof createInventoryReservationResponseSchema
>;
export type DeleteInventoryReservationResponse = z.infer<
    typeof deleteInventoryReservationResponseSchema
>;
export type PaginatedInventoryReservationResolvedListResponse = z.infer<
    typeof paginatedInventoryReservationResolvedListResponseSchema
>;
