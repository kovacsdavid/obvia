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

export interface Ordering {
  order_by: string | null;
  order: string | null;
}

export interface Paging {
  page: number | null;
  limit: number | null;
}

export interface Filtering {
  field: string | null;
  value: string | null;
}

export interface GetQuery {
  ordering: Ordering;
  paging: Paging;
  filtering: Filtering;
}

function parse_ordering(str: string): Ordering {
  const collection = str.replace("ordering:", "").split("-");
  if (collection.length === 2) {
    return {
      order_by: collection[0],
      order: collection[1],
    };
  }
  return {
    order_by: null,
    order: null,
  };
}

function parse_paging(str: string): Paging {
  const collection = str.replace("paging:", "").split("-");
  const page = parseInt(collection[0], 10);
  const limit = parseInt(collection[1], 10);
  if (collection.length === 2 && !isNaN(page) && !isNaN(limit)) {
    return {
      page,
      limit,
    };
  }
  return {
    page: null,
    limit: null,
  };
}

function parse_filtering(str: string): Filtering {
  const collection = str.replace("filtering:", "").split("-");
  if (collection.length === 2) {
    return {
      field: collection[0],
      value: collection[1].replaceAll("|", ""),
    };
  }
  return {
    field: null,
    value: null,
  };
}

export function parse_get_query(str: string): GetQuery {
  return {
    ordering: parse_ordering(extract_field(str, "ordering:")),
    paging: parse_paging(extract_field(str, "paging:")),
    filtering: parse_filtering(extract_field(str, "filtering:")),
  };
}

export function extract_field(encodedStr: string, field: string): string {
  if (!encodedStr.includes(field)) {
    return "";
  }

  const start = encodedStr.indexOf(field);
  let end = encodedStr.length;
  const rest = encodedStr.substring(start, end);
  let inside_pipes = false;

  for (let i = 0; i < rest.length; i++) {
    if (rest[i] === "|") {
      inside_pipes = !inside_pipes;
    }
    if (!inside_pipes && i > 0 && rest[i] === " ") {
      end = i;
      break;
    }
  }
  return rest.substring(0, end);
}

export function encode_get_query(get_query: GetQuery): string {
  let result = ``;
  if (
    typeof get_query.ordering.order_by === "string" &&
    typeof get_query.ordering.order === "string"
  ) {
    result += `ordering:${get_query.ordering.order_by}-${get_query.ordering.order}`;
  }
  if (
    typeof get_query.paging.page === "number" &&
    typeof get_query.paging.limit === "number"
  ) {
    if (result.length > 0) {
      result += ` `;
    }
    result += `paging:${get_query.paging.page}-${get_query.paging.limit}`;
  }
  if (
    typeof get_query.filtering.field === "string" &&
    typeof get_query.filtering.value === "string"
  ) {
    if (result.length > 0) {
      result += ` `;
    }
    result += `filtering:${get_query.filtering.field}-|${get_query.filtering.value}|`;
  }
  return result;
}
