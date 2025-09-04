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
import {Funnel, Plus, SquarePen} from "lucide-react";
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
import React, {useEffect} from "react";
import {useDataDisplayCommon} from "@/hooks/use_data_display_common.ts";
import { Paginator } from "@/components/ui/pagination.tsx";
import {query_parser} from "@/lib/utils.ts";

interface Errors {
  global: string | null
}

export default function List() {
  const dispatch = useAppDispatch();
  const [errors, setErrors] = React.useState<Errors | null>(null);
  const {
    searchParams,
    page,
    setPage,
    setLimit,
    setTotal,
    // orderBy,
    setOrderBy,
    // order,
    setOrder,
    paginatorSelect,
    // orderSelect,
    // filterSelect,
    totalPages
  } = useDataDisplayCommon();

  useEffect(() => {
    setErrors({global: "Not implemented yet!"});
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
  }, [
    searchParams,
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
          <Link to={"/leltar/uj"}>
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
            <TableHead>
              Név
            </TableHead>
            <TableHead>
              Létrehozva
            </TableHead>
            <TableHead>
              Frissítve
            </TableHead>
            <TableHead>
              Műveletek
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell>
              Placeholder
            </TableCell>
            <TableCell>
              2025-08-31T13:52:49.213086+02:00
            </TableCell>
            <TableCell>
              2025-08-31T13:52:49.213086+02:00
            </TableCell>
            <TableCell>
              <Button style={{cursor: "pointer"}} variant={"outline"}>
                <SquarePen color={"green"}/>
              </Button>
            </TableCell>
          </TableRow>
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