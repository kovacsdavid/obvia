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
  isCommonResponse,
  isPaginatedDataResponse,
  isSimpleError,
  type PaginatedDataResponse,
  type SimpeError
} from "@/lib/interfaces/common.ts";

export interface CreateWarehouse {
  name: string
  contactName: string
  contactPhone: string
  status: string
}

export interface Warehouse {
  id: string,
  name: string,
  contact_name: string,
  contact_phone: string,
  status: string,
  created_by_id: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type WarehouseList = Warehouse[];

export function isWarehouse(data: unknown): data is Warehouse {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "contact_name" in data &&
    typeof data.contact_name === "string" &&
    "contact_phone" in data &&
    typeof data.contact_phone === "string" &&
    "status" in data &&
    typeof data.status === "string" &&
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

export function isWarehouseList(data: unknown): data is WarehouseList {
  return (
    Array.isArray(data) &&
    data.every(item => isWarehouse(item))
  );
}

export function isWarehouseListResponse(data: unknown): data is CommonResponse<WarehouseList, SimpeError> {
  return isCommonResponse(
    data,
    isWarehouseList,
    isSimpleError,
  )
}

export function isPaginatedWarehouseListResponse(data: unknown): data is PaginatedDataResponse<WarehouseList> {
  return isPaginatedDataResponse(
    data,
    isWarehouseList,
  )
}

export interface WarehouseResolved {
  id: string,
  name: string,
  contact_name: string | null,
  contact_phone: string | null,
  status: string,
  created_by_id: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type WarehouseResolvedList = WarehouseResolved[];

export function isWarehouseResolved(data: unknown): data is WarehouseResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "contact_name" in data &&
    (data.contact_name === null || typeof data.contact_name === "string") &&
    "contact_phone" in data &&
    (data.contact_phone === null || typeof data.contact_phone === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
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

export function isWarehouseResolvedList(data: unknown): data is WarehouseResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isWarehouseResolved(item))
  );
}


export function isPaginatedWarehouseResolvedListResponse(data: unknown): data is PaginatedDataResponse<WarehouseResolvedList> {
  return isPaginatedDataResponse(
    data,
    isWarehouseResolvedList,
  )
}
