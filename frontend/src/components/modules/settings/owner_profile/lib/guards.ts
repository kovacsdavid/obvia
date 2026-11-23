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

import { schemaGuard } from "@/lib/api/schemas";
import {
    ownerProfileErrorsSchema,
    ownerProfileFullResponseSchema,
    ownerProfileFullSchema,
    ownerProfileSchema,
    updateOwnerProfileResponseSchema,
} from "./schemas";

export const isOwnerProfile = schemaGuard(ownerProfileSchema);
export const isOwnerProfileFull = schemaGuard(ownerProfileFullSchema);
export const isOwnerProfileErrors = schemaGuard(ownerProfileErrorsSchema);
export const isUpdateOwnerProfileResponse = schemaGuard(
    updateOwnerProfileResponseSchema,
);
export const isOwnerProfileFullResponse = schemaGuard(
    ownerProfileFullResponseSchema,
);
