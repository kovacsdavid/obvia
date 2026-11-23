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
import * as api from "./service";
import {
    isCustomerErrors,
    isCustomerFull,
    isCustomerResponse,
    isPaginatedCustomerResolvedListResponse,
} from "./guards";
import { customerUserInputSchema } from "./schemas";
import type { CustomerUserInput } from "./interface";

const address = {
    id: "address-1",
    type: "billing",
    country_code: "HU",
    postal_code: "1234",
    settlement: "Budapest",
    mailbox: null,
    topographic_number: null,
    name_of_public_space: null,
    type_of_public_space: null,
    house_number: null,
    building: null,
    stairway: null,
    floor: null,
    door: null,
};

const customer = {
    id: "customer-1",
    name: "Customer",
    contact_name: null,
    email: null,
    phone_number: null,
    status: "active",
    customer_type: "natural",
    created_by_id: "user-1",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
    billing_address: "address-1",
    mailing_address: null,
};

const fullCustomer = {
    ...customer,
    created_by: "User",
    billing_address: address,
    mailing_address: null,
};

const addressErrors = Object.fromEntries(
    Object.keys(address).map((field) => [field, null]),
);
const fields = {
    id: null,
    name: "Required",
    contact_name: null,
    email: null,
    phone_number: null,
    status: null,
    customer_type: null,
    billing_address: addressErrors,
    mailing_address: addressErrors,
};
const input: CustomerUserInput = {
    id: null,
    name: "Customer",
    contactName: "Contact",
    email: "customer@example.com",
    phoneNumber: "123",
    status: undefined,
    customerType: "natural",
    billingAddress: address,
    mailingAddress: null,
};
const meta = { page: 1, limit: 25, total: 1 };

function jsonResponse(data: unknown, status = 200) {
    return new Response(JSON.stringify(data), {
        status,
        headers: { "Content-Type": "application/json" },
    });
}

describe("customer schemas", () => {
    it("accepts the UI input shape without imposing form validation", () => {
        expect(customerUserInputSchema.safeParse(input).success).toBe(true);
        expect(
            customerUserInputSchema.safeParse({ ...input, name: "" }).success,
        ).toBe(true);
        expect(
            customerUserInputSchema.safeParse({ ...input, name: 1 }).success,
        ).toBe(false);
    });

    it("validates customers and nested addresses, preserving extra fields", () => {
        expect(
            isCustomerResponse({
                meta: null,
                data: { ...customer, extra: true },
            }),
        ).toBe(true);
        expect(isCustomerFull(fullCustomer)).toBe(true);
        expect(
            isCustomerFull({ ...fullCustomer, billing_address: "address-1" }),
        ).toBe(false);
        expect(
            isCustomerFull({
                ...fullCustomer,
                billing_address: { ...address, id: null },
            }),
        ).toBe(false);
        expect(
            isCustomerFull({ ...fullCustomer, mailing_address: undefined }),
        ).toBe(false);
    });

    it("requires all nested validation error fields", () => {
        expect(isCustomerErrors(fields)).toBe(true);
        const { house_number: _houseNumber, ...missingHouseNumber } =
            addressErrors;
        expect(
            isCustomerErrors({
                ...fields,
                billing_address: missingHouseNumber,
            }),
        ).toBe(false);
        expect(isCustomerErrors({ ...fields, mailing_address: null })).toBe(
            false,
        );
        expect(isCustomerErrors({ ...fields, name: 123 })).toBe(false);
    });

    it("requires data and list metadata in success envelopes", () => {
        expect(isCustomerResponse({})).toBe(false);
        expect(isCustomerResponse({ error: { message: "error" } })).toBe(false);
        expect(isCustomerResponse({ data: null })).toBe(false);
        expect(
            isPaginatedCustomerResolvedListResponse({
                meta,
                data: [fullCustomer],
            }),
        ).toBe(true);
        expect(
            isPaginatedCustomerResolvedListResponse({ meta, data: [] }),
        ).toBe(true);
        expect(isPaginatedCustomerResolvedListResponse({ data: [] })).toBe(
            false,
        );
        expect(
            isPaginatedCustomerResolvedListResponse({ meta: null, data: [] }),
        ).toBe(false);
        expect(
            isPaginatedCustomerResolvedListResponse({ meta, data: [customer] }),
        ).toBe(false);
    });
});

