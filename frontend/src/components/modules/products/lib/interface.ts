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

import type { z } from "zod";
import type {
    productUserInputSchema,
    productErrorSchema,
    productSchema,
    productResolvedSchema,
    productResolvedListSchema,
    productResponseSchema,
    createProductResponseSchema,
    updateProductResponseSchema,
    deleteProductResponseSchema,
    productResolvedResponseSchema,
    paginatedProductResolvedListResponseSchema,
} from "./schemas";

export type ProductUserInput = z.infer<typeof productUserInputSchema>;
export type ProductError = z.infer<typeof productErrorSchema>;
export type Product = z.infer<typeof productSchema>;
export type ProductResolved = z.infer<typeof productResolvedSchema>;
export type ProductResolvedList = z.infer<typeof productResolvedListSchema>;
export type CreateProductResponse = z.infer<typeof createProductResponseSchema>;
export type UpdateProductResponse = z.infer<typeof updateProductResponseSchema>;
export type DeleteProductResponse = z.infer<typeof deleteProductResponseSchema>;
export type ProductResponse = z.infer<typeof productResponseSchema>;
export type ProductResolvedResponse = z.infer<
    typeof productResolvedResponseSchema
>;
export type PaginatedProductResolvedListResponse = z.infer<
    typeof paginatedProductResolvedListResponseSchema
>;
