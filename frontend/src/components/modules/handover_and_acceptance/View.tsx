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

import React, { useEffect } from "react";
import { Link as LinkIcon, Plus } from "lucide-react";
import { NavLink, Link, useNavigate, useParams } from "react-router";
import { Button } from "@/components/ui";
import {
    Card,
    CardContent,
    CardHeader,
    CardTitle,
} from "@/components/ui/card.tsx";
import { ButtonGroup } from "@/components/ui/button-group";
import {
    Table,
    TableBody,
    TableCell,
    TableRow,
} from "@/components/ui/table.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import { useAuth } from "@/hooks/use_auth";
import { get_full } from "@/components/modules/handover_and_acceptance/lib/slice.ts";
import type { HandoverAndAcceptanceFull } from "@/components/modules/handover_and_acceptance/lib/interface.ts";
import { formatDateLocal, formatNumber } from "@/lib/utils.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import ActivityFeed from "@/components/modules/activity_feed/ActivityFeed";

export default function View() {
    const [data, setData] = React.useState<HandoverAndAcceptanceFull | null>(
        null,
    );
    const dispatch = useAppDispatch();
    const params = useParams();
    const navigate = useNavigate();
    const { claims } = useAuth();

    useEffect(() => {
        if (typeof params["id"] === "string") {
            dispatch(get_full({ uuid: params["id"] }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (data) setData(data);
                        },
                    });
                });
        }
    }, [dispatch, params]);

    const acceptedByName = () => {
        if (
            !data ||
            typeof data.accepted_by !== "object" ||
            data.accepted_by === null
        ) {
            return "";
        }
        const user = data.accepted_by as {
            first_name?: string | null;
            last_name?: string | null;
            email?: string;
        };
        const name = [user.last_name, user.first_name]
            .filter(Boolean)
            .join(" ")
            .trim();
        return name || user.email || "";
    };

    if (!data) return null;

    return (
        <>
            <Card className="max-w-3xl mx-auto">
                <CardHeader>
                    <CardTitle>Átadás-átvételi dokumentum</CardTitle>
                </CardHeader>
                <CardContent>
                    <Table>
                        <TableBody>
                            <TableRow>
                                <TableCell>Azonosító</TableCell>
                                <TableCell>{data.display_id}</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Vevő</TableCell>
                                <TableCell>
                                    <NavLink
                                        to={`/vevo/reszletek/${data.customer.id}`}
                                    >
                                        {data.customer.name}{" "}
                                        <LinkIcon
                                            size={15}
                                            className="inline"
                                        />
                                    </NavLink>
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Gyártó</TableCell>
                                <TableCell>
                                    {data.item_make?.name ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Típus</TableCell>
                                <TableCell>
                                    {data.item_model?.name ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Azonosító típusa</TableCell>
                                <TableCell>
                                    {data.item_identifier_type.name}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Azonosító értéke</TableCell>
                                <TableCell>
                                    {data.item_identifier_value}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Bejelentett probléma</TableCell>
                                <TableCell>
                                    {data.reported_issue ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Állapot átvételkor</TableCell>
                                <TableCell>
                                    {data.condition_at_receipt ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Mellékelt tartozékok</TableCell>
                                <TableCell>
                                    {data.included_accessories?.length
                                        ? data.included_accessories.join(", ")
                                        : ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Engedélyezett munka</TableCell>
                                <TableCell>
                                    {data.authorized_work ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Javítási keret</TableCell>
                                <TableCell>
                                    {data.repair_limit !== null
                                        ? `${formatNumber(data.repair_limit)} ${data.currency_code?.code ?? ""}`
                                        : ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Pénznem</TableCell>
                                <TableCell>
                                    {data.currency_code?.code ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Feltétel verzió</TableCell>
                                <TableCell>
                                    {data.terms_version ?? ""}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Átvette</TableCell>
                                <TableCell>{acceptedByName()}</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Átvétel dátuma</TableCell>
                                <TableCell>
                                    {formatDateLocal(
                                        data.received_at,
                                        claims?.loc,
                                        claims?.tz,
                                    )}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Létrehozva</TableCell>
                                <TableCell>
                                    {formatDateLocal(
                                        data.created_at,
                                        claims?.loc,
                                        claims?.tz,
                                    )}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>Frissítve</TableCell>
                                <TableCell>
                                    {formatDateLocal(
                                        data.updated_at,
                                        claims?.loc,
                                        claims?.tz,
                                    )}
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                    <ButtonGroup orientation="responsive" className="mt-5">
                        <Button variant="outline" asChild>
                            <Link to={`/atadas-atvetel/modositas/${data.id}`}>
                                <Plus /> Szerkesztés
                            </Link>
                        </Button>
                    </ButtonGroup>
                    <div className="mt-8">
                        <Button variant="outline" onClick={() => navigate(-1)}>
                            Vissza
                        </Button>
                    </div>
                </CardContent>
            </Card>
            <ActivityFeed
                resourceId={data.id}
                resourceType="handover_and_acceptance"
            />
        </>
    );
}
