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

import type { OtpFields } from "./interface";
import { isSimpleMessageData, type SimpleMessageData } from "@/lib/interface";

export function isOtpFields(data: unknown): data is OtpFields {
    return (
        typeof data === "object" &&
        data !== null &&
        "otp" in data &&
        (typeof data.otp === "string" || data.otp === null)
    );
}

export function isOtpMessageResponse(
    data: unknown,
): data is { data: SimpleMessageData } {
    return (
        typeof data === "object" &&
        data !== null &&
        "data" in data &&
        isSimpleMessageData(data.data)
    );
}

export function isUnimplementedResponse(_data: unknown): _data is never {
    return false;
}
