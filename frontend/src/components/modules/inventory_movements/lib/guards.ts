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

import {
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  isSimpleMessageData
} from "@/lib/interfaces/common.ts";
import type {
  CreateInventoryMovementResponse,
  DeleteInventoryMovementResponse,
  InventoryMovement,
  InventoryMovementResolved,
  InventoryMovementResolvedList,
  InventoryMovementResolvedResponse,
  InventoryMovementResponse,
  PaginatedInventoryMovementResolvedListResponse
} from "@/components/modules/inventory_movements/lib/interface.ts";

export function isInventoryMovementResolved(data: unknown): data is InventoryMovementResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "inventory_id" in data &&
    typeof data.inventory_id === "string" &&
    "movement_type" in data &&
    typeof data.movement_type === "string" &&
    "quantity" in data &&
    typeof data.quantity === "number" &&
    "reference_type" in data &&
    (data.reference_type === null || typeof data.reference_type === "string") &&
    "reference_id" in data &&
    (data.reference_id === null || typeof data.reference_id === "string") &&
    "unit_price" in data &&
    (data.unit_price === null || typeof data.unit_price === "string") &&
    "total_price" in data &&
    (data.total_price === null || typeof data.total_price === "string") &&
    "tax_id" in data &&
    typeof data.tax_id === "string" &&
    "tax" in data &&
    (data.tax === null || typeof data.tax === "string") &&
    "movement_date" in data &&
    typeof data.movement_date === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string"
  );
}

export function isInventoryMovementResolvedResponse(data: unknown): data is InventoryMovementResolvedResponse {
  return isCommonResponse(
    data,
    isInventoryMovementResolved,
    isSimpleError,
  );
}

export function isInventoryMovement(data: unknown): data is InventoryMovement {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "inventory_id" in data &&
    typeof data.inventory_id === "string" &&
    "movement_type" in data &&
    typeof data.movement_type === "string" &&
    "quantity" in data &&
    typeof data.quantity === "number" &&
    "reference_type" in data &&
    (data.reference_type === null || typeof data.reference_type === "string") &&
    "reference_id" in data &&
    (data.reference_id === null || typeof data.reference_id === "string") &&
    "unit_price" in data &&
    (data.unit_price === null || typeof data.unit_price === "string") &&
    "total_price" in data &&
    (data.total_price === null || typeof data.total_price === "string") &&
    "tax_id" in data &&
    typeof data.tax_id === "string" &&
    "movement_date" in data &&
    typeof data.movement_date === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string"
  );
}

export function isInventoryMovementResponse(data: unknown): data is InventoryMovementResponse {
  return isCommonResponse(
    data,
    isInventoryMovement,
    isSimpleError,
  );
}

export function isInventoryMovementResolvedList(data: unknown): data is InventoryMovementResolvedList {
  return Array.isArray(data) && data.every(item => isInventoryMovementResolved(item));
}

export function isPaginatedInventoryMovementResolvedListResponse(data: unknown): data is PaginatedInventoryMovementResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isInventoryMovementResolvedList,
  );
}

export function isCreateInventoryMovementResponse(data: unknown): data is CreateInventoryMovementResponse {
  return isCommonResponse(
    data,
    isInventoryMovement,
    isFormError,
  );
}

export function isDeleteInventoryMovementResponse(data: unknown): data is DeleteInventoryMovementResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError,
  );
}
