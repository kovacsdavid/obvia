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
  type CreateInventoryMovementResponse,
  type DeleteInventoryMovementResponse,
  type InventoryMovementResolvedResponse,
  type InventoryMovementResponse,
  type InventoryMovementUserInput,
  type PaginatedInventoryMovementResolvedListResponse
} from "@/components/modules/inventory_movements/lib/interface.ts";
import {
  isSelectOptionListResponse,
  type ProcessedResponse,
  ProcessResponse,
  type SelectOptionListResponse
} from "@/lib/interfaces/common.ts";
import {
  isCreateInventoryMovementResponse,
  isDeleteInventoryMovementResponse,
  isInventoryMovementResolvedResponse,
  isInventoryMovementResponse,
  isPaginatedInventoryMovementResolvedListResponse
} from "@/components/modules/inventory_movements/lib/guards.ts";

export async function create(input: InventoryMovementUserInput, token: string | null): Promise<ProcessedResponse<CreateInventoryMovementResponse>> {
  return await fetch(`/api/inventory_movements/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id: input.id,
      inventory_id: input.inventoryId,
      movement_type: input.movementType,
      quantity: input.quantity,
      reference_type: input.referenceType,
      reference_id: input.referenceId,
      unit_price: input.unitPrice,
      total_price: input.totalPrice,
      tax_id: input.taxId,
    })
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isCreateInventoryMovementResponse
    ) ?? unexpectedFormError;
  });
}

export async function list(query: string | null, token: string | null): Promise<ProcessedResponse<PaginatedInventoryMovementResolvedListResponse>> {
  const uri = query === null ? `/api/inventory_movements/list` : `/api/inventory_movements/list?q=${query}`;
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
      isPaginatedInventoryMovementResolvedListResponse
    ) ?? unexpectedError;
  });
}

export async function get(uuid: string, token: string | null): Promise<ProcessedResponse<InventoryMovementResponse>> {
  return await fetch(`/api/inventory_movements/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isInventoryMovementResponse
    ) ?? unexpectedError;
  });
}

export async function get_resolved(uuid: string, token: string | null): Promise<ProcessedResponse<InventoryMovementResolvedResponse>> {
  return await fetch(`/api/inventory_movements/get_resolved?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isInventoryMovementResolvedResponse
    ) ?? unexpectedError;
  });
}

export async function deleteItem(uuid: string, token: string | null): Promise<ProcessedResponse<DeleteInventoryMovementResponse>> {
  return await fetch(`/api/inventory_movements/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return await ProcessResponse(
      response,
      isDeleteInventoryMovementResponse
    ) ?? unexpectedError;
  });
}

export async function select_list(
  list: string,
  token: string | null): Promise<ProcessedResponse<SelectOptionListResponse>> {
  return await fetch(`/api/inventory_movements/select_list?list=${list}`, {
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
