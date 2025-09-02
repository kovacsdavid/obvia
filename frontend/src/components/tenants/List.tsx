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

import {Link, useSearchParams} from "react-router-dom";
import {query_encoder, query_parser} from "@/lib/utils.ts";
import React, {useEffect} from "react";
import {activate, list} from "@/store/slices/tenants.ts";
import { useAppDispatch } from "@/store/hooks";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from "@/components/ui/table.tsx";
import {isActivateResponse, isTenantsList, type TenantData} from "@/services/tenants.ts";
import Paginator from "@/components/ui/Paginator.tsx";
import {AlertCircle, ArrowDownAZ, ArrowUpAZ, Funnel, PlugZap, Plus} from "lucide-react";
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {Alert, AlertDescription, Tooltip, TooltipContent, TooltipTrigger} from "@/components/ui";
import {updateToken} from "@/store/slices/auth.ts";

interface Errors {
  global: string | null
}

export default function List() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [page, setPage] = React.useState<number>(1);
  const [limit, setLimit] = React.useState<number>(25);
  const [total, setTotal] = React.useState<number>(0);
  const [orderBy, setOrderBy] = React.useState("created_at");
  const [order, setOrder] = React.useState("asc");
  const [nameFilter, setNameFilter] = React.useState<string>("");
  const dispatch = useAppDispatch();
  const [data, setData] = React.useState<TenantData[]>([]);
  const [errors, setErrors] = React.useState<Errors | null>(null);

  const totalPages = React.useMemo(() => {
    return limit > 0 ? Math.ceil(total / limit) : 0;
  }, [total, limit]);

  const paginatorSelect = (pageNumber: number) => {
    const current_query = query_parser(searchParams.get("q"));
    current_query.page = pageNumber;
    searchParams.set("q", query_encoder(current_query))
    setSearchParams(searchParams)
  };

  const orderSelect = (orderBy: string) => {
    const current_query = query_parser(searchParams.get("q"));
    current_query.order_by = orderBy;
    current_query.order = order === "asc" ? "desc" : "asc";
    searchParams.set("q", query_encoder(current_query))
    setSearchParams(searchParams)
  }

  const filterSelect = (filterBy: string, value: string) => {
    if (value.trim().length > 0) {
      const current_query = query_parser(searchParams.get("q"));
      current_query[filterBy] = value;
      searchParams.set("q", query_encoder(current_query))
      setSearchParams(searchParams)
    } else {
      const current_query = query_parser(searchParams.get("q"));
      delete current_query[filterBy]
      searchParams.set("q", query_encoder(current_query))
      setSearchParams(searchParams)
    }
  }

  useEffect(() => {
    const parsed_query = query_parser(searchParams.get("q"));
    if ("page" in parsed_query) {
      setPage(parsed_query["page"] as number);
    }
    if ("limit" in parsed_query) {
      setLimit(parsed_query["limit"] as number);
    }
    if ("order_by" in parsed_query) {
      setOrderBy(parsed_query["order_by"] as string);
    }
    if ("order" in parsed_query) {
      setOrder(parsed_query["order"] as string);
    }
    if ("name" in parsed_query) {
      setNameFilter(parsed_query["name"] as string);
    }
    dispatch(list(searchParams.get("q"))).then(async (response) => {
      if (response.meta.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200: {
              if (isTenantsList(responseData)) {
                setPage(responseData.data.page);
                setLimit(responseData.data.limit);
                setTotal(responseData.data.total);
                setData(responseData.data.data);
              } else {
                setErrors({
                  global: "Váratlan hiba történt a feldolgozás során!",
                });
              }
              break;
            }
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
          });
        }
      }
    })
  }, [searchParams, dispatch]);

  const handleActivate = async (new_tenant_id: string) => {
    dispatch(activate(new_tenant_id)).then((response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        if (isActivateResponse(response.payload)) {
          dispatch(updateToken(response.payload.data))
        }
      }
    })
  }

  return (
    <>
    { errors !== null && errors.global !== null ? (
        <Alert className={"mb-5"} variant="destructive">
          <AlertCircle/>
          <AlertDescription>
            {errors.global}
          </AlertDescription>
        </Alert>
      ) : null }
      <div className={"flex justify-between items-center mb-6"}>
        <div className="flex gap-2">
          <Link to={"/szervezeti_egyseg/uj"}>
            <Button style={{color: "green"}} variant="outline">
              <Plus color="green"/> Új
            </Button>
          </Link>
        </div>
        <div className="flex gap-2">
          <Popover>
            <PopoverTrigger asChild>
              <Button variant="outline">Szűrő <Funnel/></Button>
            </PopoverTrigger>
            <PopoverContent className="w-80">
              <div className="grid gap-4">
                <div className="space-y-2">
                  <h4 className="leading-none font-medium">Szűrő</h4>
                  <p className="text-muted-foreground text-sm">
                    Szűkítsd a találatok listáját szűrőfeltételekkel!
                  </p>
                </div>
                <div className="grid gap-2">
                  <div className="grid grid-cols-3 items-center gap-4">
                    <Label htmlFor="name">Név</Label>
                    <Input
                      id="name"
                      onBlur={e => filterSelect("name", e.target.value)}
                      value={nameFilter}
                      onChange={e => setNameFilter(e.target.value)}
                      defaultValue=""
                      className="col-span-2 h-8"
                    />
                  </div>
                </div>
              </div>
            </PopoverContent>
          </Popover>
        </div>
      </div>


      <Table>
        <TableHeader>
          <TableRow>
            <TableHead
              style={{cursor: "pointer"}}
              onClick={() => orderSelect("name")}>
              Név {orderBy === "name"
              ? order === "asc"
                ? (<ArrowDownAZ style={{display: "inline"}}/>)
                : <ArrowUpAZ style={{display: "inline"}}/>
              : null}
            </TableHead>
            <TableHead>Adatbázis kiszolgáló</TableHead>
            <TableHead
              style={{cursor: "pointer"}}
              onClick={() => orderSelect("created_at")
              }>
              Létrehozva {orderBy === "created_at"
              ? order === "asc"
                ? (<ArrowDownAZ style={{display: "inline"}}/>)
                : <ArrowUpAZ style={{display: "inline"}}/>
              : null}
            </TableHead>
            <TableHead
              style={{cursor: "pointer"}}
              onClick={() => orderSelect("updated_at")}
            >
              Frissítve {orderBy === "updated_at"
              ? order === "asc"
                ? (<ArrowDownAZ style={{display: "inline"}}/>)
                : <ArrowUpAZ style={{display: "inline"}}/>
              : null}
            </TableHead>
            <TableHead>
              Műveletek
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.map((item) => (
            <TableRow key={item.id}>
              <TableCell>{item.name}</TableCell>
              <TableCell>{item.db_host}:{item.db_port}</TableCell>
              <TableCell>{item.created_at}</TableCell>
              <TableCell>{item.updated_at}</TableCell>
              <TableCell>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <Button style={{cursor: "pointer"}} onClick={() => handleActivate(item.id)} variant={"outline"}>
                      <PlugZap color={"green"}/>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side={"left"}>
                    <p>Aktiválás</p>
                  </TooltipContent>
                </Tooltip>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Paginator
        page={page}
        totalPages={totalPages}
        onPageChange={paginatorSelect}
      />
    </>
  )
}