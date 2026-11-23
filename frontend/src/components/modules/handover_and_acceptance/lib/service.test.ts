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

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
    isHandoverAndAcceptance,
    isHandoverAndAcceptanceError,
    isHandoverAndAcceptanceFull,
    isPaginatedHandoverAndAcceptanceFullListResponse,
} from "./guards";
import { get_full, print, update } from "./service";
import type {
    HandoverAndAcceptanceFull,
    HandoverAndAcceptanceUserInput,
} from "./interface";

const customer = {
    id: "customer",
    name: "Customer",
    contact_name: null,
    email: null,
    phone_number: null,
    status: "active",
    customer_type: "natural",
    created_by_id: "user",
    created_by: "User",
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
    billing_address: null,
    mailing_address: null,
};
const raw = {
    id: "handover",
    display_id: "H-1",
    customer_id: "customer",
    item_make_id: null,
    item_model_id: null,
    item_identifier_type_id: "type",
    item_identifier_value: "serial",
    reported_issue: null,
    condition_at_receipt: null,
    included_accessories: null,
    authorized_work: null,
    repair_limit: null,
    currency_code: null,
    terms_version: null,
    accepted_by: "user",
    received_at: "now",
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
};
const item = { id: "type", name: "Serial", is_active: true, created_at: "now" };
const user = {
    id: "user",
    email: "user@example.com",
    first_name: null,
    last_name: "User",
    phone: null,
    status: "active",
    profile_picture_url: null,
    locale: "en-US",
    invited_by: null,
    email_verified_at: null,
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
};
const {
    customer_id: _customerId,
    item_make_id: _itemMakeId,
    item_model_id: _itemModelId,
    item_identifier_type_id: _itemIdentifierTypeId,
    ...resolvedFields
} = raw;
const full = {
    ...resolvedFields,
    customer,
    item_make: null,
    item_model: null,
    item_identifier_type: item,
    accepted_by: user,
};
const fields = {
    id: null,
    customer_id: null,
    item_make_id: null,
    new_item_make: null,
    item_model_id: null,
    new_item_model: null,
    item_identifier_type_id: null,
    new_item_identifier_type: null,
    item_identifier_value: null,
    reported_issue: null,
    condition_at_receipt: null,
    included_accessories: [null, "Invalid"],
    authorized_work: null,
    repair_limit: null,
    currency_code: null,
    terms_version: null,
    received_at: null,
};
const input: HandoverAndAcceptanceUserInput = {
    id: "handover",
    customerId: "customer",
    itemMakeId: "",
    newItemMake: "",
    itemModelId: "",
    newItemModel: "",
    itemIdentifierTypeId: "type",
    newItemIdentifierType: "",
    itemIdentifierValue: "serial",
    reportedIssue: "",
    conditionAtReceipt: "",
    includedAccessories: [],
    authorizedWork: "",
    repairLimit: "",
    currencyCode: "",
    termsVersion: "",
    receivedAt: "now",
};
const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
        status,
        headers: { "Content-Type": "application/json" },
    });

