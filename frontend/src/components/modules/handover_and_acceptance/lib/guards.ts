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

import { isCustomer } from "@/components/modules/customers/lib/guards";
import type { Currency } from "@/components/modules/currencies/lib/interface";
import {
    isCommonResponse,
    isFormError,
    isPaginatedDataResponse,
    isSimpleError,
    isSimpleMessageData,
} from "@/lib/interface.ts";
import type {
    CreateHandoverAndAcceptanceResponse,
    DeleteHandoverAndAcceptanceResponse,
    HandoverAndAcceptance,
    HandoverAndAcceptanceError,
    HandoverAndAcceptanceFull,
    HandoverAndAcceptanceFullList,
    HandoverAndAcceptanceFullResponse,
    HandoverAndAcceptanceResponse,
    ItemIdentifierType,
    ItemMake,
    ItemModelFull,
    PaginatedHandoverAndAcceptanceFullListResponse,
    UpdateHandoverAndAcceptanceResponse,
} from "@/components/modules/handover_and_acceptance/lib/interface.ts";

export { isSelectOptionListResponse } from "@/lib/interface.ts";

export function isCreateHandoverAndAcceptanceResponse(
    data: unknown,
): data is CreateHandoverAndAcceptanceResponse {
    return isCommonResponse(data, isHandoverAndAcceptance, isFormError);
}

export function isUpdateHandoverAndAcceptanceResponse(
    data: unknown,
): data is UpdateHandoverAndAcceptanceResponse {
    return isCommonResponse(data, isHandoverAndAcceptance, isFormError);
}

export function isDeleteHandoverAndAcceptanceResponse(
    data: unknown,
): data is DeleteHandoverAndAcceptanceResponse {
    return isCommonResponse(data, isSimpleMessageData, isSimpleError);
}

export function isHandoverAndAcceptance(
    data: unknown,
): data is HandoverAndAcceptance {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "display_id" in data &&
        typeof data.display_id === "string" &&
        "customer_id" in data &&
        typeof data.customer_id === "string" &&
        "item_make_id" in data &&
        (data.item_make_id === null || typeof data.item_make_id === "string") &&
        "new_item_make" in data &&
        (data.new_item_make === null ||
            typeof data.new_item_make === "string") &&
        "item_model_id" in data &&
        (data.item_model_id === null ||
            typeof data.item_model_id === "string") &&
        "new_item_model" in data &&
        (data.new_item_model === null ||
            typeof data.new_item_model === "string") &&
        "item_identifier_type_id" in data &&
        (data.item_identifier_type_id === null ||
            typeof data.item_identifier_type_id === "string") &&
        "new_item_identifier_type" in data &&
        (data.new_item_identifier_type === null ||
            typeof data.new_item_identifier_type === "string") &&
        "item_identifier_value" in data &&
        typeof data.item_identifier_value === "string" &&
        "reported_issue" in data &&
        (data.reported_issue === null ||
            typeof data.reported_issue === "string") &&
        "condition_at_receipt" in data &&
        (data.condition_at_receipt === null ||
            typeof data.condition_at_receipt === "string") &&
        "included_accessories" in data &&
        (data.included_accessories === null ||
            (Array.isArray(data.included_accessories) &&
                data.included_accessories.every(
                    (item) => typeof item === "string",
                ))) &&
        "authorized_work" in data &&
        (data.authorized_work === null ||
            typeof data.authorized_work === "string") &&
        "repair_limit" in data &&
        (data.repair_limit === null || typeof data.repair_limit === "string") &&
        "currency_code" in data &&
        (data.currency_code === null ||
            typeof data.currency_code === "string") &&
        "terms_version" in data &&
        (data.terms_version === null ||
            typeof data.terms_version === "string") &&
        "accepted_by" in data &&
        typeof data.accepted_by === "string" &&
        "received_at" in data &&
        typeof data.received_at === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string")
    );
}

export function isHandoverAndAcceptanceError(
    data: unknown,
): data is HandoverAndAcceptanceError {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        (data.id === null || typeof data.id === "string") &&
        "customer_id" in data &&
        (data.customer_id === null || typeof data.customer_id === "string") &&
        "item_make_id" in data &&
        (data.item_make_id === null || typeof data.item_make_id === "string") &&
        "new_item_make" in data &&
        (data.new_item_make === null ||
            typeof data.new_item_make === "string") &&
        "item_model_id" in data &&
        (data.item_model_id === null ||
            typeof data.item_model_id === "string") &&
        "new_item_model" in data &&
        (data.new_item_model === null ||
            typeof data.new_item_model === "string") &&
        "item_identifier_type_id" in data &&
        (data.item_identifier_type_id === null ||
            typeof data.item_identifier_type_id === "string") &&
        "new_item_identifier_type" in data &&
        (data.new_item_identifier_type === null ||
            typeof data.new_item_identifier_type === "string") &&
        "item_identifier_value" in data &&
        (data.item_identifier_value === null ||
            typeof data.item_identifier_value === "string") &&
        "reported_issue" in data &&
        (data.reported_issue === null ||
            typeof data.reported_issue === "string") &&
        "condition_at_receipt" in data &&
        (data.condition_at_receipt === null ||
            typeof data.condition_at_receipt === "string") &&
        "included_accessories" in data &&
        Array.isArray(data.included_accessories) &&
        data.included_accessories.every(
            (item) => item === null || typeof item === "string",
        ) &&
        "authorized_work" in data &&
        (data.authorized_work === null ||
            typeof data.authorized_work === "string") &&
        "repair_limit" in data &&
        (data.repair_limit === null || typeof data.repair_limit === "string") &&
        "currency_code" in data &&
        (data.currency_code === null ||
            typeof data.currency_code === "string") &&
        "terms_version" in data &&
        (data.terms_version === null ||
            typeof data.terms_version === "string") &&
        "received_at" in data &&
        (data.received_at === null || typeof data.received_at === "string")
    );
}

