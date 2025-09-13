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

export interface CreateCustomer {
  name: string
  contactName: string
  email: string
  phoneNumber: string
  status: string
  customerType: string | undefined,
}

export async function create({
                               name,
                               contactName,
                               email,
                               phoneNumber,
                               status,
                               customerType,
                             }: CreateCustomer, token: string | null): Promise<Response> {
  return await fetch(`/api/customers/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      name,
      contact_name: contactName,
      email,
      phone_number: phoneNumber,
      status,
      customer_type: typeof customerType === "undefined" ? null : customerType,
    }),
  });
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/customers/list` : `/api/customers/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}