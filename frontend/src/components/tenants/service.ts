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
import type {CreateTenant} from "@/components/tenants/interface.ts";

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

export async function activate(new_tenant_id: string | null, token: string | null) {
  return await fetch(`/api/tenants/activate`, {
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
}