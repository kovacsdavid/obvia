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

import React, {useCallback, useEffect} from "react";
import {useAppDispatch} from "@/store/hooks.ts";
import {deleteItem, list} from "@/components/modules/inventory_movements/lib/slice.ts";
import type {InventoryMovementResolvedList} from "@/components/modules/inventory_movements/lib/interface.ts";
import {Card, CardAction, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {Button, GlobalError} from "@/components/ui";
import {Paginator} from "@/components/ui/pagination.tsx";
import {useDataDisplayCommon} from "@/hooks/use_data_display_common.ts";
import type {SimpleError} from "@/lib/interfaces/common.ts";
import {Eye, MoreHorizontal, Plus, Trash} from "lucide-react";
import {formatDateToYMDHMS} from "@/lib/utils.ts";
import {useParams} from "react-router";
import {Link} from "react-router-dom";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu.tsx";

export default function InventoryMovementsList() {
  const dispatch = useAppDispatch();
  const [errors, setErrors] = React.useState<SimpleError | null>(null);
  const [data, setData] = React.useState<InventoryMovementResolvedList>([]);
  const params = useParams();
  const routeInventoryId = React.useMemo(() => params["inventoryId"] ?? "", [params]);

  const updateSpecialQueryParams = useCallback((parsedQuery: Record<string, string | number>) => {
    console.log(parsedQuery);
  }, []);

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

  const unexpectedError = () => {
    setErrors({
      message: "Váratlan hiba történt a feldolgozás során!",
    });
  };

  const refresh = useCallback(() => {
    dispatch(list(rawQuery)).then(async (response) => {
      if (list.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          if (
            typeof response.payload.jsonData.data !== "undefined"
            && typeof response.payload.jsonData.meta !== "undefined"
          ) {
            setPage(response.payload.jsonData.meta.page);
            setLimit(response.payload.jsonData.meta.limit);
            setTotal(response.payload.jsonData.meta.total);
            setData(response.payload.jsonData.data);
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    })
  }, [dispatch, rawQuery, setPage, setLimit, setTotal])

  const handleDelete = (id: string) => {
    dispatch(deleteItem(id)).then(async (response) => {
      if (deleteItem.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          refresh();
        }
      }
    });
  }

  const resolveMovementType = (movementType: string): string => {
    switch (movementType) {
      case "in":
        return "Bevétel";
      case "out":
        return "Kiadás";
      default:
        return "N/A";
    }
  };

  useEffect(() => {
    refresh();
  }, [refresh]);

  return (
    <>
      <GlobalError error={errors}/>
      <Card>
        <CardHeader>
          <CardTitle>Készletmozgások</CardTitle>
          <CardAction>
            <Link to={`/leltar-mozgas/letrehozas/${routeInventoryId}`}>
              <Button style={{color: "green"}} variant="outline">
                <Plus color="green"/> Új
              </Button>
            </Link>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead/>
                <TableHead>Típus</TableHead>
                <TableHead>Mennyiség</TableHead>
                <TableHead>Egységár</TableHead>
                <TableHead>Összeg</TableHead>
                <TableHead>Adó</TableHead>
                <TableHead>Mozgás dátuma</TableHead>
                <TableHead>Létrehozta</TableHead>
                <TableHead>Létrehozva</TableHead>
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
                          <MoreHorizontal/>
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent side={"bottom"} align="start">
                        <DropdownMenuLabel>Műveletek</DropdownMenuLabel>
                        <Link to={`/leltar-mozgas/reszletek/${item.id}`}>
                          <DropdownMenuItem>
                            <Eye/> Részletek
                          </DropdownMenuItem>
                        </Link>
                        <DropdownMenuSeparator/>
                        <DropdownMenuItem className={"cursor-pointer"} onClick={() => handleDelete(item.id)}>
                          <Trash/> Törlés
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </TableCell>
                  <TableCell>{resolveMovementType(item.movement_type)}</TableCell>
                  <TableCell>{Math.abs(parseFloat(item.quantity))}</TableCell>
                  <TableCell>{item.unit_price ?? 'N/A'}</TableCell>
                  <TableCell>{item.total_price ?? 'N/A'}</TableCell>
                  <TableCell>{item.tax ?? 'N/A'}</TableCell>
                  <TableCell>{formatDateToYMDHMS(item.movement_date)}</TableCell>
                  <TableCell>{item.created_by}</TableCell>
                  <TableCell>{formatDateToYMDHMS(item.created_at)}</TableCell>
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
