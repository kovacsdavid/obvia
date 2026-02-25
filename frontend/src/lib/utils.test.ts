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

import { describe, expect, it } from "vitest";
import {
  formatNumber,
  isValidNumber,
  parseNumber,
  query_encoder,
  query_parser,
} from "@/lib/utils.ts";
import { extract_field } from "./get_query";

describe("query_parser", () => {
  it("should parse valid query", () => {
    const input =
      "ordering%3Atest-asc%20paging%3A1-25%20filtering%3Atest-%7Cwarehouse%201%7C";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: "test",
        order: "asc",
      },
      paging: {
        page: 1,
        limit: 25,
      },
      filtering: {
        field: "test",
        value: "warehouse 1",
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query different data", () => {
    const input =
      "ordering%3Aname-desc%20paging%3A3-30%20filtering%3Atype-%7Csome%20type%7C";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query different order", () => {
    const input =
      "filtering%3Atype-%7Csome%20type%7C%20paging%3A3-30%20ordering%3Aname-desc";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query with trailing spaces", () => {
    const input =
      "%20%20%20%20%20filtering%3Atype-%7Csome%20type%7C%20paging%3A3-30%20ordering%3Aname-desc%20%20%20%20";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query partial 1", () => {
    const input = "ordering%3Aname-desc";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query partial 2", () => {
    const input = "paging%3A3-30";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid query partial 3", () => {
    const input = "filtering%3Atype-%7Csome%20type%7C";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse valid empty query", () => {
    const input = "";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    expect(result).toEqual(expected_result);
  });
  it("should parse empty spaces", () => {
    const input = "   ";
    const result = query_parser(input);
    const expected_result = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    expect(result).toEqual(expected_result);
  });
});

describe("query_encoder", () => {
  it("should convert to valid query", () => {
    const input = {
      ordering: {
        order_by: "test",
        order: "asc",
      },
      paging: {
        page: 1,
        limit: 25,
      },
      filtering: {
        field: "test",
        value: "warehouse 1",
      },
    };
    const result = query_encoder(input);
    const expected_result =
      "ordering%3Atest-asc%20paging%3A1-25%20filtering%3Atest-%7Cwarehouse%201%7C";

    expect(result).toEqual(expected_result);
  });
  it("should convert to valid query different data", () => {
    const input = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    const result = query_encoder(input);
    const expected_result =
      "ordering%3Aname-desc%20paging%3A3-30%20filtering%3Atype-%7Csome%20type%7C";

    expect(result).toEqual(expected_result);
  });
  it("should convert to empty string", () => {
    const input = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    const result = query_encoder(input);
    const expected_result = "";

    expect(result).toEqual(expected_result);
  });
  it("should convert to partial 1", () => {
    const input = {
      ordering: {
        order_by: "name",
        order: "desc",
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    const result = query_encoder(input);
    const expected_result = "ordering%3Aname-desc";

    expect(result).toEqual(expected_result);
  });
  it("should convert to partial 2", () => {
    const input = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: 3,
        limit: 30,
      },
      filtering: {
        field: null,
        value: null,
      },
    };
    const result = query_encoder(input);
    const expected_result = "paging%3A3-30";

    expect(result).toEqual(expected_result);
  });
  it("should convert to partial 3", () => {
    const input = {
      ordering: {
        order_by: null,
        order: null,
      },
      paging: {
        page: null,
        limit: null,
      },
      filtering: {
        field: "type",
        value: "some type",
      },
    };
    const result = query_encoder(input);
    const expected_result = "filtering%3Atype-%7Csome%20type%7C";

    expect(result).toEqual(expected_result);
  });
});

describe("extract_field", () => {
  it("should extract ordering", () => {
    const input = "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
    const result = extract_field(input, "ordering:");
    const expected_result = "ordering:test-asc";
    expect(result).toEqual(expected_result);
  });
  it("should extract paging", () => {
    const input = "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
    const result = extract_field(input, "paging:");
    const expected_result = "paging:1-25";
    expect(result).toEqual(expected_result);
  });
  it("should extract filtering", () => {
    const input = "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
    const result = extract_field(input, "filtering:");
    const expected_result = "filtering:test-|warehouse 1|";
    expect(result).toEqual(expected_result);
  });
});

