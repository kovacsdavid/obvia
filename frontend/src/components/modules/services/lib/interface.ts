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
    serviceUserInputSchema,
    serviceErrorSchema,
    serviceSchema,
    serviceResolvedSchema,
    serviceResolvedListSchema,
    serviceResponseSchema,
    createServiceResponseSchema,
    updateServiceResponseSchema,
    deleteServiceResponseSchema,
    serviceResolvedResponseSchema,
    paginatedServiceResolvedListResponseSchema,
} from "./schemas";

export type ServiceUserInput = z.infer<typeof serviceUserInputSchema>;
export type ServiceError = z.infer<typeof serviceErrorSchema>;
export type Service = z.infer<typeof serviceSchema>;
export type ServiceResolved = z.infer<typeof serviceResolvedSchema>;
export type ServiceResolvedList = z.infer<typeof serviceResolvedListSchema>;
export type CreateServiceResponse = z.infer<typeof createServiceResponseSchema>;
export type UpdateServiceResponse = z.infer<typeof updateServiceResponseSchema>;
export type DeleteServiceResponse = z.infer<typeof deleteServiceResponseSchema>;
export type ServiceResponse = z.infer<typeof serviceResponseSchema>;
export type ServiceResolvedResponse = z.infer<
    typeof serviceResolvedResponseSchema
>;
export type PaginatedServiceResolvedListResponse = z.infer<
    typeof paginatedServiceResolvedListResponseSchema
>;
