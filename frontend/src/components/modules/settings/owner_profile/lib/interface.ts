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
import type {
    ownerProfileErrorsSchema,
    ownerProfileFullResponseSchema,
    ownerProfileFullSchema,
    ownerProfileSchema,
    ownerProfileUserInputSchema,
    updateOwnerProfileResponseSchema,
} from "./schemas";

export type OwnerProfileUserInput = z.infer<typeof ownerProfileUserInputSchema>;
export type OwnerProfile = z.infer<typeof ownerProfileSchema>;
export type OwnerProfileErrors = z.infer<typeof ownerProfileErrorsSchema>;
export type OwnerProfileFull = z.infer<typeof ownerProfileFullSchema>;
export type UpdateOwnerProfileResponse = z.infer<
    typeof updateOwnerProfileResponseSchema
>;
export type OwnerProfileFullResponse = z.infer<
    typeof ownerProfileFullResponseSchema
>;
