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

import type { z } from "zod";
import type { Currency as CurrencyInterface } from "@/components/modules/currencies/lib/interface";
import type { CustomerFull as CustomerInterface } from "@/components/modules/customers/lib/interface";
import type {
    createHandoverAndAcceptanceResponseSchema,
    deleteHandoverAndAcceptanceResponseSchema,
    handoverAndAcceptanceErrorSchema,
    handoverAndAcceptanceFullListSchema,
    handoverAndAcceptanceFullResponseSchema,
    handoverAndAcceptanceFullSchema,
    handoverAndAcceptanceResponseSchema,
    handoverAndAcceptanceSchema,
    handoverAndAcceptanceUserInputSchema,
    itemIdentifierTypeSchema,
    itemMakeSchema,
    itemModelFullSchema,
    paginatedHandoverAndAcceptanceFullListResponseSchema,
    updateHandoverAndAcceptanceResponseSchema,
} from "./schemas";

export type { SelectOptionListResponse } from "@/lib/interface.ts";

export type HandoverAndAcceptanceUserInput = z.infer<
    typeof handoverAndAcceptanceUserInputSchema
>;
export type HandoverAndAcceptance = z.infer<typeof handoverAndAcceptanceSchema>;
export type HandoverAndAcceptanceError = z.infer<
    typeof handoverAndAcceptanceErrorSchema
>;
export type ItemIdentifierType = z.infer<typeof itemIdentifierTypeSchema>;
export type ItemMake = z.infer<typeof itemMakeSchema>;
export type ItemModelFull = z.infer<typeof itemModelFullSchema>;
export type HandoverAndAcceptanceFull<
    Customer = CustomerInterface,
    Currency = CurrencyInterface,
    User = z.infer<typeof handoverAndAcceptanceFullSchema>["accepted_by"],
> = Pick<
    z.infer<typeof handoverAndAcceptanceFullSchema>,
    Exclude<
        keyof typeof handoverAndAcceptanceFullSchema.shape,
        "customer" | "currency_code" | "accepted_by"
    >
> & {
    customer: Customer;
    currency_code: Currency | null;
    accepted_by: User;
};
export type CreateHandoverAndAcceptanceResponse = z.infer<
    typeof createHandoverAndAcceptanceResponseSchema
>;
export type UpdateHandoverAndAcceptanceResponse = z.infer<
    typeof updateHandoverAndAcceptanceResponseSchema
>;
export type DeleteHandoverAndAcceptanceResponse = z.infer<
    typeof deleteHandoverAndAcceptanceResponseSchema
>;
export type HandoverAndAcceptanceResponse = z.infer<
    typeof handoverAndAcceptanceResponseSchema
>;
export type HandoverAndAcceptanceFullResponse = z.infer<
    typeof handoverAndAcceptanceFullResponseSchema
>;
export type HandoverAndAcceptanceFullList = z.infer<
    typeof handoverAndAcceptanceFullListSchema
>;
export type PaginatedHandoverAndAcceptanceFullListResponse = z.infer<
    typeof paginatedHandoverAndAcceptanceFullListResponseSchema
>;
