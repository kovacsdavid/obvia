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
    createCustomerResponseSchema,
    customerErrorsSchema,
    customerFullResponseSchema,
    customerFullSchema,
    customerResolvedListSchema,
    customerResponseSchema,
    customerSchema,
    deleteCustomerResponseSchema,
    paginatedCustomerResolvedListResponseSchema,
    updateCustomerResponseSchema,
} from "./schemas";

export const isCreateCustomerResponse = schemaGuard(
    createCustomerResponseSchema,
);
export const isUpdateCustomerResponse = schemaGuard(
    updateCustomerResponseSchema,
);
export const isDeleteCustomerResponse = schemaGuard(
    deleteCustomerResponseSchema,
);
export const isCustomerFull = schemaGuard(customerFullSchema);
export const isCustomerFullResponse = schemaGuard(customerFullResponseSchema);
export const isCustomerResolvedList = schemaGuard(customerResolvedListSchema);
export const isPaginatedCustomerResolvedListResponse = schemaGuard(
    paginatedCustomerResolvedListResponseSchema,
);
export const isCustomer = schemaGuard(customerSchema);
export const isCustomerResponse = schemaGuard(customerResponseSchema);
export const isCustomerErrors = schemaGuard(customerErrorsSchema);
