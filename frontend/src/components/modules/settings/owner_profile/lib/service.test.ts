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
    isOwnerProfileErrors,
    isOwnerProfileFull,
    isOwnerProfileFullResponse,
} from "./guards";
import { get_full, update } from "./service";
import type { OwnerProfileUserInput } from "./interface";

const address = {
    id: "address",
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
const profile = {
    id: "owner",
    name: "Owner",
    contact_name: null,
    email: "owner@example.com",
    website: null,
    document_prefix: null,
    phone_number: null,
    owner_profile_type: "company",
    created_at: "now",
    updated_at: "now",
    billing_address: address,
    mailing_address: null,
};
const addressErrors = Object.fromEntries(
    Object.keys(address).map((key) => [key, null]),
);
const fields = {
    name: null,
    contact_name: null,
    email: "Invalid",
    website: null,
    document_prefix: null,
    phone_number: null,
    owner_profile_type: null,
    billing_address: addressErrors,
    mailing_address: addressErrors,
};
const input: OwnerProfileUserInput = {
    id: "owner",
    name: "Owner",
    contactName: "",
    email: "owner@example.com",
    website: "",
    documentPrefix: "",
    phoneNumber: "",
    ownerProfileType: undefined,
    billingAddress: undefined,
    mailingAddress: null,
};
const json = (body: unknown, status = 200) =>
    new Response(JSON.stringify(body), {
        status,
        headers: { "Content-Type": "application/json" },
    });

describe("owner profile schemas", () => {
    it("validates nested addresses and success envelopes", () => {
        expect(isOwnerProfileFull({ ...profile, extra: true })).toBe(true);
        expect(
            isOwnerProfileFull({ ...profile, billing_address: "address" }),
        ).toBe(false);
        expect(
            isOwnerProfileFull({
                ...profile,
                billing_address: { ...address, id: null },
            }),
        ).toBe(false);
        expect(isOwnerProfileFullResponse({ data: profile })).toBe(true);
        expect(isOwnerProfileFullResponse({})).toBe(false);
        expect(isOwnerProfileFullResponse({ data: null })).toBe(false);
        expect(
            isOwnerProfileFullResponse({ error: { message: "Invalid" } }),
        ).toBe(false);
    });

    it("validates nested error fields", () => {
        expect(isOwnerProfileErrors(fields)).toBe(true);
        expect(isOwnerProfileErrors({ ...fields, billing_address: {} })).toBe(
            false,
        );
        expect(isOwnerProfileErrors({ ...fields, email: 123 })).toBe(false);
    });
});

describe("owner profile API", () => {
    const fetchMock = vi.fn<typeof fetch>();
    beforeEach(() => {
        fetchMock.mockReset();
        vi.stubGlobal("fetch", fetchMock);
    });
    afterEach(() => vi.unstubAllGlobals());

    it("maps update input and validates 422 fields", async () => {
        fetchMock.mockResolvedValueOnce(
            json({ error: { message: "Invalid", fields } }, 422),
        );
        expect(await update(input, "token")).toEqual({
            kind: "http_error",
            status: 422,
            body: { error: { message: "Invalid", fields } },
        });
        const [url, options] = fetchMock.mock.calls[0];
        expect(url).toBe("/api/owner_profile/update");
        expect(options?.method).toBe("PUT");
        expect(JSON.parse(options?.body as string)).toMatchObject({
            owner_profile_type: null,
            billing_address: null,
            mailing_address: null,
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
        fetchMock.mockResolvedValueOnce(json({ data: profile }));
        expect(await get_full(null)).toMatchObject({
            kind: "success",
            data: { data: profile },
        });
        fetchMock.mockResolvedValueOnce(json({ data: null }));
        expect(await get_full(null)).toMatchObject({
            kind: "invalid_response",
        });
        fetchMock.mockResolvedValueOnce(new Response(null, { status: 204 }));
        expect(await get_full(null)).toMatchObject({
            kind: "invalid_response",
        });
    });
});
