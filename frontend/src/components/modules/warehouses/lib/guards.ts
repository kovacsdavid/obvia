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
  CreateWarehouseResponse,
  DeleteWarehouseResponse,
  PaginatedWarehouseResolvedListResponse,
  UpdateWarehouseResponse,
  Warehouse,
  WarehouseResolved,
  WarehouseResolvedList,
  WarehouseResolvedResponse,
  WarehouseResponse
} from "@/components/modules/warehouses/lib/interface.ts";

export function isCreateWarehouseResponse(data: unknown): data is CreateWarehouseResponse {
  return isCommonResponse(
    data,
    isWarehouse,
    isFormError,
  )
}

export function isWarehouseResolvedResponse(data: unknown): data is WarehouseResolvedResponse {
  return isCommonResponse(
    data,
    isWarehouseResolved,
    isSimpleError,
  )
}

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

export function isPaginatedWarehouseResolvedListResponse(data: unknown): data is PaginatedWarehouseResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isWarehouseResolvedList,
  )
}

export function isWarehouse(data: unknown): data is Warehouse {
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
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isWarehouseResponse(data: unknown): data is WarehouseResponse {
  return isCommonResponse(
    data,
    isWarehouse,
    isSimpleError
  )
}

export function isUpdateWarehouseResponse(data: unknown): data is UpdateWarehouseResponse {
  return isCommonResponse(
    data,
    isWarehouse,
    isFormError
  )
}

export function isDeleteWarehouseResponse(data: unknown): data is DeleteWarehouseResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}
