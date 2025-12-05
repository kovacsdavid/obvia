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

import type {
  CommonResponse,
  FormError,
  PaginatedDataResponse,
  SimpleError,
  SimpleMessageData,
} from "@/lib/interfaces/common.ts";

export interface InventoryMovementUserInput {
  id: string | null;
  inventoryId: string;
  movementType: string;
  quantity: string;
  referenceType: string | null;
  referenceId: string | null;
  unitPrice: string | null;
  totalPrice: string | null;
  taxId: string;
}

export interface InventoryMovement {
  id: string;
  inventory_id: string;
  movement_type: string;
  quantity: string;
  reference_type: string | null;
  reference_id: string | null;
  unit_price: string | null; // BigDecimal serialized as string
  total_price: string | null; // BigDecimal serialized as string
  tax_id: string;
  movement_date: string;
  created_by_id: string;
  created_at: string;
}

export interface InventoryMovementResolved {
  id: string;
  inventory_id: string;
  movement_type: string;
  quantity: string;
  reference_type: string | null;
  reference_id: string | null;
  unit_price: string | null; // BigDecimal serialized as string
  total_price: string | null; // BigDecimal serialized as string
  tax_id: string;
  tax: string | null;
  movement_date: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
}

export type CreateInventoryMovementResponse = CommonResponse<
  InventoryMovement,
  FormError
>;
export type DeleteInventoryMovementResponse = CommonResponse<
  SimpleMessageData,
  SimpleError
>;
export type InventoryMovementResponse = CommonResponse<
  InventoryMovement,
  SimpleError
>;
export type InventoryMovementResolvedResponse = CommonResponse<
  InventoryMovementResolved,
  SimpleError
>;
export type InventoryMovementResolvedList = InventoryMovementResolved[];
export type PaginatedInventoryMovementResolvedListResponse =
  PaginatedDataResponse<InventoryMovementResolvedList, SimpleError>;
