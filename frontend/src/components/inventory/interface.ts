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

import {type CommonResponse, isCommonResponse, isSimpleError, type SimpeError} from "@/lib/interfaces/common.ts";

export interface CreateInventory {
  productId: string
  warehouseId: string
  quantity: string
  price: string
  cost: string
  currencyId: string
  newCurrency: string
}

export interface Currency {
  id: string,
  currency: string,
  created_at: string,
  deleted_at: string | null
}

export type CurrencyList = Currency[];

export function isCurrency(data: unknown): data is Currency {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "currency" in data &&
    typeof data.currency === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isCurrencyList(data: unknown): data is CurrencyList {
  return (
    Array.isArray(data) &&
    data.every(item => isCurrency(item))
  );
}


export function isCurrencyListResponse(data: unknown): data is CommonResponse<CurrencyList, SimpeError> {
  return isCommonResponse(
    data,
    isCurrencyList,
    isSimpleError
  )
}
