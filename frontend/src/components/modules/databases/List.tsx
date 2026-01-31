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

import { Link } from "react-router-dom";
import React, { useCallback, useEffect } from "react";
import { list } from "@/components/modules/databases/lib/slice.ts";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  SortableTableHead,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table.tsx";
import { Paginator } from "@/components/ui/pagination.tsx";
import { Funnel, MoreHorizontal, PlugZap, Plus, Trash } from "lucide-react";
import { Button } from "@/components/ui/button.tsx";
import { Input } from "@/components/ui/input.tsx";
import { Label } from "@/components/ui/label.tsx";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover.tsx";
import { GlobalError } from "@/components/ui";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { type DatabaseList } from "@/components/modules/databases/lib/interface.ts";
import { useActivateDatabase } from "@/hooks/use_activate_database.ts";
import { formatDateToYMDHMS } from "@/lib/utils.ts";
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
  const [nameFilter, setNameFilter] = React.useState<string>("");
  const dispatch = useAppDispatch();
  const [data, setData] = React.useState<DatabaseList>([]);
  const { errors, setErrors, unexpectedError } = useSimpleError();

  const updateSpecialQueryParams = useCallback(
    (parsedQuery: Record<string, string | number>) => {
      if ("name" in parsedQuery) {
        setNameFilter(parsedQuery["name"] as string);
      }
    },
    [],
  );

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

  const activateDatabase = useActivateDatabase();

  useEffect(() => {
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
    searchParams,
    rawQuery,
    dispatch,
    setOrder,
    setOrderBy,
    setLimit,
    setPage,
    setTotal,
    setErrors,
    unexpectedError,
  ]);

  const handleActivate = async (new_tenant_id: string) => {
    await activateDatabase(new_tenant_id);
  };

  return (
    <>
      <GlobalError error={errors} />
      <Card>
        <CardHeader>
          <CardTitle>Adatbázisok</CardTitle>
        </CardHeader>
        <CardContent>
          <div className={"flex justify-between items-center mb-6"}>
            <div className="flex gap-2">
              <Link to={"/adatbazis/letrehozas"}>
                <Button style={{ color: "green" }} variant="outline">
                  <Plus color="green" /> Új
                </Button>
              </Link>
            </div>
            <div className="flex gap-2">
              <Popover>
                <PopoverTrigger asChild>
                  <Button variant="outline">
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
                        <Label htmlFor="name">Név</Label>
                        <Input
                          id="name"
                          onBlur={(e) => filterSelect("name", e.target.value)}
                          value={nameFilter}
                          onChange={(e) => setNameFilter(e.target.value)}
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
                <SortableTableHead
                  field="name"
                  orderBy={orderBy}
                  order={order}
                  onOrderSelect={orderSelect}
                >
                  Név
                </SortableTableHead>
                <SortableTableHead
                  field="created_at"
                  orderBy={orderBy}
                  order={order}
                  onOrderSelect={orderSelect}
                >
                  Létrehozva
                </SortableTableHead>
                <SortableTableHead
                  field="updated_at"
                  orderBy={orderBy}
                  order={order}
                  onOrderSelect={orderSelect}
                >
                  Frissítve
                </SortableTableHead>
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
                        <DropdownMenuItem
                          onClick={() => handleActivate(item.id)}
                        >
                          <PlugZap /> Aktiválás
                        </DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>
                          <Trash /> Törlés
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                  <TableCell>{item.name}</TableCell>
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
