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
    createWarehouseResponseSchema,
    deleteWarehouseResponseSchema,
    paginatedWarehouseResolvedListResponseSchema,
    updateWarehouseResponseSchema,
    warehouseErrorSchema,
    warehouseResolvedListSchema,
    warehouseResolvedResponseSchema,
    warehouseResolvedSchema,
    warehouseResponseSchema,
    warehouseSchema,
    warehouseUserInputSchema,
} from "./schemas";

export type WarehouseUserInput = z.infer<typeof warehouseUserInputSchema>;
export type WarehouseError = z.infer<typeof warehouseErrorSchema>;
export type Warehouse = z.infer<typeof warehouseSchema>;
export type WarehouseResolved = z.infer<typeof warehouseResolvedSchema>;
export type CreateWarehouseResponse = z.infer<
    typeof createWarehouseResponseSchema
>;
export type UpdateWarehouseResponse = z.infer<
    typeof updateWarehouseResponseSchema
>;
export type DeleteWarehouseResponse = z.infer<
    typeof deleteWarehouseResponseSchema
>;
export type WarehouseResponse = z.infer<typeof warehouseResponseSchema>;
export type WarehouseResolvedResponse = z.infer<
    typeof warehouseResolvedResponseSchema
>;
export type WarehouseResolvedList = z.infer<typeof warehouseResolvedListSchema>;
export type PaginatedWarehouseResolvedListResponse = z.infer<
    typeof paginatedWarehouseResolvedListResponseSchema
>;
