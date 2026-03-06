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
import React, { useEffect, useMemo, useCallback } from "react";
import type { GetQuery } from "@/lib/get_query";

export function useDataDisplayCommon(
  updateSpecialQueryParams: ((parsedQuery: GetQuery) => void) | null,
) {
  const [searchParams, setSearchParams] = useSearchParams();
  const [page, setPage] = React.useState<number>(1);
  const [limit, setLimit] = React.useState<number>(25);
  const [total, setTotal] = React.useState<number>(1);
  const [orderBy, setOrderBy] = React.useState("created_at");
  const [order, setOrder] = React.useState("asc");
  const rawQuery = useMemo(() => searchParams.get("q"), [searchParams]);
  const parsedQuery = useMemo(() => query_parser(rawQuery), [rawQuery]);
  const [filterBy, setFilterBy] = React.useState<string>("");
  const [filterValue, setFilterValue] = React.useState<string>("");

  const updateCommonQueryParams = useCallback((getQuery: GetQuery) => {
    if (
      typeof getQuery.paging?.page === "number" &&
      typeof getQuery.paging?.limit === "number"
    ) {
      setPage(getQuery.paging.page);
      setLimit(getQuery.paging.limit);
    } else {
      setPage(1);
      setLimit(25);
    }
    if (
      typeof getQuery.ordering?.order_by === "string" &&
      typeof getQuery.ordering?.order === "string"
    ) {
      setOrderBy(getQuery.ordering.order_by);
      setOrder(getQuery.ordering?.order);
    } else {
      setOrderBy("created_at");
      setOrder("asc");
    }
    if (
      typeof getQuery?.filtering?.filter_by === "string" &&
      typeof getQuery?.filtering?.value === "string"
    ) {
      setFilterBy(getQuery.filtering.filter_by);
      setFilterValue(getQuery.filtering.value);
    }
  }, []);

  useEffect(() => {
    // TODO: improve this
    // eslint-disable-next-line react-hooks/set-state-in-effect
    updateCommonQueryParams(parsedQuery);
    if (typeof updateSpecialQueryParams === "function") {
      updateSpecialQueryParams(parsedQuery);
    }
  }, [parsedQuery, updateSpecialQueryParams, updateCommonQueryParams]);

  const paginatorSelect = (pageNumber: number) => {
    const currentQuery = query_parser(searchParams.get("q"));
    if (
      typeof currentQuery.paging?.page === "number" &&
      typeof currentQuery.paging?.limit === "number"
    ) {
      currentQuery.paging.page = pageNumber;
    } else {
      currentQuery.paging = {
        page: pageNumber,
        limit,
      };
    }
    searchParams.set("q", query_encoder(currentQuery));
    setSearchParams(searchParams);
  };
  const orderSelect = (orderBy: string) => {
    const currentQuery = query_parser(searchParams.get("q"));

    if (
      typeof currentQuery.ordering?.order_by === "string" &&
      typeof currentQuery.ordering?.order === "string"
    ) {
      currentQuery.ordering = {
        order_by: orderBy,
        order: currentQuery.ordering?.order === "desc" ? "asc" : "desc",
      };
    } else {
      currentQuery.ordering = {
        order_by: orderBy,
        order: "asc",
      };
    }
    searchParams.set("q", query_encoder(currentQuery));
    setSearchParams(searchParams);
  };

  const filterSelect = (field: string, value: string) => {
    const currentQuery = query_parser(searchParams.get("q"));
    if (value.trim().length > 0) {
      currentQuery.filtering = {
        filter_by: field,
        value,
      };
      searchParams.set("q", query_encoder(currentQuery));
      setSearchParams(searchParams);
    } else {
      delete currentQuery.filtering;
      searchParams.set("q", query_encoder(currentQuery));
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
    filterBy,
    setFilterBy,
    filterValue,
    setFilterValue,
  };
}
