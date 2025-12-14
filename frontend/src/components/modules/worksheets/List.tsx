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

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover.tsx";
import { Button, GlobalError, Input, Label } from "@/components/ui";
import { Eye, Funnel, MoreHorizontal, Pencil, Plus, Trash } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table.tsx";
import { Link } from "react-router-dom";
import { useAppDispatch } from "@/store/hooks.ts";
import React, { useCallback, useEffect } from "react";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { Paginator } from "@/components/ui/pagination.tsx";
import { deleteItem, list } from "@/components/modules/worksheets/lib/slice.ts";
import { type WorksheetResolvedList } from "@/components/modules/worksheets/lib/interface.ts";
import { formatDateToYMDHMS, formatNumber } from "@/lib/utils.ts";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { useSimpleError } from "@/hooks/use_simple_error.ts";

export default function List() {
  const dispatch = useAppDispatch();
  const { errors, setErrors, unexpectedError } = useSimpleError();
  const [data, setData] = React.useState<WorksheetResolvedList>([]);
  const updateSpecialQueryParams = useCallback(
    (parsedQuery: Record<string, string | number>) => {
      console.log(parsedQuery);
    },
    [],
  );

  const {
    //searchParams,
    rawQuery,
    page,
    setPage,
    setLimit,
    setTotal,
    //orderBy,
    //setOrderBy,
    //order,
    //setOrder,
    paginatorSelect,
    //orderSelect,
    //filterSelect,
    totalPages,
  } = useDataDisplayCommon(updateSpecialQueryParams);

  const refresh = useCallback(() => {
    dispatch(list(rawQuery)).then(async (response) => {
      if (list.fulfilled.match(response)) {
        if (
          response.payload.statusCode === 200 &&
          typeof response.payload.jsonData?.data !== "undefined" &&
          typeof response.payload.jsonData?.meta !== "undefined"
        ) {
          setPage(response.payload.jsonData.meta.page);
          setLimit(response.payload.jsonData.meta.limit);
          setTotal(response.payload.jsonData.meta.total);
          setData(response.payload.jsonData.data);
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    dispatch,
    rawQuery,
    setLimit,
    setPage,
    setTotal,
    setErrors,
    unexpectedError,
  ]);

  const handleDelete = (id: string) => {
    dispatch(deleteItem(id)).then(async (response) => {
      if (deleteItem.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          refresh();
        }
      }
    });
  };

  useEffect(() => {
    refresh();
  }, [refresh]);

  return (
    <>
      <GlobalError error={errors} />
      <Card>
        <CardHeader>
          <CardTitle>Munkalapok</CardTitle>
        </CardHeader>
        <CardContent>
          <div className={"flex justify-between items-center mb-6"}>
            <div className="flex gap-2">
              <Link to={"/munkalap/letrehozas"}>
                <Button style={{ color: "green" }} variant="outline">
                  <Plus color="green" /> Új
                </Button>
              </Link>
            </div>
            <div className="flex gap-2">
              <Popover>
                <PopoverTrigger asChild>
                  <Button
                    className={"justify-self-end"}
                    variant="outline"
                    style={{ marginBottom: "25px" }}
                  >
                    Szűrő <Funnel />
                  </Button>
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
                <TableHead />
                <TableHead>Név</TableHead>
                <TableHead>Leírás</TableHead>
                <TableHead>Vevő</TableHead>
                <TableHead>Nettó anyagköltség</TableHead>
                <TableHead>Bruttó anyagköltség</TableHead>
                <TableHead>Nettó munkadíj</TableHead>
                <TableHead>Bruttó munkadíj</TableHead>
                <TableHead>Létrehozta</TableHead>
                <TableHead>Létrehozva</TableHead>
                <TableHead>Frissítve</TableHead>
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
                        <Link to={`/munkalap/reszletek/${item.id}`}>
                          <DropdownMenuItem>
                            <Eye /> Részletek
                          </DropdownMenuItem>
                        </Link>
                        <Link to={`/munkalap/modositas/${item.id}`}>
                          <DropdownMenuItem>
                            <Pencil /> Szerkesztés
                          </DropdownMenuItem>
                        </Link>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem
                          className={"cursor-pointer"}
                          onClick={() => handleDelete(item.id)}
                        >
                          <Trash /> Törlés
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                  <TableCell>{item.name}</TableCell>
                  <TableCell>
                    {item.description ? item.description : ""}
                  </TableCell>
                  <TableCell>{item.customer}</TableCell>
                  <TableCell>{formatNumber(item.net_material_cost)}</TableCell>
                  <TableCell>
                    {formatNumber(item.gross_material_cost)}
                  </TableCell>
                  <TableCell>{formatNumber(item.net_work_cost)}</TableCell>
                  <TableCell>{formatNumber(item.gross_work_cost)}</TableCell>
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
        </CardContent>
      </Card>
    </>
  );
}
