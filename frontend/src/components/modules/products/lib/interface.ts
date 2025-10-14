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
  type PaginatedDataResponse,
  type SimpleError,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface ProductUserInput {
  id: string | null
  name: string
  description: string
  unitOfMeasureId: string
  newUnitOfMeasure: string
  status: string
}

export interface Product {
  id: string,
  name: string,
  description: string | null,
  unit_of_measure_id: string,
  status: string,
  created_by_id: string,
  created_at: string,
  updated_at: string,
  deleted_at: string | null,
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

export type CreateProductResponse = CommonResponse<SimpleMessageData, FormError>;
export type UpdateProductResponse = CommonResponse<SimpleMessageData, FormError>;
export type DeleteProductResponse = CommonResponse<SimpleMessageData, SimpleError>;
export type ProductResponse = CommonResponse<Product, SimpleError>;
export type ProductResolvedResponse = CommonResponse<ProductResolved, SimpleError>;
export type ProductResolvedList = ProductResolved[];
export type PaginatedProductResolvedListResponse = PaginatedDataResponse<ProductResolvedList, SimpleError>;

