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

import {Popover, PopoverContent, PopoverTrigger} from "@/components/ui/popover.tsx";
import {Button, GlobalError, Input, Label} from "@/components/ui";
import {Eye, Funnel, MoreHorizontal, Pencil, Plus, Trash} from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow
} from "@/components/ui/table.tsx";
import {Link} from "react-router-dom";
import {useAppDispatch} from "@/store/hooks.ts";
import React, {useCallback, useEffect} from "react";
import {useDataDisplayCommon} from "@/hooks/use_data_display_common.ts";
import { Paginator } from "@/components/ui/pagination.tsx";
import {list} from "@/components/products/slice.ts";
import {type SimpeError} from "@/lib/interfaces/common.ts";
import {
  isPaginatedProductResolvedListResponse,
  type ProductResolvedList
} from "@/components/products/interface.ts";
import {formatDateToYMDHMS} from "@/lib/utils.ts";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel, DropdownMenuSeparator,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";

export default function List() {
  const dispatch = useAppDispatch();
  const [errors, setErrors] = React.useState<SimpeError | null>(null);
  const [data, setData] = React.useState<ProductResolvedList>([]);
  const updateSpecialQueryParams = useCallback((parsedQuery: Record<string, string | number>) => {
    console.log(parsedQuery);
  }, []);

  const {
    searchParams,
    rawQuery,
    page,
    setPage,
    setLimit,
    setTotal,
    //orderBy,
    setOrderBy,
    //order,
    setOrder,
    paginatorSelect,
    //orderSelect,
    //filterSelect,
    totalPages,
  } = useDataDisplayCommon(updateSpecialQueryParams);

  const unexpectedError = () => {
    setErrors({
      message: "Váratlan hiba történt a feldolgozás során!",
    });
  };

  useEffect(() => {
    dispatch(list(rawQuery)).then(async (response) => {
      if (response.meta.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200: {
              if (isPaginatedProductResolvedListResponse(responseData)) {
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
    });
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

  return (
    <>
      <GlobalError error={errors} />
      <div className={"flex justify-between items-center mb-6"}>
        <div className="flex gap-2">
          <Link to={"/termek/uj"}>
            <Button style={{color: "green"}} variant="outline">
              <Plus color="green"/> Új
            </Button>
          </Link>
        </div>
        <div className="flex gap-2">
          <Popover>
            <PopoverTrigger asChild>
              <Button className={"justify-self-end"} variant="outline"
                      style={{marginBottom: "25px"}}>Szűrő <Funnel/></Button>
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
                    <Label htmlFor="name">Szűrő</Label>
                    <Input
                      id="name"
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
            <TableHead/>
            <TableHead>
              Név
            </TableHead>
            <TableHead>
              Leírás
            </TableHead>
            <TableHead>
              Mértékegység
            </TableHead>
            <TableHead>
              Státusz
            </TableHead>
            <TableHead>
              Létrehozta
            </TableHead>
            <TableHead>
              Létrehozva
            </TableHead>
            <TableHead>
              Frissítve
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {data.map((item) => (
            <TableRow key={item.id}>
              <TableCell>
                <DropdownMenu>
                  <DropdownMenuTrigger asChild>
                    <Button variant="ghost" className="h-8 w-8 p-0">
                      <span className="sr-only">Menü megnyitása</span>
                      <MoreHorizontal />
                    </Button>
                  </DropdownMenuTrigger>
                  <DropdownMenuContent side={"bottom"} align="start">
                    <DropdownMenuLabel>Műveletek</DropdownMenuLabel>
                    <DropdownMenuItem>
                      <Eye/> Részletek
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                      <Pencil/> Szerkesztés
                    </DropdownMenuItem>
                    <DropdownMenuSeparator />
                    <DropdownMenuItem>
                      <Trash/> Törlés
                    </DropdownMenuItem>
                  </DropdownMenuContent>
                </DropdownMenu>
              </TableCell>
              <TableCell>{item.name}</TableCell>
              <TableCell>{item.description ? item.description : ''}</TableCell>
              <TableCell>{item.unit_of_measure}</TableCell>
              <TableCell>{item.status}</TableCell>
              <TableCell>{item.created_by}</TableCell>
              <TableCell>{formatDateToYMDHMS(item.created_at)}</TableCell>
              <TableCell>{formatDateToYMDHMS(item.updated_at)}</TableCell>
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