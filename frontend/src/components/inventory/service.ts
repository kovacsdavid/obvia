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

import {globalRequestTimeout, unexpectedError, unexpectedFormError} from "@/services/utils/consts.ts";
import {
  type CreateInventory,
  type CreateInventoryResponse,
  type InventoryResolvedResponse,
  isCreateInventoryResponse,
  isInventoryResolvedResponse,
  isPaginatedInventoryResolvedListResponse,
  type PaginatedInventoryResolvedListResponse
} from "@/components/inventory/interface.ts";
import {
  isSelectOptionListResponse,
  type ProcessedResponse,
  ProcessResponse,
  type SelectOptionListResponse
} from "@/lib/interfaces/common.ts";

export async function create({
                               productId,
                               warehouseId,
                               quantity,
                               price,
                               cost,
                               currencyId,
                               newCurrency,
                             }: CreateInventory, token: string | null): Promise<ProcessedResponse<CreateInventoryResponse>> {
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
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateInventoryResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedInventoryResolvedListResponse>> {
  const uri = query === null ? `/api/inventory/list` : `/api/inventory/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isPaginatedInventoryResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function select_list(
  list: string,
  token: string | null): Promise<ProcessedResponse<SelectOptionListResponse>> {
  return await fetch(`/api/inventory/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isSelectOptionListResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<InventoryResolvedResponse>> {
  return await fetch(`/api/inventory/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isInventoryResolvedResponse
    ) ?? unexpectedError;
  });
}
