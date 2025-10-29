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

export function formatDateToYMDHMS(dateString: string): string {
  try {
    const date = new Date(dateString);

    if (isNaN(date.getTime())) {
      return '';
    }

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    const hours = String(date.getHours()).padStart(2, '0');
    const minutes = String(date.getMinutes()).padStart(2, '0');
    const seconds = String(date.getSeconds()).padStart(2, '0');

    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  } catch (error) {
    console.log(error)
    return '';
  }
}

export function formatDateToYMD(dateString: string): string {
  try {
    const date = new Date(dateString);

    if (isNaN(date.getTime())) {
      return '';
    }

    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');

    return `${year}-${month}-${day}`;
  } catch (error) {
    console.log(error)
    return '';
  }
}

/**
 * Formats a number with thousand separators and decimal formatting
 * @param value - The number or string to format
 * @param options - Formatting options
 * @returns Formatted string with number conventions
 */
export function formatNumber(
  value: string | number,
  options: {
    showThousandSeparator?: boolean;
    decimalPlaces?: number;
    allowEmpty?: boolean;
    preserveIncomplete?: boolean; // New option to preserve incomplete inputs
  } = {}
): string {
  const {
    showThousandSeparator = true,
    decimalPlaces,
    allowEmpty = true,
    preserveIncomplete = false
  } = options;

  // Handle empty values
  if (value === "" || value == null) {
    return allowEmpty ? "" : "0";
  }

  // Convert to string and clean up
  const stringValue = value.toString().trim();

  if (stringValue === "") {
    return allowEmpty ? "" : "0";
  }

  // If preserveIncomplete is true and value ends with decimal separator, don't format
  if (preserveIncomplete && (stringValue.endsWith(',') || stringValue.endsWith('.'))) {
    return stringValue.replace(/\./g, ',');
  }

  // Remove existing separators and normalize decimal separator
  const cleanValue = stringValue
    .replace(/\s/g, '') // Remove spaces (thousand separators)
    .replace(/,/g, '.'); // Convert comma to dot for parsing

  // Parse as number
  const numericValue = parseFloat(cleanValue);

  if (isNaN(numericValue)) {
    return stringValue; // Return original if not a valid number
  }

  // Format the number
  let formatted = decimalPlaces !== undefined
    ? numericValue.toFixed(decimalPlaces)
    : numericValue.toString();

  if (showThousandSeparator) {
    // Split integer and decimal parts
    const [integerPart, decimalPart] = formatted.split('.');

    // Add thousand separators (spaces) to integer part
    const formattedInteger = integerPart.replace(/\B(?=(\d{3})+(?!\d))/g, ' ');

    // Combine with decimal part using comma as decimal separator
    formatted = decimalPart
      ? `${formattedInteger},${decimalPart}`
      : formattedInteger;
  } else {
    // Just replace dot with comma for decimal separator
    formatted = formatted.replace('.', ',');
  }

  return formatted;
}

/**
 * Parses a formatted number string to a standard number
 * @param value - The formatted string
 * @returns Parsed number or NaN if invalid
 */
export function parseNumber(value: string): number {
  if (!value || value.trim() === "") {
    return NaN;
  }

  // Remove spaces (thousand separators) and convert comma to dot
  const normalized = value
    .replace(/\s/g, '')
    .replace(/,/g, '.');

  return parseFloat(normalized);
}

/**
 * Validates if a string is a valid number format
 * @param value - The string to validate
 * @returns True if valid  number format
 */
export function isValidNumber(value: string): boolean {
  if (!value || value.trim() === "") {
    return true; // Allow empty values
  }

  // Number pattern: optional minus, digits with spaces as thousand separators, optional comma and decimals
  const numberPattern = /^-?(\d{1,3}(\s\d{3})*|\d+)(,\d+)?$/;
  return numberPattern.test(value.trim());
}
