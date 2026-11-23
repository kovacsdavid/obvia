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
    createCustomerResponseSchema,
    customerErrorsSchema,
    customerFullResponseSchema,
    customerFullSchema,
    customerResolvedListSchema,
    customerResponseSchema,
    customerSchema,
    customerUserInputSchema,
    deleteCustomerResponseSchema,
    paginatedCustomerResolvedListResponseSchema,
    updateCustomerResponseSchema,
} from "./schemas";

export type CustomerUserInput = z.infer<typeof customerUserInputSchema>;
export type Customer = z.infer<typeof customerSchema>;
export type CustomerErrors = z.infer<typeof customerErrorsSchema>;
export type CustomerFull = z.infer<typeof customerFullSchema>;
export type CustomerResolvedList = z.infer<typeof customerResolvedListSchema>;
export type CreateCustomerResponse = z.infer<
    typeof createCustomerResponseSchema
>;
export type UpdateCustomerResponse = z.infer<
    typeof updateCustomerResponseSchema
>;
export type DeleteCustomerResponse = z.infer<
    typeof deleteCustomerResponseSchema
>;
export type CustomerResponse = z.infer<typeof customerResponseSchema>;
export type CustomerFullResponse = z.infer<typeof customerFullResponseSchema>;
export type PaginatedCustomerResolvedListResponse = z.infer<
    typeof paginatedCustomerResolvedListResponseSchema
>;
