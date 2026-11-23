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

import { z } from "zod";

export const paginatorMetaSchema = z.looseObject({
    page: z.number(),
    limit: z.number(),
    total: z.number(),
});

export function successResponseSchema<T extends z.ZodType>(data: T) {
    return z
        .looseObject({
            meta: z.null().optional(),
            data,
            error: z.never().optional(),
        })
        .refine(
            (value) =>
                "data" in value &&
                value.data !== undefined &&
                !("error" in value),
        );
}

export function paginatedSuccessResponseSchema<T extends z.ZodType>(data: T) {
    return z
        .looseObject({
            meta: paginatorMetaSchema,
            data,
            error: z.never().optional(),
        })
        .refine(
            (value) =>
                "data" in value &&
                value.data !== undefined &&
                !("error" in value),
        );
}

export const selectOptionSchema = z.looseObject({
    value: z.string(),
    title: z.string(),
});

export const selectOptionListSchema = z.array(selectOptionSchema);

export const selectOptionListResponseSchema = successResponseSchema(
    selectOptionListSchema,
);

export const base64DataResponseSchema = z.looseObject({
    data: z.string(),
    error: z.never().optional(),
});

export function schemaGuard<T extends z.ZodType>(
    schema: T,
): (data: unknown) => data is z.input<T> {
    return (data: unknown): data is z.input<T> =>
        schema.safeParse(data).success;
}
