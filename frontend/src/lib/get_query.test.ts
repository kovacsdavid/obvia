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

import { describe, expect, it } from "vitest";
import {
    encode_url_encoded_get_query,
    decode_url_encoded_get_query,
    extract_field,
    encode_get_query,
    type GetQuery,
    decode_get_query,
} from "@/lib/get_query";

describe("query_parser", () => {
    it("should parse valid query", () => {
        const input =
            "ordering%3Atest-asc%20paging%3A1-25%20filtering%3Atest-%7Cwarehouse%201%7C";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: "test",
                value: "warehouse 1",
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query different data", () => {
        const input =
            "ordering%3Aname-desc%20paging%3A3-30%20filtering%3Atype-%7Csome%20type%7C";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: "type",
                value: "some type",
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query different order", () => {
        const input =
            "filtering%3Atype-%7Csome%20type%7C%20paging%3A3-30%20ordering%3Aname-desc";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: "type",
                value: "some type",
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query with trailing spaces", () => {
        const input =
            "%20%20%20%20%20filtering%3Atype-%7Csome%20type%7C%20paging%3A3-30%20ordering%3Aname-desc%20%20%20%20";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: "type",
                value: "some type",
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query partial 1", () => {
        const input = "ordering%3Aname-desc";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: null,
                value: null,
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query partial 2", () => {
        const input = "paging%3A3-30";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: null,
                value: null,
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid query partial 3", () => {
        const input = "filtering%3Atype-%7Csome%20type%7C";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: "type",
                value: "some type",
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse valid empty query", () => {
        const input = "";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: null,
                value: null,
            },
        };
        expect(result).toEqual(expected_result);
    });
    it("should parse empty spaces", () => {
        const input = "   ";
        const result = decode_url_encoded_get_query(input);
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
                filter_by: null,
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
                filter_by: "test",
                value: "warehouse 1",
            },
        };
        const result = encode_url_encoded_get_query(input);
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
                filter_by: "type",
                value: "some type",
            },
        };
        const result = encode_url_encoded_get_query(input);
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
                filter_by: null,
                value: null,
            },
        };
        const result = encode_url_encoded_get_query(input);
        const expected_result = "";

        expect(result).toEqual(expected_result);
    });
    it("should convert to partial 1", () => {
        const input = {
            ordering: {
                order_by: "name",
                order: "desc",
            },
        };
        const result = encode_url_encoded_get_query(input);
        const expected_result = "ordering%3Aname-desc";

        expect(result).toEqual(expected_result);
    });
    it("should convert to partial 2", () => {
        const input = {
            paging: {
                page: 3,
                limit: 30,
            },
        };
        const result = encode_url_encoded_get_query(input);
        const expected_result = "paging%3A3-30";

        expect(result).toEqual(expected_result);
    });
    it("should convert to partial 3", () => {
        const input = {
            filtering: {
                filter_by: "type",
                value: "some type",
            },
        };
        const result = encode_url_encoded_get_query(input);
        const expected_result = "filtering%3Atype-%7Csome%20type%7C";

        expect(result).toEqual(expected_result);
    });
});

describe("extract_field", () => {
    it("should extract ordering", () => {
        const input =
            "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        const result = extract_field(input, "ordering:");
        const expected_result = "ordering:test-asc";
        expect(result).toEqual(expected_result);
    });
    it("should extract paging", () => {
        const input =
            "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        const result = extract_field(input, "paging:");
        const expected_result = "paging:1-25";
        expect(result).toEqual(expected_result);
    });
    it("should extract filtering", () => {
        const input =
            "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        const result = extract_field(input, "filtering:");
        const expected_result = "filtering:test-|warehouse 1|";
        expect(result).toEqual(expected_result);
    });
});

describe("encode_get_query", () => {
    it("should encode valid get query", () => {
        const get_query_params: GetQuery = {
            ordering: {
                order_by: "test",
                order: "asc",
            },
            paging: {
                page: 1,
                limit: 25,
            },
            filtering: {
                filter_by: "test",
                value: "warehouse 1",
            },
        };
        const result = encode_get_query(get_query_params);
        const expectedResult =
            "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";

        expect(result).toEqual(expectedResult);
    });

    it("should encode valid get query with dashes", () => {
        const get_query_params: GetQuery = {
            ordering: {
                order_by: "type-with-dash",
                order: "desc",
            },
            paging: {
                page: 1,
                limit: 25,
            },
            filtering: {
                filter_by: "test",
                value: "KD-ML-2026-1",
            },
        };
        const result = encode_get_query(get_query_params);
        const expectedResult =
            "ordering:type-with-dash-desc paging:1-25 filtering:test-|KD-ML-2026-1|";

        expect(result).toEqual(expectedResult);
    });
});

describe("decode_get_query", () => {
    it("should decode valid get query", () => {
        const input =
            "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        const result = decode_get_query(input);
        const expectedResult: GetQuery = {
            ordering: {
                order_by: "test",
                order: "asc",
            },
            paging: {
                page: 1,
                limit: 25,
            },
            filtering: {
                filter_by: "test",
                value: "warehouse 1",
            },
        };

        expect(result).toEqual(expectedResult);
    });

    it("should decode valid get query with dashes", () => {
        const input =
            "ordering:type-with-dash-desc paging:1-25 filtering:test-|KD-ML-2026-1|";
        const result = decode_get_query(input);
        const expectedResult: GetQuery = {
            ordering: {
                order_by: "type-with-dash",
                order: "desc",
            },
            paging: {
                page: 1,
                limit: 25,
            },
            filtering: {
                filter_by: "test",
                value: "KD-ML-2026-1",
            },
        };

        expect(result).toEqual(expectedResult);
    });
});
