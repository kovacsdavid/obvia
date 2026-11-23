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

import { schemaGuard } from "@/lib/api/schemas";
import {
    productErrorSchema,
    productSchema,
    productResolvedSchema,
    productResolvedListSchema,
    productResponseSchema,
    productResolvedResponseSchema,
    createProductResponseSchema,
    updateProductResponseSchema,
    deleteProductResponseSchema,
    paginatedProductResolvedListResponseSchema,
} from "./schemas";

export const isProductError = schemaGuard(productErrorSchema);
export const isProduct = schemaGuard(productSchema);
export const isProductResolved = schemaGuard(productResolvedSchema);
export const isProductResolvedList = schemaGuard(productResolvedListSchema);
export const isProductResponse = schemaGuard(productResponseSchema);
export const isProductResolvedResponse = schemaGuard(
    productResolvedResponseSchema,
);
export const isCreateProductResponse = schemaGuard(createProductResponseSchema);
export const isUpdateProductResponse = schemaGuard(updateProductResponseSchema);
export const isDeleteProductResponse = schemaGuard(deleteProductResponseSchema);
export const isPaginatedProductResolvedListResponse = schemaGuard(
    paginatedProductResolvedListResponseSchema,
);
