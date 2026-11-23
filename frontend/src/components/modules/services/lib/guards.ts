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
    serviceErrorSchema,
    serviceSchema,
    serviceResolvedSchema,
    serviceResolvedListSchema,
    serviceResponseSchema,
    serviceResolvedResponseSchema,
    createServiceResponseSchema,
    updateServiceResponseSchema,
    deleteServiceResponseSchema,
    paginatedServiceResolvedListResponseSchema,
} from "./schemas";

export const isServiceError = schemaGuard(serviceErrorSchema);
export const isService = schemaGuard(serviceSchema);
export const isServiceResolved = schemaGuard(serviceResolvedSchema);
export const isServiceResolvedList = schemaGuard(serviceResolvedListSchema);
export const isServiceResponse = schemaGuard(serviceResponseSchema);
export const isServiceResolvedResponse = schemaGuard(
    serviceResolvedResponseSchema,
);
export const isCreateServiceResponse = schemaGuard(createServiceResponseSchema);
export const isUpdateServiceResponse = schemaGuard(updateServiceResponseSchema);
export const isDeleteServiceResponse = schemaGuard(deleteServiceResponseSchema);
export const isPaginatedServiceResolvedListResponse = schemaGuard(
    paginatedServiceResolvedListResponseSchema,
);
