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

export interface CreateProduct {
  name: string
  description: string
  unitOfMeasureId: string
  newUnitOfMeasure: string,
  status: string
}

export async function create({
                               name,
                               description,
                               unitOfMeasureId,
                               newUnitOfMeasure,

                               status
                             }: CreateProduct, token: string | null): Promise<Response> {
  return await fetch(`/api/products/create`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
    body: JSON.stringify({
      name,
      description,
      unit_of_measure_id: unitOfMeasureId,
      new_unit_of_measure: newUnitOfMeasure,
      status
    })
  })
}

export async function list(query: string | null, token: string | null): Promise<Response> {
  const uri = query === null ? `/api/products/list` : `/api/products/list?q=${query}`;
  return await fetch(uri, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}



export interface UnitsOfMeasureSelectListItem {
  id: string,
  unit_of_measure: string,
  created_at: string,
  deleted_at: string | null
}

export interface UnitsOfMeasureSelectListResponse {
  success: boolean,
  data: UnitsOfMeasureSelectListItem[]
}


export function isUnitsOfMeasureSelectListItem(obj: any): obj is UnitsOfMeasureSelectListItem {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.id === 'string' &&
    typeof obj.unit_of_measure === 'string' &&
    typeof obj.created_at === 'string' &&
    (obj.deleted_at === null || typeof obj.deleted_at === 'string')
  );
}

export function isUnitsOfMeasureSelectListResponse(obj: any): obj is UnitsOfMeasureSelectListResponse {
  return (
    typeof obj === 'object' &&
    obj !== null &&
    typeof obj.success === 'boolean' &&
    Array.isArray(obj.data) &&
    obj.data.every((item: any) => isUnitsOfMeasureSelectListItem(item))
  );
}


export async function select_list(list: string, token: string | null): Promise<Response> {
  return await fetch(`/api/products/select_list?list=${list}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...(token ? {"Authorization": `Bearer ${token}`} : {})
    },
    signal: AbortSignal.timeout(globalRequestTimeout),
  });
}