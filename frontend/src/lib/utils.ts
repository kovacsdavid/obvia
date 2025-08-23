/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import {type ClassValue, clsx} from "clsx"
import {twMerge} from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function query_parser(encodedStr: unknown): Record<string, string | number> {
  const result: Record<string, string | number> = {};
  if (typeof encodedStr === "string") {
    const decodedStr = decodeURIComponent(encodedStr);
    const pairs = decodedStr.split('|');

    pairs.forEach(pair => {
      const keyValue = pair.split(':');
      if (keyValue.length === 2) {
        const key = keyValue[0].trim();
        const value = keyValue[1].trim();

        if (key.length > 0 && value.length > 0) {
          result[key] = !isNaN(Number(value)) ? Number(value) : value;
        }
      }
    });
  }
  return result
}

export function query_encoder(params: Record<string, string | number>): string {
  const pairs = Object.entries(params).map(([key, valueRaw]) => {
    const value = valueRaw.toString().trim()
    return `${key}:${value}`;
  });
  const concatenated = pairs.join('|');
  return encodeURIComponent(concatenated);
}
