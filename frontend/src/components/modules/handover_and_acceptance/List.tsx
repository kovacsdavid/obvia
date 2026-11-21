/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import React, { useCallback, useEffect } from "react";
import { Link } from "react-router";
import {
    Eye,
    Funnel,
    MoreHorizontal,
    Pencil,
    Plus,
    Printer,
    Trash,
} from "lucide-react";
import { Button, Input, Label } from "@/components/ui";
import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card.tsx";
import { ButtonGroup } from "@/components/ui/button-group";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover.tsx";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu.tsx";
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
import { useAppDispatch } from "@/store/hooks.ts";
import { useDataDisplayCommon } from "@/hooks/use_data_display_common.ts";
import { useAuth } from "@/hooks/use_auth";
import {
    deleteItem,
    list,
    print,
} from "@/components/modules/handover_and_acceptance/lib/slice.ts";
import type { HandoverAndAcceptanceFullList } from "@/components/modules/handover_and_acceptance/lib/interface.ts";
import {
    formatDateLocal,
    openPopup,
    updateWindowWithPdfData,
} from "@/lib/utils.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";

export default function List() {
    const dispatch = useAppDispatch();
    const { claims } = useAuth();
    const [data, setData] = React.useState<HandoverAndAcceptanceFullList>([]);

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
                if (meta) {
                    setPage(meta.page);
                    setLimit(meta.limit);
                    setTotal(meta.total);
                }
                setData(data ?? []);
            },
        });
    }, [dispatch, rawQuery, setLimit, setPage, setTotal]);

    const handleDelete = async (id: string) => {
        const result = await dispatch(deleteItem({ uuid: id })).unwrap();
        handleApiResult(result, { onSuccess: refresh });
    };

    const handlePrint = async (id: string) => {
        const handler = openPopup();
        if (!handler) return;

        dispatch(print({ uuid: id }))
            .unwrap()
            .then((result) => {
                handleApiResult(result, {
                    onSuccess: (pdf) => {
                        if (pdf !== null) updateWindowWithPdfData(handler, pdf);
                        else handler.close();
                    },
                    onError: () => handler.close(),
                });
            })
            .catch(() => handler.close());
    };

    useEffect(() => {
        refresh();
    }, [refresh]);

    return (
        <Card>
            <CardHeader>
                <CardTitle>Átadás-átvételi dokumentumok</CardTitle>
            </CardHeader>
            <CardContent>
                <ButtonGroup orientation="responsive" className="ml-auto mb-5">
                    <Popover>
                        <PopoverTrigger asChild>
                            <Button variant="outline">
                                Szűrő <Funnel />
                            </Button>
                        </PopoverTrigger>
                        <PopoverContent className="w-80">
                            <div className="grid gap-4">
                                <div className="space-y-2">
                                    <h4 className="font-medium">Szűrő</h4>
                                    <p className="text-muted-foreground text-sm">
                                        Szűkítsd a találatok listáját
                                        szűrőfeltételekkel!
                                    </p>
                                </div>
                                <div className="grid grid-cols-3 items-center gap-4">
                                    <Label htmlFor="display_id">
                                        Azonosító
                                    </Label>
                                    <Input
                                        id="display_id"
                                        value={filterValue}
                                        onChange={(e) =>
                                            setFilterValue(e.target.value)
                                        }
                                        onBlur={(e) =>
                                            filterSelect(
                                                "display_id",
                                                e.target.value,
                                            )
                                        }
                                        className="col-span-2 h-8"
                                    />
                                </div>
                            </div>
                        </PopoverContent>
                    </Popover>
                    <Button
                        style={{ color: "green" }}
                        variant="outline"
                        asChild
                    >
                        <Link to="/atadas-atvetel/letrehozas">
                            Új <Plus color="green" />
                        </Link>
                    </Button>
                </ButtonGroup>

                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead />
                            <SortableTableHead
                                field="display_id"
                                orderBy={orderBy}
                                order={order}
                                onOrderSelect={orderSelect}
                            >
                                Azonosító
                            </SortableTableHead>
                            <SortableTableHead
                                field="customer"
                                orderBy={orderBy}
                                order={order}
                                onOrderSelect={orderSelect}
                            >
                                Vevő
                            </SortableTableHead>
                            <SortableTableHead
                                field="item_make"
                                orderBy={orderBy}
                                order={order}
                                onOrderSelect={orderSelect}
                            >
                                Gyártó
                            </SortableTableHead>
                            <SortableTableHead
                                field="item_model"
                                orderBy={orderBy}
                                order={order}
                                onOrderSelect={orderSelect}
                            >
                                Típus
                            </SortableTableHead>
                            <TableHead>Azonosító</TableHead>
                            <SortableTableHead
                                field="received_at"
                                orderBy={orderBy}
                                order={order}
                                onOrderSelect={orderSelect}
                            >
                                Átvétel
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
                                            side="bottom"
                                            align="start"
                                        >
                                            <DropdownMenuLabel>
                                                Műveletek
                                            </DropdownMenuLabel>
                                            <Link
                                                to={`/atadas-atvetel/reszletek/${item.id}`}
                                            >
                                                <DropdownMenuItem>
                                                    <Eye /> Részletek
                                                </DropdownMenuItem>
                                            </Link>
                                            <Link
                                                to={`/atadas-atvetel/modositas/${item.id}`}
                                            >
                                                <DropdownMenuItem>
                                                    <Pencil /> Szerkesztés
                                                </DropdownMenuItem>
                                            </Link>
                                            <DropdownMenuItem
                                                className="cursor-pointer"
                                                onClick={() =>
                                                    handlePrint(item.id)
                                                }
                                            >
                                                <Printer /> Nyomtatás
                                            </DropdownMenuItem>
                                            <DropdownMenuSeparator />
                                            <DropdownMenuItem
                                                className="cursor-pointer"
                                                onClick={() =>
                                                    handleDelete(item.id)
                                                }
                                            >
                                                <Trash /> Törlés
                                            </DropdownMenuItem>
                                        </DropdownMenuContent>
                                    </DropdownMenu>
                                </TableCell>
                                <TableCell>{item.display_id}</TableCell>
                                <TableCell>{item.customer.name}</TableCell>
                                <TableCell>
                                    {item.item_make?.name ?? ""}
                                </TableCell>
                                <TableCell>
                                    {item.item_model?.name ?? ""}
                                </TableCell>
                                <TableCell>
                                    {item.item_identifier_value}
                                </TableCell>
                                <TableCell>
                                    {formatDateLocal(
                                        item.received_at,
                                        claims?.loc,
                                        claims?.tz,
                                    )}
                                </TableCell>
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
    );
}
