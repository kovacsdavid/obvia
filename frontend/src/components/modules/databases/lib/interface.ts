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
  type SimpleError,
} from "@/lib/interfaces/common.ts";
import type {Claims} from "@/components/modules/auth/lib/interface.ts";
import {isClaims} from "@/components/modules/auth/lib/guards.ts";

export interface CreateDatabase {
  name: string;
  dbIsSelfHosted: boolean;
  dbHost: string;
  dbPort: number;
  dbName: string;
  dbUser: string;
  dbPassword: string;
}

export type CreateDatabaseResponse = CommonResponse<Database, FormError>;

export function isCreateDatabaseResponse(data: unknown): data is CreateDatabaseResponse {
  return isCommonResponse(
    data,
    isDatabase,
    isFormError,
  )
}

export interface Database {
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

export type DatabaseResponse = CommonResponse<Database, SimpleError>;

export function isDatabaseResponse(data: unknown): data is DatabaseResponse {
  return isCommonResponse(
    data,
    isDatabase,
    isSimpleError
  )
}

export type DatabaseList = Database[];

export function isDatabase(data: unknown): data is Database {
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

export function isDatabaseList(data: unknown): data is DatabaseList {
  return (
    Array.isArray(data) &&
    data.every(item => isDatabase(item))
  );
}

export type PaginatedDatabaseListResponse = PaginatedDataResponse<DatabaseList, SimpleError>;

export function isPaginatedDatabaseListResponse(data: unknown): data is PaginatedDatabaseListResponse {
  return isPaginatedDataResponse(
    data,
    isDatabaseList
  );
}

export interface NewTokenResponse {
  token: string,
  claims: Claims,
}

export function isNewTokenResponse(data: unknown): data is NewTokenResponse {
  return (
    typeof data === "object" &&
    data !== null &&
    "token" in data &&
    typeof data.token === "string" &&
    "claims" in data &&
    isClaims(data.claims)
  )
}

export type ActiveDatabaseResponse = CommonResponse<NewTokenResponse, SimpleError>;

export function isActiveDatabaseResponse(data: unknown): data is ActiveDatabaseResponse {
  return isCommonResponse(
    data,
    isNewTokenResponse,
    isSimpleError
  )
}
