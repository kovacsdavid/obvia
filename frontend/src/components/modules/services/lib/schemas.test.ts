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

import { afterEach, describe, expect, it, vi } from "vitest";
import * as servicesApi from "./service";
import { isService, isServiceResolved } from "./guards";

const service = {
    id: "service-1",
    name: "Service",
    description: null,
    default_price: null,
    default_tax_id: null,
    currency_code: null,
    status: "active",
    created_by_id: "user-1",
    created_at: "2026-01-01",
    updated_at: "2026-01-01",
    deleted_at: null,
    unit_of_measure_id: "unit-1",
    extra: "preserved",
};
const resolved = {
    ...service,
    default_tax: null,
    created_by: "User",
    unit_of_measure: "hour",
};
const input = {
    id: null,
    name: "Service",
    description: null,
    default_price: null,
    default_tax_id: null,
    currency_code: null,
    status: "active",
    unit_of_measure_id: "unit-1",
    new_unit_of_measure: "",
};
const fields = {
    id: null,
    name: "Required",
    description: null,
    default_price: null,
    default_tax_id: null,
    currency_code: null,
    status: null,
    unit_of_measure_id: null,
    new_unit_of_measure: null,
};

function respond(body: unknown, status = 200) {
    vi.stubGlobal(
        "fetch",
        vi.fn().mockResolvedValue(
            new Response(JSON.stringify(body), {
                status,
                headers: { "Content-Type": "application/json" },
            }),
        ),
    );
}

afterEach(() => vi.unstubAllGlobals());

describe("service schemas in the API", () => {
    it("validates payloads and preserves unknown fields", async () => {
        expect(isService(service)).toBe(true);
        expect(isService({ ...service, default_price: 4 })).toBe(false);
        expect(isServiceResolved(resolved)).toBe(true);
        for (const unit_of_measure of [null, undefined, 1, true, {}, []]) {
            expect(isServiceResolved({ ...resolved, unit_of_measure })).toBe(
                false,
            );
        }
        const { unit_of_measure: _unitOfMeasure, ...missingUnit } = resolved;
        expect(isServiceResolved(missingUnit)).toBe(false);
        const body = { meta: null, data: service, extra: true };
        respond(body);
        expect(await servicesApi.get(service.id, null)).toEqual({
            kind: "success",
            status: 200,
            data: body,
        });
    });

    it("validates success envelopes and delete messages", async () => {
        for (const body of [
            {},
            { data: null },
            { error: { message: "error" } },
            { data: service, error: { message: "error" } },
        ]) {
            respond(body);
            expect(await servicesApi.get(service.id, null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        }
        respond({ data: { message: "Deleted" } });
        expect(await servicesApi.deleteItem(service.id, null)).toMatchObject({
            kind: "success",
        });
        respond({ data: { message: 1 } });
        expect(await servicesApi.deleteItem(service.id, null)).toMatchObject({
            kind: "invalid_response",
        });
    });

    it("accepts successful create, update and resolved responses", async () => {
        for (const operation of ["create", "update"] as const) {
            respond({ data: service });
            expect(await servicesApi[operation](input, null)).toMatchObject({
                kind: "success",
                data: { data: service },
            });
        }
        respond({ data: resolved });
        expect(await servicesApi.get_resolved(service.id, null)).toMatchObject({
            kind: "success",
            data: { data: resolved },
        });
        for (const unit_of_measure of [null, 1, true, {}, []]) {
            respond({ data: { ...resolved, unit_of_measure } });
            expect(await servicesApi.get_resolved(service.id, null)).toEqual({
                kind: "invalid_response",
                status: 200,
            });
        }
        vi.stubGlobal(
            "fetch",
            vi.fn().mockResolvedValue(new Response(null, { status: 204 })),
        );
        expect(await servicesApi.get(service.id, null)).toMatchObject({
            kind: "invalid_response",
            status: 204,
        });
    });

    it("requires valid pagination metadata and resolved records", async () => {
        const meta = { page: 1, limit: 25, total: 1, extra: true };
        const body = { meta, data: [resolved], extra: true };
        respond(body);
        expect(await servicesApi.list(null, null)).toEqual({
            kind: "success",
            status: 200,
            data: body,
        });
        for (const invalid of [
            { data: [] },
            { meta: null, data: [] },
            { meta: { ...meta, total: "1" }, data: [] },
            { meta, data: [{ ...resolved, default_tax: 1 }] },
            { meta, data: [{ ...resolved, unit_of_measure: 1 }] },
        ]) {
            respond(invalid);
            expect(await servicesApi.list(null, null)).toMatchObject({
                kind: "invalid_response",
            });
        }
        respond({ meta, data: [] });
        expect(await servicesApi.list(null, null)).toMatchObject({
            kind: "success",
        });
    });

    it.each(["create", "update"] as const)(
        "handles %s form errors",
        async (operation) => {
            for (const error of [
                { message: "Invalid", fields },
                { message: "Invalid" },
            ]) {
                const body = { error };
                respond(body, 422);
                expect(await servicesApi[operation](input, null)).toEqual({
                    kind: "http_error",
                    status: 422,
                    body,
                });
            }
            for (const invalid of [{}, { ...fields, status: 1 }]) {
                respond(
                    { error: { message: "Invalid", fields: invalid } },
                    422,
                );
                expect(await servicesApi[operation](input, null)).toEqual({
                    kind: "http_error",
                    status: 422,
                    body: undefined,
                });
            }
        },
    );
});