describe("handover schemas", () => {
    it("validates raw and resolved records including nested customer", () => {
        expect(isHandoverAndAcceptance(raw)).toBe(true);
        for (const identifier of [null, undefined, 123, {}]) {
            expect(
                isHandoverAndAcceptance({
                    ...raw,
                    item_identifier_type_id: identifier,
                }),
            ).toBe(false);
        }
        expect(
            isHandoverAndAcceptance({ ...raw, included_accessories: [1] }),
        ).toBe(false);
        expect(isHandoverAndAcceptanceFull(full)).toBe(true);
        expect(
            isHandoverAndAcceptanceFull({
                ...full,
                customer: { ...customer, id: null },
            }),
        ).toBe(false);
        expect(
            isHandoverAndAcceptanceFull({
                ...full,
                item_identifier_type: null,
            }),
        ).toBe(false);
        const { accepted_by: _acceptedBy, ...missingAcceptedBy } = full;
        expect(isHandoverAndAcceptanceFull(missingAcceptedBy)).toBe(false);
        for (const acceptedBy of [null, undefined, "user", {}]) {
            expect(
                isHandoverAndAcceptanceFull({
                    ...full,
                    accepted_by: acceptedBy,
                }),
            ).toBe(false);
        }
        for (const field of Object.keys(user) as Array<keyof typeof user>) {
            const { [field]: _omitted, ...missingField } = user;
            expect(
                isHandoverAndAcceptanceFull({
                    ...full,
                    accepted_by: missingField,
                }),
            ).toBe(false);
            expect(
                isHandoverAndAcceptanceFull({
                    ...full,
                    accepted_by: { ...user, [field]: 123 },
                }),
            ).toBe(false);
        }
        for (const field of [
            "id",
            "email",
            "status",
            "locale",
            "created_at",
            "updated_at",
        ]) {
            expect(
                isHandoverAndAcceptanceFull({
                    ...full,
                    accepted_by: { ...user, [field]: null },
                }),
            ).toBe(false);
        }
        expect(
            isHandoverAndAcceptanceFull({
                ...full,
                accepted_by: {
                    ...user,
                    first_name: "First",
                    invited_by: "inviter",
                    email_verified_at: "now",
                    extra: true,
                },
            }),
        ).toBe(true);
    });

    it("validates errors and paginated envelopes", () => {
        expect(isHandoverAndAcceptanceError(fields)).toBe(true);
        expect(
            isHandoverAndAcceptanceError({
                ...fields,
                included_accessories: null,
            }),
        ).toBe(false);
        const meta = { page: 1, limit: 25, total: 1 };
        expect(
            isPaginatedHandoverAndAcceptanceFullListResponse({
                meta,
                data: [full],
            }),
        ).toBe(true);
        expect(
            isPaginatedHandoverAndAcceptanceFullListResponse({ data: [] }),
        ).toBe(false);
        expect(
            isPaginatedHandoverAndAcceptanceFullListResponse({
                meta,
                data: null,
            }),
        ).toBe(false);
    });

    it("retains generic full-record field types", () => {
        const record: HandoverAndAcceptanceFull<string, number, boolean> = {
            ...full,
            customer: "customer",
            currency_code: 3,
            accepted_by: true,
        };
        expect(record.display_id).toBe("H-1");
    });
});

describe("handover API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it("maps input and validates 422 fields", async () => {
        fetchMock.mockResolvedValueOnce(
            json({ error: { message: "Invalid", fields } }, 422),
        );
        expect(await update(input, "token")).toEqual({
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid", fields } },
        });
        const [url, options] = fetchMock.mock.calls[0];
        expect(url).toBe("/api/handover_and_acceptance/update");
        expect(options?.method).toBe("PUT");
        expect(JSON.parse(options?.body as string)).toMatchObject({
            customer_id: "customer",
            item_identifier_value: "serial",
            included_accessories: [],
        });
        fetchMock.mockResolvedValueOnce(
            json({ error: { message: "Invalid", fields: {} } }, 422),
        );
        expect(await update(input, null)).toMatchObject({
            kind: "http_error",
            body: undefined,
        });
    });

    it("rejects malformed or empty success responses", async () => {
        fetchMock.mockResolvedValueOnce(json({ data: full }));
        expect(await get_full("handover", null)).toMatchObject({
            kind: "success",
            data: { data: full },
        });
        fetchMock.mockResolvedValueOnce(
            json({ data: { ...full, accepted_by: {} } }),
        );
        expect(await get_full("handover", null)).toMatchObject({
            kind: "invalid_response",
        });
        fetchMock.mockResolvedValueOnce(json({ data: null }));
        expect(await get_full("handover", null)).toMatchObject({
            kind: "invalid_response",
        });
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await get_full("handover", null)).toMatchObject({
            kind: "invalid_response",
        });
    });

    it("accepts PDF bytes and rejects JSON or no content", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const pdf = new Response(null, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(pdf, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValueOnce(pdf);
        expect(await print("handover", null)).toMatchObject({
            kind: "success",
            data: bytes,
        });
        fetchMock.mockResolvedValueOnce(json({ data: full }));
        expect(await print("handover", null)).toMatchObject({
            kind: "invalid_response",
        });
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await print("handover", null)).toMatchObject({
            kind: "invalid_response",
        });
    });
});
