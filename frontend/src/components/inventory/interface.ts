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
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface InventoryUserInput {
  id: string | null
  productId: string
  warehouseId: string
  quantity: string
  price: string
  cost: string
  currencyId: string
  newCurrency: string
}

export interface Inventory {
  id: string,
  product_id: string,
  warehouse_id: string,
  quantity: number,
  price: string | null,
  cost: string | null,
  currency_id: string,
  created_by_id: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
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

export type CreateInventoryResponse = CommonResponse<SimpleMessageData, FormError>;
export type UpdateInventoryResponse = CommonResponse<SimpleMessageData, FormError>;
export type DeleteInventoryResponse = CommonResponse<SimpleMessageData, SimpleError>;
export type InventoryResolvedList = InventoryResolved[];
export type InventoryResponse = CommonResponse<Inventory, SimpleError>;
export type InventoryResolvedResponse = CommonResponse<InventoryResolved, SimpleError>;
export type PaginatedInventoryResolvedListResponse = PaginatedDataResponse<InventoryResolvedList, SimpleError>;