describe("customer API", () => {
    const fetchMock = vi.fn<typeof fetch>();

    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it.each([
        ["create", api.create, "/api/customers/create", "POST"],
        ["update", api.update, "/api/customers/update", "PUT"],
    ])(
        "sends %s inputs and returns form errors",
        async (_name, invoke, url, method) => {
            fetchMock.mockResolvedValue(
                jsonResponse({ error: { message: "Invalid", fields } }, 422),
            );
            const controller = new AbortController();
            const result = await invoke(input, "token", controller.signal);
            const [actualUrl, options] = fetchMock.mock.calls[0];
            expect(actualUrl).toBe(url);
            expect(options?.method).toBe(method);
            expect(options?.headers).toMatchObject({
                Authorization: "Bearer token",
            });
            expect(JSON.parse(options?.body as string)).toEqual({
                id: null,
                name: "Customer",
                contact_name: "Contact",
                email: "customer@example.com",
                phone_number: "123",
                status: null,
                customer_type: "natural",
                billing_address: address,
                mailing_address: null,
            });
            controller.abort();
            expect(options?.signal?.aborted).toBe(true);
            expect(result).toEqual({
                kind: "http_error",
                status: 422,
                body: { error: { message: "Invalid", fields } },
            });
        },
    );

    it("accepts error bodies without fields and rejects malformed fields", async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ error: { message: "Invalid" } }, 422),
        );
        expect(await api.create(input, null)).toEqual({
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid" } },
        });
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ error: { message: "Invalid", fields: {} } }, 422),
        );
        expect(await api.update(input, null)).toEqual({
            kind: "http_error",
            status: 422,
            body: undefined,
        });
    });

    it("returns valid success data and rejects malformed success envelopes", async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ meta: null, data: customer }),
        );
        expect(await api.get("customer-1", null)).toEqual({
            kind: "success",
            status: 200,
            data: { meta: null, data: customer },
        });
        expect(fetchMock.mock.calls[0][0]).toBe(
            "/api/customers/get?uuid=customer-1",
        );
        for (const body of [
            {},
            { data: null },
            { error: { message: "error" } },
            { data: {} },
        ]) {
            fetchMock.mockResolvedValueOnce(jsonResponse(body));
            expect(await api.get("customer-1", null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        }
        fetchMock.mockResolvedValueOnce(
            new Response("{", {
                headers: { "Content-Type": "application/json" },
            }),
        );
        expect(await api.get("customer-1", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
        fetchMock.mockResolvedValueOnce(new Response("not json"));
        expect(await api.get("customer-1", null)).toEqual({
            kind: "invalid_response",
            status: 200,
        });
    });

    it("validates list metadata and full customer responses", async () => {
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ meta, data: [fullCustomer] }),
        );
        expect((await api.list(null, null)).kind).toBe("success");
        expect(fetchMock.mock.calls[0][0]).toBe("/api/customers/list");
        fetchMock.mockResolvedValueOnce(jsonResponse({ data: [] }));
        expect((await api.list("page=2", null)).kind).toBe("invalid_response");
        expect(fetchMock.mock.calls[1][0]).toBe("/api/customers/list?q=page=2");
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ meta: null, data: fullCustomer }),
        );
        expect((await api.get_full("customer-1", null)).kind).toBe("success");
        fetchMock.mockResolvedValueOnce(
            jsonResponse({ meta: null, data: { message: "Deleted" } }),
        );
        expect((await api.deleteItem("customer-1", null)).kind).toBe("success");
    });

    it("preserves PDF response handling", async () => {
        const bytes = new Uint8Array([37, 80, 68, 70]);
        const response = new Response(bytes, {
            headers: { "Content-Type": "application/pdf" },
        });
        Object.defineProperty(response, "bytes", {
            value: vi.fn().mockResolvedValue(bytes),
        });
        fetchMock.mockResolvedValueOnce(response);
        expect(await api.print("customer-1", null)).toEqual({
            kind: "success",
            status: 200,
            data: bytes,
        });
        fetchMock.mockResolvedValueOnce(jsonResponse({ data: customer }));
        expect((await api.print("customer-1", null)).kind).toBe(
            "invalid_response",
        );
    });
});
