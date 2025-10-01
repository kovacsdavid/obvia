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
  isCommonResponse,
  isFormError,
  isPaginatedDataResponse,
  isSimpleError,
  type PaginatedDataResponse,
  type SimpeError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateTenant {
  name: string;
  dbIsSelfHosted: boolean;
  dbHost: string;
  dbPort: number;
  dbName: string;
  dbUser: string;
  dbPassword: string;
}


export interface Tenant {
  id: string;
  name: string;
  db_host: string;
  db_port: number;
  db_name: string;
  db_user: string;
  db_password: string;
  db_max_pool_size: number;
  db_ssl_mode: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type TenantList = Tenant[];

export function isTenant(data: unknown): data is Tenant {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "db_host" in data &&
    typeof data.db_host === "string" &&
    "db_port" in data &&
    typeof data.db_port === "number" &&
    "db_name" in data &&
    typeof data.db_name === "string" &&
    "db_user" in data &&
    typeof data.db_user === "string" &&
    "db_password" in data &&
    typeof data.db_password === "string" &&
    "db_max_pool_size" in data &&
    typeof data.db_max_pool_size === "number" &&
    "db_ssl_mode" in data &&
    typeof data.db_ssl_mode === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "updated_at" in data &&
    typeof data.updated_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isTenantList(data: unknown): data is TenantList {
  return (
    Array.isArray(data) &&
    data.every(item => isTenant(item))
  );
}

export function isTenantResponse(data: unknown): data is CommonResponse<Tenant, FormError> {
  return isCommonResponse(
    data,
    isTenant,
    isFormError,
  )
}

export function isPaginatedTenantListResponse(data: unknown): data is PaginatedDataResponse<TenantList> {
  return isPaginatedDataResponse(
    data,
    isTenantList
  );
}

export type NewTokenResponse = string;

export function isNewTokenResponse(data: unknown): data is NewTokenResponse {
  return typeof data === "string"
}

export function isActiveTenantResponse(data: unknown): data is CommonResponse<SimpleMessageData, SimpeError> {
  return isCommonResponse(
    data,
    isNewTokenResponse,
    isSimpleError
  )
}
