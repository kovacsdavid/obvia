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

import { act, renderHook } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { useNumberInput } from "./use_number_input";

describe("useNumberInput", () => {
  it("should initialize with default values", () => {
    const { result } = renderHook(() => useNumberInput());
    expect(result.current.displayValue).toBe("");
    expect(result.current.getRawValue()).toBe("");
    expect(result.current.getNumericValue()).toBeNaN();
    expect(result.current.isValid()).toBe(true);
  });

  it("should initialize with provided initial value", () => {
    const { result } = renderHook(() =>
      useNumberInput({ initialValue: "1234.56" }),
    );
    expect(result.current.displayValue).toBe("1 234,56");
    expect(result.current.getRawValue()).toBe("1234.56");
    expect(result.current.getNumericValue()).toBe(1234.56);
  });

  it("should handle input changes", () => {
    const onValueChange = vi.fn();
    const { result } = renderHook(() => useNumberInput({ onValueChange }));

    act(() => {
      result.current.handleInputChange("1234,56");
    });

    expect(result.current.displayValue).toBe("1 234,56");
    expect(result.current.getRawValue()).toBe("1234,56");
    expect(result.current.getNumericValue()).toBe(1234.56);
    expect(onValueChange).toHaveBeenCalledWith("1234,56", 1234.56);
  });

  it("should format numbers according to options", () => {
    const { result } = renderHook(() =>
      useNumberInput({
        showThousandSeparator: true,
        decimalPlaces: 2,
      }),
    );

    act(() => {
      result.current.handleInputChange("1234.5678");
    });

    expect(result.current.displayValue).toBe("1 234,57");
  });

  it("should handle invalid input", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.handleInputChange("abc");
    });

    expect(result.current.displayValue).toBe("abc");
    expect(result.current.isValid()).toBe(false);
    expect(result.current.getNumericValue()).toBeNaN();
  });

  it("should set value programmatically", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.setValue("1234.56");
    });

    expect(result.current.displayValue).toBe("1 234,56");
    expect(result.current.getRawValue()).toBe("1234.56");
    expect(result.current.getNumericValue()).toBe(1234.56);
  });

  it("should handle empty values when allowEmpty is true", () => {
    const { result } = renderHook(() => useNumberInput({ allowEmpty: true }));

    act(() => {
      result.current.handleInputChange("");
    });

    expect(result.current.displayValue).toBe("");
    expect(result.current.getRawValue()).toBe("");
    expect(result.current.getNumericValue()).toBeNaN();
    expect(result.current.isValid()).toBe(true);
  });

  it("should handle empty values when allowEmpty is false", () => {
    const { result } = renderHook(() => useNumberInput({ allowEmpty: false }));

    act(() => {
      result.current.handleInputChange("");
    });

    expect(result.current.displayValue).toBe("0");
  });

  it("should handle negative numbers", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.handleInputChange("-1234.56");
    });

    expect(result.current.displayValue).toBe("-1 234,56");
    expect(result.current.getRawValue()).toBe("-1234.56");
    expect(result.current.getNumericValue()).toBe(-1234.56);
    expect(result.current.isValid()).toBe(true);
  });

  it("should respect decimal places option", () => {
    const { result } = renderHook(() => useNumberInput({ decimalPlaces: 3 }));

    act(() => {
      result.current.handleInputChange("1234.5678");
    });

    expect(result.current.displayValue).toBe("1 234,568");
  });

  it("should handle numbers without thousand separator", () => {
    const { result } = renderHook(() =>
      useNumberInput({ showThousandSeparator: false }),
    );

    act(() => {
      result.current.handleInputChange("1234.56");
    });

    expect(result.current.displayValue).toBe("1234,56");
  });

  it("should handle multiple input changes", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.handleInputChange("1234.56");
    });

    expect(result.current.displayValue).toBe("1 234,56");

    act(() => {
      result.current.handleInputChange("5678.90");
    });

    expect(result.current.displayValue).toBe("5 678,9");
  });

  it("should preserve incomplete decimal inputs", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.handleInputChange("1234,");
    });

    expect(result.current.displayValue).toBe("1234,");

    act(() => {
      result.current.handleInputChange("1234,5");
    });

    expect(result.current.displayValue).toBe("1 234,5");
  });

  it("should handle cursor position maintenance", () => {
    const mockInput = document.createElement("input");
    const { result } = renderHook(() => useNumberInput());

    // Mock selection range methods
    mockInput.setSelectionRange = vi.fn();
    mockInput.selectionStart = 2;

    act(() => {
      result.current.handleInputChangeWithCursor("1234", mockInput);
    });

    // Should attempt to maintain cursor position after formatting
    expect(mockInput.setSelectionRange).toHaveBeenCalled();
  });

  it("should handle cursor position with thousand separators", () => {
    const mockInput = document.createElement("input");
    const { result } = renderHook(() =>
      useNumberInput({ showThousandSeparator: true }),
    );

    mockInput.setSelectionRange = vi.fn();
    mockInput.selectionStart = 4;

    act(() => {
      result.current.handleInputChangeWithCursor("12345", mockInput);
    });

    // Should adjust cursor position for added thousand separator
    expect(mockInput.setSelectionRange).toHaveBeenCalled();
  });

  it("should handle setValue with number type", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.setValue(1234.56);
    });

    expect(result.current.displayValue).toBe("1 234,56");
    expect(result.current.getRawValue()).toBe("1234.56");
    expect(result.current.getNumericValue()).toBe(1234.56);
  });

  it("should handle large numbers with thousand separators", () => {
    const { result } = renderHook(() => useNumberInput());

    act(() => {
      result.current.handleInputChange("1234567890.12");
    });

    expect(result.current.displayValue).toBe("1 234 567 890,12");
    expect(result.current.getRawValue()).toBe("1234567890.12");
    expect(result.current.getNumericValue()).toBe(1234567890.12);
  });

  it("should handle very small decimal numbers", () => {
    const { result } = renderHook(() => useNumberInput({ decimalPlaces: 8 }));

    act(() => {
      result.current.handleInputChange("0.00000001");
    });

    expect(result.current.displayValue).toBe("0,00000001");
    expect(result.current.getRawValue()).toBe("0.00000001");
    expect(result.current.getNumericValue()).toBe(0.00000001);
  });
});
