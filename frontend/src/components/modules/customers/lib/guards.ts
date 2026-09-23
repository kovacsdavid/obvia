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
    isCommonResponse,
    isFormError,
    isFormErrorV2,
    isPaginatedDataResponse,
    isSimpleError,
    isSimpleMessageData,
} from "@/lib/interface.ts";
import type {
    CreateCustomerResponse,
    Customer,
    CustomerErrors,
    CustomerFull,
    CustomerResolvedList,
    CustomerFullResponse,
    CustomerResponse,
    DeleteCustomerResponse,
    PaginatedCustomerResolvedListResponse,
    UpdateCustomerResponse,
} from "@/components/modules/customers/lib/interface.ts";
import {
    isAddress,
    isAddressError,
} from "@/components/modules/address/lib/guards";

export function isCreateCustomerResponse(
    data: unknown,
): data is CreateCustomerResponse {
    return isCommonResponse(data, isCustomer, (data: unknown) =>
        isFormErrorV2<CustomerErrors>(data, isCustomerErrors),
    );
}

export function isUpdateCustomerResponse(
    data: unknown,
): data is UpdateCustomerResponse {
    return isCommonResponse(data, isCustomer, isFormError);
}

export function isDeleteCustomerResponse(
    data: unknown,
): data is DeleteCustomerResponse {
    return isCommonResponse(data, isSimpleMessageData, isSimpleError);
}

export function isCustomerFull(data: unknown): data is CustomerFull {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "name" in data &&
        typeof data.name === "string" &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        typeof data.email === "string" &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "status" in data &&
        typeof data.status === "string" &&
        "customer_type" in data &&
        typeof data.customer_type === "string" &&
        "created_by_id" in data &&
        typeof data.created_by_id === "string" &&
        "created_by" in data &&
        typeof data.created_by === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string") &&
        "billing_address" in data &&
        (data.billing_address === null || isAddress(data.billing_address)) &&
        "mailing_address" in data &&
        (data.mailing_address === null || isAddress(data.mailing_address))
    );
}

export function isCustomerResolvedResponse(
    data: unknown,
): data is CustomerFullResponse {
    return isCommonResponse(data, isCustomerFull, isSimpleError);
}

export function isCustomerResolvedList(
    data: unknown,
): data is CustomerResolvedList {
    return Array.isArray(data) && data.every((item) => isCustomerFull(item));
}

export function isPaginatedCustomerResolvedListResponse(
    data: unknown,
): data is PaginatedCustomerResolvedListResponse {
    return isPaginatedDataResponse(data, isCustomerResolvedList);
}

export function isCustomer(data: unknown): data is Customer {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "name" in data &&
        typeof data.name === "string" &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        typeof data.email === "string" &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "status" in data &&
        typeof data.status === "string" &&
        "customer_type" in data &&
        typeof data.customer_type === "string" &&
        "created_by_id" in data &&
        typeof data.created_by_id === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string") &&
        "billing_address" in data &&
        (data.billing_address === null ||
            typeof data.billing_address === "string") &&
        "mailing_address" in data &&
        (data.mailing_address === null ||
            typeof data.mailing_address === "string")
    );
}

export function isCustomerResponse(data: unknown): data is CustomerResponse {
    return isCommonResponse(data, isCustomer, isSimpleError);
}

export const isCustomerErrors = (data: unknown): data is CustomerErrors => {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        (data.id === null || typeof data.id === "string") &&
        "name" in data &&
        (data.name === null || typeof data.name === "string") &&
        "contact_name" in data &&
        (data.contact_name === null || typeof data.contact_name === "string") &&
        "email" in data &&
        (data.email === null || typeof data.email === "string") &&
        "phone_number" in data &&
        (data.phone_number === null || typeof data.phone_number === "string") &&
        "status" in data &&
        (data.status === null || typeof data.status === "string") &&
        "customer_type" in data &&
        (data.customer_type === null ||
            typeof data.customer_type === "string") &&
        "billing_address" in data &&
        isAddressError(data.billing_address) &&
        "mailing_address" in data &&
        isAddressError(data.mailing_address)
    );
};
