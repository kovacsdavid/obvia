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
  type SimpleMessageData,
} from "@/lib/interfaces/common.ts";

export interface ServiceUserInput {
  id: string | null;
  name: string;
  description: string | null;
  defaultPrice: string | null;
  defaultTaxId: string | null;
  currencyCode: string | null;
  status: string;
}

export interface Service {
  id: string;
  name: string;
  description: string | null;
  default_price: string | null;
  default_tax_id: string | null;
  currency_code: string | null;
  status: string;
  created_by_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface ServiceResolved {
  id: string;
  name: string;
  description: string | null;
  default_price: string | null;
  default_tax_id: string | null;
  default_tax: string | null;
  currency_code: string | null;
  status: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type CreateServiceResponse = CommonResponse<Service, FormError>;
export type UpdateServiceResponse = CommonResponse<Service, FormError>;
export type DeleteServiceResponse = CommonResponse<
  SimpleMessageData,
  SimpleError
>;
export type ServiceResolvedList = ServiceResolved[];
export type ServiceResponse = CommonResponse<Service, SimpleError>;
export type ServiceResolvedResponse = CommonResponse<
  ServiceResolved,
  SimpleError
>;
export type PaginatedServiceResolvedListResponse = PaginatedDataResponse<
  ServiceResolvedList,
  SimpleError
>;
