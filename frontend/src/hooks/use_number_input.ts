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

import { useCallback, useRef, useState } from "react";
import { formatNumber, isValidNumber, parseNumber } from "@/lib/utils";

interface UseNumberInputOptions {
  initialValue?: string | number;
  showThousandSeparator?: boolean;
  decimalPlaces?: number;
  allowEmpty?: boolean;
  onValueChange?: (rawValue: string, numericValue: number) => void;
}

export function useNumberInput(options: UseNumberInputOptions = {}) {
  const {
    initialValue = "",
    showThousandSeparator = true,
    decimalPlaces,
    allowEmpty = true,
    onValueChange,
  } = options;

  const [displayValue, setDisplayValue] = useState(() =>
    formatNumber(initialValue, {
      showThousandSeparator,
      decimalPlaces,
      allowEmpty,
    }),
  );
  const [rawValue, setRawValue] = useState(initialValue?.toString() || "");

  // Track if we're in the middle of editing to avoid aggressive formatting
  const isEditingRef = useRef(false);

  const handleInputChange = useCallback(
    (inputValue: string) => {
      isEditingRef.current = true;

      // Store the raw input for validation and parsing
      setRawValue(inputValue);

      // For decimal numbers, be less aggressive with formatting while typing
      let formatted: string;

      if (
        decimalPlaces !== undefined ||
        inputValue.includes(",") ||
        inputValue.includes(".")
      ) {
        // When dealing with decimals, only format if the input looks complete
        // or if we're not in the middle of typing a decimal
        const hasIncompleteDecimal =
          inputValue.endsWith(",") || inputValue.endsWith(".");

        if (hasIncompleteDecimal) {
          // Don't format incomplete decimals, just normalize separators
          formatted = inputValue.replace(/\./g, ",");
        } else {
          // Format normally for complete numbers
          formatted = formatNumber(inputValue, {
            showThousandSeparator,
            decimalPlaces,
            allowEmpty,
          });
        }
      } else {
        // For integers, format normally
        formatted = formatNumber(inputValue, {
          showThousandSeparator,
          decimalPlaces,
          allowEmpty,
        });
      }

      setDisplayValue(formatted);

      if (onValueChange && isValidNumber(inputValue)) {
        const numericValue = parseNumber(inputValue);
        onValueChange(inputValue, numericValue);
      }

      // Reset editing flag after a short delay
      setTimeout(() => {
        isEditingRef.current = false;
      }, 100);
    },
    [showThousandSeparator, decimalPlaces, allowEmpty, onValueChange],
  );

  // Enhanced version that handles cursor position
  const handleInputChangeWithCursor = useCallback(
    (inputValue: string, inputElement?: HTMLInputElement) => {
      const cursorPosition = inputElement?.selectionStart || 0;
      const oldValue = displayValue;

      handleInputChange(inputValue);

      // Restore cursor position after formatting if the input element is provided
      if (inputElement && showThousandSeparator) {
        setTimeout(() => {
          const newValue = formatNumber(inputValue, {
            showThousandSeparator,
            decimalPlaces,
            allowEmpty,
          });

          // Calculate new cursor position based on the formatting changes
          let newCursorPosition = cursorPosition;

          // Count spaces before cursor in old and new values
          const oldSpacesBefore = (
            oldValue.substring(0, cursorPosition).match(/\s/g) || []
          ).length;
          const newSpacesBefore = (
            newValue.substring(0, cursorPosition).match(/\s/g) || []
          ).length;

          // Adjust cursor position based on space difference
          newCursorPosition += newSpacesBefore - oldSpacesBefore;

          // Ensure cursor position is within bounds
          newCursorPosition = Math.max(
            0,
            Math.min(newCursorPosition, newValue.length),
          );

          if (inputElement.selectionStart !== newCursorPosition) {
            inputElement.setSelectionRange(
              newCursorPosition,
              newCursorPosition,
            );
          }
        }, 0);
      }
    },
    [
      displayValue,
      handleInputChange,
      showThousandSeparator,
      decimalPlaces,
      allowEmpty,
    ],
  );

  const setValue = useCallback(
    (value: string | number) => {
      const stringValue = value?.toString() || "";
      setRawValue(stringValue);

      const formatted = formatNumber(stringValue, {
        showThousandSeparator,
        decimalPlaces,
        allowEmpty,
      });

      setDisplayValue(formatted);

      if (onValueChange) {
        const numericValue = parseNumber(stringValue);
        onValueChange(stringValue, numericValue);
      }
    },
    [showThousandSeparator, decimalPlaces, allowEmpty, onValueChange],
  );

  const getNumericValue = useCallback(() => {
    return parseNumber(rawValue);
  }, [rawValue]);

  const getRawValue = useCallback(() => {
    return rawValue;
  }, [rawValue]);

  const isValid = useCallback(() => {
    return isValidNumber(displayValue);
  }, [displayValue]);

  return {
    displayValue,
    handleInputChange,
    handleInputChangeWithCursor,
    setValue,
    getNumericValue,
    getRawValue,
    isValid,
  };
}
