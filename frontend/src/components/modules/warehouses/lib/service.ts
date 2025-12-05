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
  globalRequestTimeout,
  unexpectedError,
  unexpectedFormError,
} from "@/services/utils/consts.ts";
import {
  type CreateWarehouseResponse,
  type DeleteWarehouseResponse,
  type PaginatedWarehouseResolvedListResponse,
  type UpdateWarehouseResponse,
  type WarehouseResolvedResponse,
  type WarehouseResponse,
  type WarehouseUserInput,
} from "@/components/modules/warehouses/lib/interface.ts";
import {
  type ProcessedResponse,
  ProcessResponse,
} from "@/lib/interfaces/common.ts";
import {
  isCreateWarehouseResponse,
  isDeleteWarehouseResponse,
  isPaginatedWarehouseResolvedListResponse,
  isUpdateWarehouseResponse,
  isWarehouseResolvedResponse,
  isWarehouseResponse,
} from "@/components/modules/warehouses/lib/guards.ts";

export async function create(
  { id, name, contactName, contactPhone, status }: WarehouseUserInput,
  token: string | null,
): Promise<ProcessedResponse<CreateWarehouseResponse>> {
  return await fetch(`/api/warehouses/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      contact_name: contactName,
      contact_phone: contactPhone,
      status,
    }),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isCreateWarehouseResponse)) ??
      unexpectedFormError
    );
  });
}

export async function list(
  query: string | null,
  token: string | null,
): Promise<ProcessedResponse<PaginatedWarehouseResolvedListResponse>> {
  const uri =
    query === null ? `/api/warehouses/list` : `/api/warehouses/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(
        response,
        isPaginatedWarehouseResolvedListResponse,
      )) ?? unexpectedError
    );
  });
}

export async function get_resolved(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<WarehouseResolvedResponse>> {
  return await fetch(`/api/warehouses/get_resolved?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isWarehouseResolvedResponse)) ??
      unexpectedError
    );
  });
}

export async function update(
  { id, name, contactName, contactPhone, status }: WarehouseUserInput,
  token: string | null,
): Promise<ProcessedResponse<UpdateWarehouseResponse>> {
  return await fetch(`/api/warehouses/update`, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      id,
      name,
      contact_name: contactName,
      contact_phone: contactPhone,
      status,
    }),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isUpdateWarehouseResponse)) ??
      unexpectedFormError
    );
  });
}

export async function get(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<WarehouseResponse>> {
  return await fetch(`/api/warehouses/get?uuid=${uuid}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isWarehouseResponse)) ?? unexpectedError
    );
  });
}

export async function deleteItem(
  uuid: string,
  token: string | null,
): Promise<ProcessedResponse<DeleteWarehouseResponse>> {
  return await fetch(`/api/warehouses/delete?uuid=${uuid}`, {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  }).then(async (response: Response) => {
    return (
      (await ProcessResponse(response, isDeleteWarehouseResponse)) ??
      unexpectedError
    );
  });
}
