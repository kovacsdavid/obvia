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

import { useSearchParams } from "react-router-dom";
import { query_encoder, query_parser } from "@/lib/utils.ts";
import React, { useEffect, useMemo } from "react";

export function useDataDisplayCommon(
  updateSpecialQueryParams: (
    parsedQuery: Record<string, string | number>,
  ) => void,
) {
  const [searchParams, setSearchParams] = useSearchParams();
  const [page, setPage] = React.useState<number>(1);
  const [limit, setLimit] = React.useState<number>(25);
  const [total, setTotal] = React.useState<number>(1);
  const [orderBy, setOrderBy] = React.useState("created_at");
  const [order, setOrder] = React.useState("asc");
  const rawQuery = useMemo(() => searchParams.get("q"), [searchParams]);
  const parsedQuery = useMemo(() => query_parser(rawQuery), [rawQuery]);

  const updateCommonQueryParams = (
    parsedQuery: Record<string, string | number>,
  ) => {
    if ("page" in parsedQuery) {
      setPage(parsedQuery["page"] as number);
    }
    if ("limit" in parsedQuery) {
      setLimit(parsedQuery["limit"] as number);
    }
    if ("order_by" in parsedQuery) {
      setOrderBy(parsedQuery["order_by"] as string);
    }
    if ("order" in parsedQuery) {
      setOrder(parsedQuery["order"] as string);
    }
  };

  useEffect(() => {
    // TODO: improve this
    // eslint-disable-next-line react-hooks/set-state-in-effect
    updateCommonQueryParams(parsedQuery);
    updateSpecialQueryParams(parsedQuery);
  }, [parsedQuery, updateSpecialQueryParams]);

  const paginatorSelect = (pageNumber: number) => {
    const current_query = query_parser(searchParams.get("q"));
    current_query.page = pageNumber;
    searchParams.set("q", query_encoder(current_query));
    setSearchParams(searchParams);
  };
  const orderSelect = (orderBy: string) => {
    const current_query = query_parser(searchParams.get("q"));
    current_query.order_by = orderBy;
    current_query.order = current_query.order === "desc" ? "asc" : "desc";
    searchParams.set("q", query_encoder(current_query));
    setSearchParams(searchParams);
  };

  const filterSelect = (filterBy: string, value: string) => {
    if (value.trim().length > 0) {
      const current_query = query_parser(searchParams.get("q"));
      current_query[filterBy] = value;
      searchParams.set("q", query_encoder(current_query));
      setSearchParams(searchParams);
    } else {
      const current_query = query_parser(searchParams.get("q"));
      delete current_query[filterBy];
      searchParams.set("q", query_encoder(current_query));
      setSearchParams(searchParams);
    }
  };

  const totalPages = React.useMemo(() => {
    return limit > 0 ? Math.ceil(total / limit) : 0;
  }, [total, limit]);

  return {
    searchParams,
    rawQuery,
    page,
    setPage,
    limit,
    setLimit,
    total,
    setTotal,
    orderBy,
    setOrderBy,
    order,
    setOrder,
    paginatorSelect,
    orderSelect,
    filterSelect,
    totalPages,
    parsedQuery,
  };
}
