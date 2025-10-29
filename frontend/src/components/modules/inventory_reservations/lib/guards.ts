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
  CreateInventoryReservationResponse,
  DeleteInventoryReservationResponse,
  InventoryReservation,
  InventoryReservationResolved,
  InventoryReservationResolvedList,
  InventoryReservationResolvedResponse,
  InventoryReservationResponse,
  PaginatedInventoryReservationResolvedListResponse
} from "@/components/modules/inventory_reservations/lib/interface.ts";

export function isInventoryReservationResolved(data: unknown): data is InventoryReservationResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "inventory_id" in data &&
    typeof data.inventory_id === "string" &&
    "quantity" in data &&
    typeof data.quantity === "string" &&
    "reference_type" in data &&
    (data.reference_type === null || typeof data.reference_type === "string") &&
    "reference_id" in data &&
    (data.reference_id === null || typeof data.reference_id === "string") &&
    "reserved_until" in data &&
    (data.reserved_until === null || typeof data.reserved_until === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_by" in data &&
    typeof data.created_by === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string"
  );
}

export function isInventoryReservationResolvedResponse(data: unknown): data is InventoryReservationResolvedResponse {
  return isCommonResponse(
    data,
    isInventoryReservationResolved,
    isSimpleError,
  );
}

export function isInventoryReservation(data: unknown): data is InventoryReservation {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "inventory_id" in data &&
    typeof data.inventory_id === "string" &&
    "quantity" in data &&
    typeof data.quantity === "string" &&
    "reference_type" in data &&
    (data.reference_type === null || typeof data.reference_type === "string") &&
    "reference_id" in data &&
    (data.reference_id === null || typeof data.reference_id === "string") &&
    "reserved_until" in data &&
    (data.reserved_until === null || typeof data.reserved_until === "string") &&
    "status" in data &&
    typeof data.status === "string" &&
    "created_by_id" in data &&
    typeof data.created_by_id === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string"
  );
}

export function isInventoryReservationResponse(data: unknown): data is InventoryReservationResponse {
  return isCommonResponse(
    data,
    isInventoryReservation,
    isSimpleError,
  );
}

export function isInventoryReservationResolvedList(data: unknown): data is InventoryReservationResolvedList {
  return Array.isArray(data) && data.every(item => isInventoryReservationResolved(item));
}

export function isPaginatedInventoryReservationResolvedListResponse(data: unknown): data is PaginatedInventoryReservationResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isInventoryReservationResolvedList,
  );
}

export function isCreateInventoryReservationResponse(data: unknown): data is CreateInventoryReservationResponse {
  return isCommonResponse(
    data,
    isInventoryReservation,
    isFormError,
  );
}

export function isDeleteInventoryReservationResponse(data: unknown): data is DeleteInventoryReservationResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError,
  );
}
