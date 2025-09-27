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

import {Link} from "react-router-dom";
import React, {useCallback, useEffect} from "react";
import {activate, list} from "@/components/tenants/slice.ts";
import { useAppDispatch } from "@/store/hooks";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from "@/components/ui/table.tsx";
import { Paginator } from "@/components/ui/pagination.tsx";
import {ArrowDownAZ, ArrowUpAZ, Funnel, PlugZap, Plus} from "lucide-react";
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover"
import {GlobalError, Tooltip, TooltipContent, TooltipTrigger} from "@/components/ui";
import {updateToken} from "@/components/auth/slice.ts";
import {useDataDisplayCommon} from "@/hooks/use_data_display_common.ts";
import {type SimpeError} from "@/lib/interfaces/common.ts";
import {
  isActiveTenantResponse,
  isPaginatedTenantListResponse,
  type TenantList
} from "@/components/tenants/interface.ts";

export default function List() {
  const [nameFilter, setNameFilter] = React.useState<string>("");
  const dispatch = useAppDispatch();
  const [data, setData] = React.useState<TenantList>([]);
  const [errors, setErrors] = React.useState<SimpeError | null>(null);

  const updateSpecialQueryParams = useCallback((parsedQuery: Record<string, string | number>) => {
    if ("name" in parsedQuery) {
      setNameFilter(parsedQuery["name"] as string);
    }
  }, []);

  const unexpectedError = () => {
    setErrors({
      message: "Váratlan hiba történt a feldolgozás során!",
    });
  };

  const {
    searchParams,
    rawQuery,
    page,
    setPage,
    setLimit,
    setTotal,
    orderBy,
    setOrderBy,
    order,
    setOrder,
    paginatorSelect,
    orderSelect,
    filterSelect,
    totalPages,
  } = useDataDisplayCommon(updateSpecialQueryParams);

  useEffect(() => {
    dispatch(list(rawQuery)).then(async (response) => {
      if (response.meta.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200: {
              if (isPaginatedTenantListResponse(responseData)) {
                if (typeof responseData.data !== "undefined") {
                setPage(responseData.meta.page);
                setLimit(responseData.meta.limit);
                setTotal(responseData.meta.total);
                setData(responseData.data);
              }
              } else {
                unexpectedError();
              }
              break;
            }
            default:
              unexpectedError();
          }
        } catch {
          unexpectedError();
        }
      }
    })
  }, [
    searchParams,
    rawQuery,
    dispatch,
    setOrder,
    setOrderBy,
    setLimit,
    setPage,
    setTotal
  ]);

  const handleActivate = async (new_tenant_id: string) => {
    dispatch(activate(new_tenant_id)).then((response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        if (isActiveTenantResponse(response.payload)) {
          dispatch(updateToken(response.payload.data))
        }
      }
    })
  }

  return (
    <>
      <GlobalError error={errors} />
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