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

import { describe, expect, it } from "vitest";
import { taskResponseSchema, taskSchema } from "./schemas";

const task = {
    id: "task",
    worksheet_id: "worksheet",
    service_id: "service",
    currency_code: "EUR",
    quantity: null,
    price: "10",
    tax_id: "tax",
    created_by_id: "user",
    status: "open",
    priority: null,
    due_date: null,
    created_at: "now",
    updated_at: "now",
    deleted_at: null,
    description: null,
};

describe("task schemas", () => {
    it("validates required fields while accepting nullable and extra fields", () => {
        expect(taskSchema.safeParse({ ...task, extra: true }).success).toBe(
            true,
        );
        expect(taskSchema.safeParse({ ...task, quantity: 1 }).success).toBe(
            false,
        );
        expect(
            taskSchema.safeParse({ ...task, description: undefined }).success,
        ).toBe(false);
    });

    it("validates success envelope metadata and rejects error envelopes", () => {
        expect(
            taskResponseSchema.safeParse({ data: task, meta: null }).success,
        ).toBe(true);
        expect(taskResponseSchema.safeParse({ data: task }).success).toBe(true);
        expect(
            taskResponseSchema.safeParse({ data: task, meta: { extra: true } })
                .success,
        ).toBe(false);
        expect(
            taskResponseSchema.safeParse({ data: task, error: undefined })
                .success,
        ).toBe(false);
        expect(taskResponseSchema.safeParse({}).success).toBe(false);
    });
});
