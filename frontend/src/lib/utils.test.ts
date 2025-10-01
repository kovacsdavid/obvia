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

import {describe, expect, it} from "vitest";
import {query_encoder, query_parser} from "@/lib/utils.ts";

describe("query_parser", () => {
  it("should parse valid query", () => {
    const input = "page%3A1%7Climit%3A25%7Cname%3A%C3%A9%C3%A1%C5%B1%C3%BA";
    const result = query_parser(input);
    const expected_result = {page: 1, limit: 25, name: 'éáűú'};
    expect(result).toEqual(expected_result);
  });
  it("should skip empty values", () => {
    const input = "page%7Climit%3A%7Cname%3A%C3%A9%C3%A1%C5%B1%C3%BA";
    const result = query_parser(input);
    const expected_result = {name: 'éáűú'};
    expect(result).toEqual(expected_result);
  });
  it("should handle whitespaces", () => {
    const input = "   ";
    const result = query_parser(input);
    const expected_result = {};
    expect(result).toEqual(expected_result);
  });
  it("should trim whitespaces", () => {
    const input = "page%3A1%20%20%20%20%20%20%20%7Climit%3A25%7Cname%3A%20%20%20%20%20%20%20%20%20%20%20%C3%A9%C3%A1%C5%B1%C3%BA";
    const result = query_parser(input);
    const expected_result = {page: 1, limit: 25, name: 'éáűú'};
    expect(result).toEqual(expected_result);
  });
});

describe("query_encoder", () => {
  it("should convert to valid query", () => {
    const input = {page: 1, limit: 25, name: 'éáűú'};
    const result = query_encoder(input);
    const expected_result = "page%3A1%7Climit%3A25%7Cname%3A%C3%A9%C3%A1%C5%B1%C3%BA";

    expect(result).toEqual(expected_result);
  });
  it("should trim string values", () => {
    const input = {page: 1, limit: 25, name: '   éáűú   '};
    const result = query_encoder(input);
    const expected_result = "page%3A1%7Climit%3A25%7Cname%3A%C3%A9%C3%A1%C5%B1%C3%BA";

    expect(result).toEqual(expected_result);
  });
});