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
    globalRequestTimeout,
    unexpectedError,
    unexpectedFormError,
} from "@/services/utils/consts.ts";
import {
    type CreateTaxResponse,
    type DeleteTaxResponse,
    type PaginatedTaxResolvedListResponse,
    type TaxResolvedResponse,
    type TaxResponse,
    type TaxUserInput,
    type UpdateTaxResponse,
} from "@/components/modules/taxes/lib/interface.ts";
import {
    isSelectOptionListResponse,
    type ProcessedJsonResponse,
    ProcessJsonResponse,
    type SelectOptionListResponse,
} from "@/lib/interface.ts";
import {
    isCreateTaxResponse,
    isDeleteTaxResponse,
    isPaginatedTaxResolvedListResponse,
    isTaxResolvedResponse,
    isTaxResponse,
    isUpdateTaxResponse,
} from "@/components/modules/taxes/lib/guards.ts";

export async function create(
    {
        id,
        rate,
        description,
        countryCode,
        taxCategory,
        isRateApplicable,
        legalText,
        reportingCode,
        isDefault,
        status,
    }: TaxUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<CreateTaxResponse>> {
    return await fetch(`/api/taxes/create`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            rate,
            description,
            country_code: countryCode,
            tax_category: taxCategory,
            is_rate_applicable: isRateApplicable,
            legal_text: legalText,
            reporting_code: reportingCode,
            is_default: isDefault,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isCreateTaxResponse)) ??
            unexpectedFormError
        );
    });
}

export async function update(
    {
        id,
        rate,
        description,
        countryCode,
        taxCategory,
        isRateApplicable,
        legalText,
        reportingCode,
        isDefault,
        status,
    }: TaxUserInput,
    token: string | null,
): Promise<ProcessedJsonResponse<UpdateTaxResponse>> {
    return await fetch(`/api/taxes/update`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
        body: JSON.stringify({
            id,
            rate,
            description,
            country_code: countryCode,
            tax_category: taxCategory,
            is_rate_applicable: isRateApplicable,
            legal_text: legalText,
            reporting_code: reportingCode,
            is_default: isDefault,
            status,
        }),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isUpdateTaxResponse)) ??
            unexpectedFormError
        );
    });
}

export async function list(
    query: string | null,
    token: string | null,
): Promise<ProcessedJsonResponse<PaginatedTaxResolvedListResponse>> {
    const uri =
        query === null ? `/api/taxes/list` : `/api/taxes/list?q=${query}`;
    return await fetch(uri, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(
                response,
                isPaginatedTaxResolvedListResponse,
            )) ?? unexpectedError
        );
    });
}

export async function get_resolved(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<TaxResolvedResponse>> {
    return await fetch(`/api/taxes/get_resolved?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isTaxResolvedResponse)) ??
            unexpectedError
        );
    });
}

export async function get(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<TaxResponse>> {
    return await fetch(`/api/taxes/get?uuid=${uuid}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isTaxResponse)) ??
            unexpectedError
        );
    });
}

export async function deleteItem(
    uuid: string,
    token: string | null,
): Promise<ProcessedJsonResponse<DeleteTaxResponse>> {
    return await fetch(`/api/taxes/delete?uuid=${uuid}`, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isDeleteTaxResponse)) ??
            unexpectedError
        );
    });
}

export async function select_list(
    list: string,
    token: string | null,
): Promise<ProcessedJsonResponse<SelectOptionListResponse>> {
    return await fetch(`/api/taxes/select_list?list=${list}`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            ...(token ? { Authorization: `Bearer ${token}` } : {}),
        },
        signal: AbortSignal.timeout(globalRequestTimeout),
    }).then(async (response: Response) => {
        return (
            (await ProcessJsonResponse(response, isSelectOptionListResponse)) ??
            unexpectedError
        );
    });
}
