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

import type {
  CommonResponse,
  FormError,
  PaginatedDataResponse,
  SimpleError,
  SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface InventoryReservationUserInput {
  id: string | null;
  inventoryId: string;
  quantity: string;
  referenceType: string | null;
  referenceId: string | null;
  reservedUntil: string | null;
  status: string;
}

export interface InventoryReservation {
  id: string;
  inventory_id: string;
  quantity: number;
  reference_type: string | null;
  reference_id: string | null;
  reserved_until: string | null;
  status: string;
  created_by_id: string;
  created_at: string;
  updated_at: string;
}

export interface InventoryReservationResolved {
  id: string;
  inventory_id: string;
  quantity: number;
  reference_type: string | null;
  reference_id: string | null;
  reserved_until: string | null;
  status: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export type CreateInventoryReservationResponse = CommonResponse<SimpleMessageData, FormError>;
export type DeleteInventoryReservationResponse = CommonResponse<SimpleMessageData, SimpleError>;
export type InventoryReservationResponse = CommonResponse<InventoryReservation, SimpleError>;
export type InventoryReservationResolvedResponse = CommonResponse<InventoryReservationResolved, SimpleError>;
export type InventoryReservationResolvedList = InventoryReservationResolved[];
export type PaginatedInventoryReservationResolvedListResponse = PaginatedDataResponse<InventoryReservationResolvedList, SimpleError>;
