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
  type CommonResponse,
  type FormError,
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  isSimpleMessageData,
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateInventory {
  productId: string
  warehouseId: string
  quantity: string
  price: string
  cost: string
  currencyId: string
  newCurrency: string
}

export type CreateInventoryResponse = CommonResponse<SimpleMessageData, FormError>;

export function isCreateInventoryResponse(data: unknown): data is CreateInventoryResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export interface InventoryResolved {
  id: string,
  product_id: string,
  product: string,
  warehouse_id: string,
  warehouse: string,
  quantity: number,
  price: string | null,
  cost: string | null,
  currency_id: string,
  currency: string,
  created_by_id: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type InventoryResolvedList = InventoryResolved[];

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
    "currency_id" in data &&
    typeof data.currency_id === "string" &&
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

export type InventoryResolvedResponse = CommonResponse<InventoryResolved, SimpleError>;

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

export type PaginatedInventoryResolvedListResponse = PaginatedDataResponse<InventoryResolvedList, SimpleError>;

export function isPaginatedInventoryResolvedListResponse(data: unknown): data is PaginatedInventoryResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isInventoryResolvedList
  );
}