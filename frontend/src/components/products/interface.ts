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
  isCommonResponse, isPaginatedDataResponse,
  isSimpleError,
  type PaginatedDataResponse,
  type SimpeError
} from "@/lib/interfaces/common.ts";

export interface CreateProduct {
  name: string
  description: string
  unitOfMeasureId: string
  newUnitOfMeasure: string,
  status: string
}

export interface Product {
  id: string,
  name: string,
  description: string,
  unit_of_measure_id: string,
  status: string,
  created_by_id: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null
}

export type ProductList = Product[];

export function isProduct(data: unknown): data is Product {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "description" in data &&
    typeof data.description === "string" &&
    "unit_of_measure_id" in data &&
    typeof data.unit_of_measure_id === "string" &&
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

export function isProductList(data: unknown): data is ProductList {
  return (
    Array.isArray(data) &&
    data.every(item => isProduct(item))
  );
}

export function isProductListResponse(data: unknown): data is CommonResponse<ProductList, SimpeError> {
  return isCommonResponse(
    data,
    isProductList,
    isSimpleError,
  )
}

export function isPaginatedProductListResponse(data: unknown): data is PaginatedDataResponse<ProductList> {
  return isPaginatedDataResponse(
    data,
    isProductList
  )
}

export interface UnitOfMeasure {
  id: string,
  unit_of_measure: string,
  created_at: string,
  deleted_at: string | null
}

export type UnitsOfMeasureList = UnitOfMeasure[];

export function isUnitOfMeasure(data: unknown): data is UnitOfMeasure {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "unit_of_measure" in data &&
    typeof data.unit_of_measure === "string" &&
    "created_at" in data &&
    typeof data.created_at === "string" &&
    "deleted_at" in data &&
    (data.deleted_at === null || typeof data.deleted_at === "string")
  );
}

export function isUnitsOfMeasureList(data: unknown): data is UnitsOfMeasureList {
  return (
    Array.isArray(data) &&
    data.every(item => isUnitOfMeasure(item))
  );
}

export function isUnitsOfMeasureListResponse(data: unknown): data is CommonResponse<UnitsOfMeasureList, SimpeError> {
  return isCommonResponse(
    data,
    isUnitsOfMeasureList,
    isSimpleError,
  );
}

export interface ProductResolved {
  id: string,
  name: string,
  description: string | null,
  unit_of_measure_id: string,
  unit_of_measure: string,
  status: string,
  created_by_id: string,
  created_by: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
}

export type ProductResolvedList = ProductResolved[];

export function isProductResolved(data: unknown): data is ProductResolved {
  return (
    typeof data === "object" &&
    data !== null &&
    "id" in data &&
    typeof data.id === "string" &&
    "name" in data &&
    typeof data.name === "string" &&
    "description" in data &&
    (data.description === null || typeof data.description === "string") &&
    "unit_of_measure_id" in data &&
    typeof data.unit_of_measure_id === "string" &&
    "unit_of_measure" in data &&
    typeof data.unit_of_measure === "string" &&
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

export function isProductResolvedList(data: unknown): data is ProductResolvedList {
  return (
    Array.isArray(data) &&
    data.every(item => isProductResolved(item))
  );
}

export function isPaginatedProductResolvedListResponse(data: unknown): data is PaginatedDataResponse<ProductResolvedList> {
  return isPaginatedDataResponse(
    data,
    isProductResolvedList
  )
}