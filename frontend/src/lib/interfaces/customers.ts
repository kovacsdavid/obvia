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
  isCommonResponse, isFormError, isSimpleMessageData,
  type SimpleMessageData
} from "@/lib/interfaces/common.ts";

export interface CreateCustomer {
  name: string
  contactName: string
  email: string
  phoneNumber: string
  status: string | undefined
  customerType: string | undefined,
}

export interface Customer {
  id: string // TODO
}

export function isCreateCustomerResponse(data: unknown): data is CommonResponse<SimpleMessageData, FormError> {
  return isCommonResponse<SimpleMessageData, FormError>(
    data,
    isSimpleMessageData,
    isFormError
  )
}

