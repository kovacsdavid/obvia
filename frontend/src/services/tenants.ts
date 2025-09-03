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

import {globalRequestTimeout} from "@/services/utils/consts.ts";

export interface CreateTenant {
  name: string;
  dbIsSelfHosted: boolean;
  dbHost: string;
  dbPort: number;
  dbName: string;
  dbUser: string;
  dbPassword: string;
}

export interface CreateTenantResponse {
  success: boolean,
  data?: {
    message: string,
  },
  error?: {
    reference: string | null,
    global: string | null,
    fields: Record<string, string | null>,
  }
}

export function isTenantsResponse(data: unknown): data is CreateTenantResponse {
  return (
    typeof data === "object" &&
    data !== null &&
    "success" in data &&
    typeof data.success === "boolean" &&
    (
      !("data" in data) ||
      (typeof data.data === "object" &&
        data.data !== null &&
        "message" in data.data &&
        typeof data.data.message === "string")
    ) &&
    (
      !("error" in data) ||
      (typeof data.error === "object" &&
        data.error !== null &&
        "reference" in data.error &&
        (data.error.reference === null || typeof data.error.reference === "string") &&
        "global" in data.error &&
        (data.error.global === null || typeof data.error.global === "string") &&
        "fields" in data.error &&
        typeof data.error.fields === "object")
    )
  );
}

export async function create({
                               name,
                               dbIsSelfHosted,
                               dbHost,
                               dbPort,
                               dbName,
                               dbUser,
                               dbPassword

                             }: CreateTenant, token: string | null): Promise<Response> {
  return await fetch(`/api/tenants/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})

    },
    body: JSON.stringify({
      name,
      is_self_hosted: dbIsSelfHosted,
      db_host: dbHost,
      db_port: dbPort,
      db_name: dbName,
      db_user: dbUser,
      db_password: dbPassword
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),  
  });
}

export interface TenantData {
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

// Interface for the response
export interface TenantsListResponse {
  success: boolean,
  data: {
    page: number,
    limit: number,
    total: number,
    data: TenantData[]
  };
}


export function isTenantsList(data: unknown): data is TenantsListResponse {
  if (typeof data !== 'object' || data === null) return false;

  const response = data as Record<string, any>;

  // Validate top-level properties
  if (!('success' in response) || typeof response.success !== 'boolean') return false;
  if (!('data' in response) || typeof response.data !== 'object' || response.data === null) return false;

  // Validate the pagination info inside data
  const dataObj = response.data;

  if (
    !('page' in dataObj) || typeof dataObj.page !== 'number' ||
    !('limit' in dataObj) || typeof dataObj.limit !== 'number' ||
    !('total' in dataObj) || typeof dataObj.total !== 'number' ||
    !('data' in dataObj) || !Array.isArray(dataObj.data)
  ) {
    return false;
  }

  // Validate each TenantData object inside the data array
  return dataObj.data.every((item: any) => {
    if (typeof item !== 'object' || item === null) return false;
    return (
      typeof item.id === 'string' &&
      typeof item.name === 'string' &&
      typeof item.db_host === 'string' &&
      typeof item.db_port === 'number' &&
      typeof item.db_name === 'string' &&
      typeof item.db_user === 'string' &&
      typeof item.db_password === 'string' &&
      typeof item.db_max_pool_size === 'number' &&
      typeof item.db_ssl_mode === 'string' &&
      typeof item.created_at === 'string' &&
      typeof item.updated_at === 'string' &&
      ('deleted_at' in item ? (item.deleted_at === null || typeof item.deleted_at === 'string') : true)
    );
  });
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/tenants/list` : `/api/tenants/list?q=${query}`
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}

export interface ActivateResponse {
  success: boolean,
  data: string | null
}

export function isActivateResponse(data: unknown): data is ActivateResponse {
  return (
    typeof data === "object" &&
    data !== null &&
    "success" in data &&
    typeof (data as any).success === "boolean" &&
    "data" in data &&
    (typeof (data as any).data === "string" || (data as any).data === null)
  );
}

export async function activate(new_tenant_id: string | null, token: string | null) {
  const response = await fetch(`/api/tenants/activate`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    body: JSON.stringify({
      new_tenant_id
    }),
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
  let responseJson;
  try {
    responseJson = await response.json();
  } catch {
    throw new Error("Server responded with invalid JSON format");
  }

  if (!isActivateResponse(responseJson)) {
    throw new Error("Server responded with invalid data");
  }
  return responseJson;
}