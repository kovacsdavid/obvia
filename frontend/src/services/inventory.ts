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

import {globalRequestTimeout} from "@/services/utils/consts.ts";

export interface CreateInventory {
  productId: string
  warehouseId: string
  quantity: string
  price: string
  cost: string
  currencyId: string
  newCurrency: string
}

export async function create({
                               productId,
                               warehouseId,
                               quantity,
                               price,
                               cost,
                               currencyId,
                               newCurrency,
                             }: CreateInventory, token: string | null) {
  return await fetch(`/api/inventory/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      product_id: productId,
      warehouse_id: warehouseId,
      quantity,
      price,
      cost,
      currency_id: currencyId,
      new_currency: newCurrency,
    })
  })
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/inventory/list` : `/api/inventory/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}


export interface CurrencySelectListItem {
  id: string,
  currency: string,
  created_at: string,
  deleted_at: string | null
}

export interface CurrencySelectListResponse {
  success: boolean,
  data: CurrencySelectListItem[]
}

export function isCurrencySelectListItem(obj: any): obj is CurrencySelectListItem {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.id === 'string' &&
    typeof obj.currency === 'string' &&
    typeof obj.created_at === 'string' &&
    (obj.deleted_at === null || typeof obj.deleted_at === 'string')
  );
}

export function isCurrencySelectListResponse(obj: any): obj is CurrencySelectListResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.success === 'boolean' &&
    Array.isArray(obj.data) &&
    obj.data.every((item: any) => isCurrencySelectListItem(item))
  );
}

export interface WarehouseSelectListItem {
  id: string,
  name: string,
  contact_name: string,
  contact_phone: string,
  status: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export interface WarehouseSelectListResponse {
  success: boolean,
  data: WarehouseSelectListItem[]
}

export function isWarehouseSelectListItem(obj: any): obj is WarehouseSelectListItem {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.id === 'string' &&
    typeof obj.name === 'string' &&
    typeof obj.contact_name === 'string' &&
    typeof obj.contact_phone === 'string' &&
    typeof obj.status === 'string' &&
    typeof obj.created_by === 'string' &&
    typeof obj.created_at === 'string' &&
    typeof obj.updated_at === 'string' &&
    (obj.deleted_at === null || typeof obj.deleted_at === 'string')
  );
}

export function isWarehouseSelectListResponse(obj: any): obj is WarehouseSelectListResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.success === 'boolean' &&
    Array.isArray(obj.data) &&
    obj.data.every((item: any) => isWarehouseSelectListItem(item))
  );
}

export interface ProductSelectListItem {
  id: string,
  name: string,
  description: string,
  unit_of_measure_id: string,
  status: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null
}

export interface ProductSelectListResponse {
  success: boolean,
  data: ProductSelectListItem[]
}

export function isProductSelectListItem(obj: any): obj is ProductSelectListItem {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.id === 'string' &&
    typeof obj.name === 'string' &&
    typeof obj.description === 'string' &&
    typeof obj.unit_of_measure_id === 'string' &&
    typeof obj.status === 'string' &&
    typeof obj.created_by === 'string' &&
    typeof obj.created_at === 'string' &&
    typeof obj.updated_at === 'string' &&
    (obj.deleted_at === null || typeof obj.deleted_at === 'string')
  );
}

export function isProductSelectListResponse(obj: any): obj is ProductSelectListResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.success === 'boolean' &&
    Array.isArray(obj.data) &&
    obj.data.every((item: any) => isProductSelectListItem(item))
  );
}

export async function select_list(list: string, token: string | null): Promise<Response> {
  return await fetch(`/api/inventory/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}
