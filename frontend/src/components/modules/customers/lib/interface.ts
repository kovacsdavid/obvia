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

export interface CustomerUserInput {
  id: string | null;
  name: string;
  contactName: string;
  email: string;
  phoneNumber: string;
  status: string | undefined;
  customerType: string | undefined;
}

export interface Customer {
  id: string;
  name: string;
  contact_name: string | null;
  email: string;
  phone_number: string | null;
  status: string;
  customer_type: string;
  created_by_id: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface CustomerResolved {
  id: string;
  name: string;
  contact_name: string | null;
  email: string;
  phone_number: string | null;
  status: string;
  customer_type: string;
  created_by_id: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export type CreateCustomerResponse = CommonResponse<Customer, FormError>;
export type UpdateCustomerResponse = CommonResponse<Customer, FormError>;
export type DeleteCustomerResponse = CommonResponse<
  SimpleMessageData,
  SimpleError
>;
export type CustomerResolvedList = CustomerResolved[];
export type CustomerResponse = CommonResponse<Customer, SimpleError>;
export type CustomerResolvedResponse = CommonResponse<
  CustomerResolved,
  SimpleError
>;
export type PaginatedCustomerResolvedListResponse = PaginatedDataResponse<
  CustomerResolvedList,
  SimpleError
>;
