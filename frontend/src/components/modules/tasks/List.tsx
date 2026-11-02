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
import { Button, Input, Label } from "@/components/ui";
import {
    Eye,
    Funnel,
    MoreHorizontal,
    Pencil,
    Plus,
    Trash,
    Link as LinkIcon,
    Printer,
} from "lucide-react";
import {
    SortableTableHead,
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table.tsx";
import { Link, NavLink } from "react-router";
import { useAppDispatch } from "@/store/hooks.ts";
import React, { useCallback, useEffect } from "react";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { Paginator } from "@/components/ui/pagination.tsx";
import {
    deleteItem,
    list,
    print,
} from "@/components/modules/tasks/lib/slice.ts";
import { type TaskResolvedList } from "@/components/modules/tasks/lib/interface.ts";
import {
    formatDateLocal,
    formatNumber,
    openPopup,
    updateWindowWithPdfData,
} from "@/lib/utils.ts";
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
import Status from "./Status";
import Priority from "./Priority";
import { useAuth } from "@/hooks/use_auth";
import { Badge } from "@/components/ui/badge";
import { ButtonGroup } from "@/components/ui/button-group";
import { handleApiResult } from "@/lib/api/handleApiResult";

export default function List() {
    const dispatch = useAppDispatch();
    const [data, setData] = React.useState<TaskResolvedList>([]);
    const { claims } = useAuth();

    const {
        rawQuery,
        page,
        setPage,
        setLimit,
        setTotal,
        orderBy,
        order,
        paginatorSelect,
        orderSelect,
        filterSelect,
        totalPages,
        filterValue,
        setFilterValue,
    } = useDataDisplayCommon(null);

    const refresh = useCallback(async () => {
        const result = await dispatch(list({ query: rawQuery })).unwrap();

        handleApiResult(result, {
            onSuccess: ({ meta, data }) => {
                if (typeof meta !== "undefined") {
                    setPage(meta.page);
                    setLimit(meta.limit);
                    setTotal(meta.total);
                }
                if (typeof data !== "undefined") {
                    setData(data);
                }
            },
        });
    }, [dispatch, rawQuery, setLimit, setPage, setTotal]);

    const handleDelete = async (id: string) => {
        const result = await dispatch(deleteItem({ uuid: id })).unwrap();

        handleApiResult(result, {
            onSuccess: () => {
                refresh();
            },
        });
    };

    const handlePrint = (id: string) => {
        const handler = openPopup();

        if (!handler) {
            return;
        }

        dispatch(print({ uuid: id }))
            .unwrap()
            .then((result) => {
                handleApiResult(result, {
                    onSuccess: (data) => {
                        if (data !== null) {
                            updateWindowWithPdfData(handler, data);
                        } else {
                            handler.close();
                            return;
                        }
                    },
                    onError: () => {
                        handler.close();
                        return;
                    },
                });
            })
            .catch(() => {
                handler.close();
                return;
            });
    };

    useEffect(() => {
        refresh();
    }, [refresh]);

    return (
        <>
            <Card>
                <CardHeader>
                    <CardTitle>Feladatok</CardTitle>
                </CardHeader>
                <CardContent>
                    <ButtonGroup
                        orientation="responsive"
                        className="ml-auto mb-5"
                    >
                        <Popover>
                            <PopoverTrigger asChild>
                                <Button variant="outline">
                                    Szűrő <Funnel />
                                </Button>
                            </PopoverTrigger>
                            <PopoverContent className="w-80">
                                <div className="grid gap-4">
                                    <div className="space-y-2">
                                        <h4 className="leading-none font-medium">
                                            Szűrő
                                        </h4>
                                        <p className="text-muted-foreground text-sm">
                                            Szűkítsd a találatok listáját
                                            szűrőfeltételekkel!
                                        </p>
                                    </div>
                                    <div className="grid gap-2">
                                        <div className="grid grid-cols-3 items-center gap-4">
                                            <Label htmlFor="name">Név</Label>
                                            <Input
                                                id="name"
                                                value={filterValue}
                                                onChange={(e) =>
                                                    setFilterValue(
                                                        e.target.value,
                                                    )
                                                }
                                                onBlur={(e) =>
                                                    filterSelect(
                                                        "name",
                                                        e.target.value,
                                                    )
                                                }
                                                className="col-span-2 h-8"
                                            />
                                        </div>
                                    </div>
                                </div>
                            </PopoverContent>
                        </Popover>
                        <Button
                            style={{ color: "green" }}
                            variant="outline"
                            asChild
                        >
                            <Link to={"/feladat/letrehozas"}>
                                Új <Plus color="green" />
                            </Link>
                        </Button>
                    </ButtonGroup>
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead />
                                <SortableTableHead
                                    field="service"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Szolgáltatás
                                </SortableTableHead>
                                <TableHead>Leírás</TableHead>
                                <SortableTableHead
                                    field="worksheet"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Munkalap
                                </SortableTableHead>
                                <SortableTableHead
                                    field="quantity"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Munkaóra
                                </SortableTableHead>
                                <SortableTableHead
                                    field="price"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Ár
                                </SortableTableHead>
                                <TableHead>Pénznem</TableHead>
                                <TableHead>Adó</TableHead>
                                <SortableTableHead
                                    field="status"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Státusz
                                </SortableTableHead>
                                <SortableTableHead
                                    field="priority"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Prioritás
                                </SortableTableHead>
                                <SortableTableHead
                                    field="due_date"
                                    orderBy={orderBy}
                                    order={order}
                                    onOrderSelect={orderSelect}
                                >
                                    Határidő
                                </SortableTableHead>
                                <TableHead>Létrehozta</TableHead>
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
                                                <Button
                                                    variant="ghost"
                                                    className="h-8 w-8 p-0"
                                                >
                                                    <span className="sr-only">
                                                        Menü megnyitása
                                                    </span>
                                                    <MoreHorizontal />
                                                </Button>
                                            </DropdownMenuTrigger>
                                            <DropdownMenuContent
                                                side={"bottom"}
                                                align="start"
                                            >
                                                <DropdownMenuLabel>
                                                    Műveletek
                                                </DropdownMenuLabel>
                                                <Link
                                                    to={`/feladat/reszletek/${item.id}`}
                                                >
                                                    <DropdownMenuItem>
                                                        <Eye /> Részletek
                                                    </DropdownMenuItem>
                                                </Link>
                                                <Link
                                                    to={`/feladat/modositas/${item.id}`}
                                                >
                                                    <DropdownMenuItem>
                                                        <Pencil /> Szerkesztés
                                                    </DropdownMenuItem>
                                                </Link>
                                                <DropdownMenuItem
                                                    className={"cursor-pointer"}
                                                    onClick={() =>
                                                        handlePrint(item.id)
                                                    }
                                                >
                                                    <Printer /> Nyomtatás
                                                </DropdownMenuItem>
                                                <DropdownMenuSeparator />
                                                <DropdownMenuItem
                                                    className={"cursor-pointer"}
                                                    onClick={() =>
                                                        handleDelete(item.id)
                                                    }
                                                >
                                                    <Trash /> Törlés
                                                </DropdownMenuItem>
                                            </DropdownMenuContent>
                                        </DropdownMenu>
                                    </TableCell>
                                    <TableCell>{item.service}</TableCell>
                                    <TableCell>
                                        {item.description ?? ""}
                                    </TableCell>
                                    <TableCell>{item.worksheet}</TableCell>
                                    <TableCell>
                                        {item.quantity
                                            ? formatNumber(item.quantity)
                                            : "N/A"}
                                    </TableCell>
                                    <TableCell>
                                        {item.price
                                            ? formatNumber(item.price)
                                            : "N/A"}
                                    </TableCell>
                                    <TableCell>
                                        <Badge variant="secondary">
                                            {item.currency_code}
                                        </Badge>
                                    </TableCell>
                                    <TableCell>
                                        <NavLink
                                            to={`/ado/reszletek/${item.tax_id}`}
                                        >
                                            {item.tax}{" "}
                                            <LinkIcon
                                                size={15}
                                                className="inline"
                                            />
                                        </NavLink>
                                    </TableCell>
                                    <TableCell>
                                        <Status status={item.status} />
                                    </TableCell>
                                    <TableCell>
                                        {item.priority ? (
                                            <Priority
                                                priority={item.priority}
                                            />
                                        ) : (
                                            "N/A"
                                        )}
                                    </TableCell>
                                    <TableCell>
                                        {item.due_date
                                            ? formatDateLocal(
                                                  item.due_date,
                                                  claims?.loc,
                                                  claims?.tz,
                                              )
                                            : "N/A"}
                                    </TableCell>
                                    <TableCell>{item.created_by}</TableCell>
                                    <TableCell>
                                        {formatDateLocal(
                                            item.created_at,
                                            claims?.loc,
                                            claims?.tz,
                                        )}
                                    </TableCell>
                                    <TableCell>
                                        {formatDateLocal(
                                            item.updated_at,
                                            claims?.loc,
                                            claims?.tz,
                                        )}
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
                </CardContent>
            </Card>
        </>
    );
}
