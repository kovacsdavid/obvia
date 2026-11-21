/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import type { Currency as CurrencyInterface } from "@/components/modules/currencies/lib/interface";
import type { CustomerFull as CustomerInterface } from "@/components/modules/customers/lib/interface";
import {
    type CommonResponse,
    type FormError,
    type PaginatedDataResponse,
    type SimpleError,
    type SimpleMessageData,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";

export type { SelectOptionListResponse };

export interface HandoverAndAcceptanceUserInput {
    id: string | null;
    customerId: string;
    itemMakeId: string;
    newItemMake: string;
    itemModelId: string;
    newItemModel: string;
    itemIdentifierTypeId: string;
    newItemIdentifierType: string;
    itemIdentifierValue: string;
    reportedIssue: string;
    conditionAtReceipt: string;
    includedAccessories: string[];
    authorizedWork: string;
    repairLimit: string;
    currencyCode: string;
    termsVersion: string;
    receivedAt: string;
}

export interface HandoverAndAcceptance {
    id: string;
    display_id: string;
    customer_id: string;
    item_make_id: string | null;
    item_model_id: string | null;
    item_identifier_type_id: string;
    item_identifier_value: string;
    reported_issue: string | null;
    condition_at_receipt: string | null;
    included_accessories: string[] | null;
    authorized_work: string | null;
    repair_limit: string | null;
    currency_code: string | null;
    terms_version: string | null;
    accepted_by: string;
    received_at: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
}

export interface HandoverAndAcceptanceError {
    id: string | null;
    customer_id: string | null;
    item_make_id: string | null;
    new_item_make: string | null;
    item_model_id: string | null;
    new_item_model: string | null;
    item_identifier_type_id: string | null;
    new_item_identifier_type: string | null;
    item_identifier_value: string | null;
    reported_issue: string | null;
    condition_at_receipt: string | null;
    included_accessories: (string | null)[];
    authorized_work: string | null;
    repair_limit: string | null;
    currency_code: string | null;
    terms_version: string | null;
    received_at: string | null;
}

export interface ItemIdentifierType {
    id: string;
    name: string;
    is_active: boolean;
    created_at: string;
}

export interface ItemMake {
    id: string;
    name: string;
    is_active: boolean;
    created_at: string;
}

export interface ItemModelFull {
    id: string;
    make: ItemMake;
    name: string;
    is_active: boolean;
    created_at: string;
}

export interface HandoverAndAcceptanceFull<
    Customer = CustomerInterface,
    Currency = CurrencyInterface,
    User = unknown,
> {
    id: string;
    display_id: string;
    customer: Customer;
    item_make: ItemMake | null;
    item_model: ItemModelFull | null;
    item_identifier_type: ItemIdentifierType;
    item_identifier_value: string;
    reported_issue: string | null;
    condition_at_receipt: string | null;
    included_accessories: string[] | null;
    authorized_work: string | null;
    repair_limit: string | null;
    currency_code: Currency | null;
    terms_version: string | null;
    accepted_by: User;
    received_at: string;
    created_at: string;
    updated_at: string;
    deleted_at: string | null;
}

export type CreateHandoverAndAcceptanceResponse = CommonResponse<
    HandoverAndAcceptance,
    FormError
>;
export type UpdateHandoverAndAcceptanceResponse = CommonResponse<
    HandoverAndAcceptance,
    FormError
>;
export type DeleteHandoverAndAcceptanceResponse = CommonResponse<
    SimpleMessageData,
    SimpleError
>;
export type HandoverAndAcceptanceResponse = CommonResponse<
    HandoverAndAcceptance,
    SimpleError
>;
export type HandoverAndAcceptanceFullResponse = CommonResponse<
    HandoverAndAcceptanceFull,
    SimpleError
>;
export type HandoverAndAcceptanceFullList = HandoverAndAcceptanceFull[];
export type PaginatedHandoverAndAcceptanceFullListResponse =
    PaginatedDataResponse<HandoverAndAcceptanceFullList, SimpleError>;
