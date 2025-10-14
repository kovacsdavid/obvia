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
  CreateInventoryResponse,
  DeleteInventoryResponse,
  Inventory,
  InventoryResolved,
  InventoryResolvedList,
  InventoryResolvedResponse,
  InventoryResponse,
  PaginatedInventoryResolvedListResponse,
  UpdateInventoryResponse
} from "@/components/modules/inventory/lib/interface.ts";

export function isCreateInventoryResponse(data: unknown): data is CreateInventoryResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export function isInventoryResolved(data: unknown): data is InventoryResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "product_id" in data &&
    typeof data.product_id === "string" &&
    "product" in data &&
    typeof data.product === "string" &&
    "warehouse_id" in data &&
    typeof data.warehouse_id === "string" &&
    "warehouse" in data &&
    typeof data.warehouse === "string" &&
    "quantity" in data &&
    typeof data.quantity === "number" &&
    "price" in data &&
    (data.price === null || typeof data.price === "string") &&
    "cost" in data &&
    (data.cost === null || typeof data.cost === "string") &&
    "currency_code" in data &&
    typeof data.currency_code === "string" &&
    "currency" in data &&
    typeof data.currency === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isInventoryResolvedResponse(data: unknown): data is InventoryResolvedResponse {
  return isCommonResponse(
    data,
    isInventoryResolved,
    isSimpleError
  )
}

export function isInventoryResolvedList(data: unknown): data is InventoryResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isInventoryResolved(item))
  );
}

export function isPaginatedInventoryResolvedListResponse(data: unknown): data is PaginatedInventoryResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isInventoryResolvedList
  );
}

export function isInventory(data: unknown): data is Inventory {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "product_id" in data &&
    typeof data.product_id === "string" &&
    "warehouse_id" in data &&
    typeof data.warehouse_id === "string" &&
    "quantity" in data &&
    typeof data.quantity === "number" &&
    "price" in data &&
    (data.price === null || typeof data.price === "string") &&
    "cost" in data &&
    (data.cost === null || typeof data.cost === "string") &&
    "currency_code" in data &&
    typeof data.currency_code === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isInventoryResponse(data: unknown): data is InventoryResponse {
  return isCommonResponse(
    data,
    isInventory,
    isSimpleError
  )
}

export function isUpdateInventoryResponse(data: unknown): data is UpdateInventoryResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteInventoryResponse(data: unknown): data is DeleteInventoryResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}