export function isHandoverAndAcceptanceResponse(
    data: unknown,
): data is HandoverAndAcceptanceResponse {
    return isCommonResponse(data, isHandoverAndAcceptance, isSimpleError);
}

export function isItemIdentifierType(
    data: unknown,
): data is ItemIdentifierType {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "name" in data &&
        typeof data.name === "string" &&
        "is_active" in data &&
        typeof data.is_active === "boolean" &&
        "created_at" in data &&
        typeof data.created_at === "string"
    );
}

export function isItemMake(data: unknown): data is ItemMake {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "name" in data &&
        typeof data.name === "string" &&
        "is_active" in data &&
        typeof data.is_active === "boolean" &&
        "created_at" in data &&
        typeof data.created_at === "string"
    );
}

export function isItemModelFull(data: unknown): data is ItemModelFull {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "make" in data &&
        isItemMake(data.make) &&
        "name" in data &&
        typeof data.name === "string" &&
        "is_active" in data &&
        typeof data.is_active === "boolean" &&
        "created_at" in data &&
        typeof data.created_at === "string"
    );
}

function isCurrency(data: unknown): data is Currency {
    return (
        typeof data === "object" &&
        data !== null &&
        "code" in data &&
        typeof data.code === "string" &&
        "number" in data &&
        typeof data.number === "string" &&
        "name" in data &&
        typeof data.name === "string"
    );
}

export function isHandoverAndAcceptanceFull(
    data: unknown,
): data is HandoverAndAcceptanceFull {
    return (
        typeof data === "object" &&
        data !== null &&
        "id" in data &&
        typeof data.id === "string" &&
        "display_id" in data &&
        typeof data.display_id === "string" &&
        "customer" in data &&
        isCustomer(data.customer) &&
        "item_make" in data &&
        (data.item_make === null || isItemMake(data.item_make)) &&
        "item_model" in data &&
        (data.item_model === null || isItemModelFull(data.item_model)) &&
        "item_identifier_type" in data &&
        isItemIdentifierType(data.item_identifier_type) &&
        "item_identifier_value" in data &&
        typeof data.item_identifier_value === "string" &&
        "reported_issue" in data &&
        (data.reported_issue === null ||
            typeof data.reported_issue === "string") &&
        "condition_at_receipt" in data &&
        (data.condition_at_receipt === null ||
            typeof data.condition_at_receipt === "string") &&
        "included_accessories" in data &&
        (data.included_accessories === null ||
            (Array.isArray(data.included_accessories) &&
                data.included_accessories.every(
                    (item) => typeof item === "string",
                ))) &&
        "authorized_work" in data &&
        (data.authorized_work === null ||
            typeof data.authorized_work === "string") &&
        "repair_limit" in data &&
        (data.repair_limit === null || typeof data.repair_limit === "string") &&
        "currency_code" in data &&
        (data.currency_code === null || isCurrency(data.currency_code)) &&
        "terms_version" in data &&
        (data.terms_version === null ||
            typeof data.terms_version === "string") &&
        "accepted_by" in data &&
        "received_at" in data &&
        typeof data.received_at === "string" &&
        "created_at" in data &&
        typeof data.created_at === "string" &&
        "updated_at" in data &&
        typeof data.updated_at === "string" &&
        "deleted_at" in data &&
        (data.deleted_at === null || typeof data.deleted_at === "string")
    );
}

export function isHandoverAndAcceptanceFullResponse(
    data: unknown,
): data is HandoverAndAcceptanceFullResponse {
    return isCommonResponse(data, isHandoverAndAcceptanceFull, isSimpleError);
}

export function isHandoverAndAcceptanceFullList(
    data: unknown,
): data is HandoverAndAcceptanceFullList {
    return (
        Array.isArray(data) &&
        data.every((item) => isHandoverAndAcceptanceFull(item))
    );
}

export function isPaginatedHandoverAndAcceptanceFullListResponse(
    data: unknown,
): data is PaginatedHandoverAndAcceptanceFullListResponse {
    return isPaginatedDataResponse(
        data,
        isHandoverAndAcceptanceFullList,
        isSimpleError,
    );
}
