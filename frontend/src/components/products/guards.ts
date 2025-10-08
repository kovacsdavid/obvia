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
import type {CreateProjectResponse} from "@/components/projects/interface.ts";
import type {
  DeleteProductResponse,
  PaginatedProductResolvedListResponse,
  Product,
  ProductResolved,
  ProductResolvedList,
  ProductResolvedResponse,
  ProductResponse,
  UpdateProductResponse
} from "@/components/products/interface.ts";

export function isCreateProductResponse(data: unknown): data is CreateProjectResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError,
  )
}

export function isProductResolvedResponse(data: unknown): data is ProductResolvedResponse {
  return isCommonResponse(
    data,
    isProductResolved,
    isSimpleError,
  )
}

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

export function isPaginatedProductResolvedListResponse(data: unknown): data is PaginatedProductResolvedListResponse {
  return isPaginatedDataResponse(
    data,
    isProductResolvedList
  )
}

export function isProduct(data: unknown): data is Product {
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

export function isProductResponse(data: unknown): data is ProductResponse {
  return isCommonResponse(
    data,
    isProduct,
    isSimpleError
  )
}

export function isUpdateProductResponse(data: unknown): data is UpdateProductResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isFormError
  )
}

export function isDeleteProductResponse(data: unknown): data is DeleteProductResponse {
  return isCommonResponse(
    data,
    isSimpleMessageData,
    isSimpleError
  )
}