describe("formatNumber", () => {
  describe("Empty values", () => {
    it("should return empty string for empty input when allowEmpty is true", () => {
      expect(formatNumber("", { allowEmpty: true })).toBe("");
      expect(formatNumber(null as any, { allowEmpty: true })).toBe("");
    });

    it("should return '0' for empty input when allowEmpty is false", () => {
      expect(formatNumber("", { allowEmpty: false })).toBe("0");
      expect(formatNumber(null as any, { allowEmpty: false })).toBe("0");
    });
  });

  describe("Basic formatting", () => {
    it("should format integer numbers correctly", () => {
      expect(formatNumber(1234)).toBe("1 234");
      expect(formatNumber("1234")).toBe("1 234");
    });

    it("should handle negative numbers", () => {
      expect(formatNumber(-1234)).toBe("-1 234");
      expect(formatNumber("-1234")).toBe("-1 234");
    });
  });

  describe("Thousand separators", () => {
    it("should add thousand separators when enabled", () => {
      expect(formatNumber(1234567, { showThousandSeparator: true })).toBe(
        "1 234 567",
      );
    });

    it("should not add thousand separators when disabled", () => {
      expect(formatNumber(1234567, { showThousandSeparator: false })).toBe(
        "1234567",
      );
    });
  });

  describe("Decimal places", () => {
    it("should format decimal numbers with specified decimal places", () => {
      expect(formatNumber(1234.5678, { decimalPlaces: 2 })).toBe("1 234,57");
      expect(formatNumber(1234.5678, { decimalPlaces: 0 })).toBe("1 235");
    });

    it("should add trailing zeros when needed", () => {
      expect(formatNumber(1234.5, { decimalPlaces: 2 })).toBe("1 234,50");
    });
  });

  describe("Invalid inputs", () => {
    it("should return original string for non-numeric input", () => {
      expect(formatNumber("abc")).toBe("abc");
    });
  });

  describe("Edge cases", () => {
    it("should handle very large numbers", () => {
      expect(formatNumber(1234567890.12)).toBe("1 234 567 890,12");
    });

    it("should handle very small decimals", () => {
      expect(formatNumber(0.000001, { decimalPlaces: 6 })).toBe("0,000001");
    });

    it("should handle numbers with existing formatting", () => {
      expect(formatNumber("1 234,56")).toBe("1 234,56");
    });
  });
});

describe("parseNumber", () => {
  describe("Empty values", () => {
    it("should return NaN for empty input", () => {
      expect(isNaN(parseNumber(""))).toBe(true);
      expect(isNaN(parseNumber("   "))).toBe(true);
    });
  });

  describe("Basic parsing", () => {
    it("should parse integer numbers correctly", () => {
      expect(parseNumber("1234")).toBe(1234);
      expect(parseNumber("1 234")).toBe(1234);
    });

    it("should handle negative numbers", () => {
      expect(parseNumber("-1234")).toBe(-1234);
      expect(parseNumber("-1 234")).toBe(-1234);
    });

    it("should parse decimal numbers correctly", () => {
      expect(parseNumber("1234,56")).toBe(1234.56);
      expect(parseNumber("1 234,56")).toBe(1234.56);
    });
  });

  describe("Edge cases", () => {
    it("should handle multiple spaces", () => {
      expect(parseNumber("1   234")).toBe(1234);
    });

    it("should handle very large numbers", () => {
      expect(parseNumber("1 234 567 890,12")).toBe(1234567890.12);
    });

    it("should handle very small decimals", () => {
      expect(parseNumber("0,000001")).toBe(0.000001);
    });
  });
});

describe("isValidNumber", () => {
  describe("Empty values", () => {
    it("should return true for empty input", () => {
      expect(isValidNumber("")).toBe(true);
      expect(isValidNumber("   ")).toBe(true);
    });
  });

  describe("Valid formats", () => {
    it("should validate integer numbers", () => {
      expect(isValidNumber("1234")).toBe(true);
      expect(isValidNumber("1 234")).toBe(true);
      expect(isValidNumber("1 234 567")).toBe(true);
    });

    it("should validate negative numbers", () => {
      expect(isValidNumber("-1234")).toBe(true);
      expect(isValidNumber("-1 234")).toBe(true);
    });

    it("should validate decimal numbers", () => {
      expect(isValidNumber("1234,56")).toBe(true);
      expect(isValidNumber("1 234,56")).toBe(true);
    });
  });

  describe("Invalid formats", () => {
    it("should reject invalid separators", () => {
      expect(isValidNumber("1.234")).toBe(false);
      expect(isValidNumber("1,234,56")).toBe(false);
    });

    it("should reject non-numeric characters", () => {
      expect(isValidNumber("abc")).toBe(false);
      expect(isValidNumber("123abc")).toBe(false);
    });

    it("should reject multiple decimal points", () => {
      expect(isValidNumber("1,23,45")).toBe(false);
    });

    it("should reject invalid thousand separator positions", () => {
      expect(isValidNumber("12 34")).toBe(false);
      expect(isValidNumber("1 23 456")).toBe(false);
    });
  });

  describe("Edge cases", () => {
    it("should validate numbers with many decimal places", () => {
      expect(isValidNumber("0,000001")).toBe(true);
    });

    it("should validate very large numbers", () => {
      expect(isValidNumber("1 234 567 890,12")).toBe(true);
    });
  });
});
