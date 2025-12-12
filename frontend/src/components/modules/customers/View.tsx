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

import { useParams } from "react-router";
import React, { useEffect } from "react";
import { useAppDispatch } from "@/store/hooks.ts";
import { get_resolved } from "@/components/modules/customers/lib/slice.ts";
import { type CustomerResolved } from "@/components/modules/customers/lib/interface.ts";
import type { SimpleError } from "@/lib/interfaces/common.ts";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@/components/ui/table.tsx";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { GlobalError, Button } from "@/components/ui";
import { formatDateToYMDHMS } from "@/lib/utils.ts";
import { useNavigate } from "react-router-dom";

export default function View() {
  const [data, setData] = React.useState<CustomerResolved | null>(null);
  const [errors, setErrors] = React.useState<SimpleError | null>(null);
  const dispatch = useAppDispatch();
  const params = useParams();
  const navigate = useNavigate();

  const unexpectedError = () => {
    setErrors({
      message: "Váratlan hiba történt a feldolgozás során!",
    });
  };

  useEffect(() => {
    if (typeof params["id"] === "string") {
      dispatch(get_resolved(params["id"])).then(async (response) => {
        if (get_resolved.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              setData(response.payload.jsonData.data);
            }
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors(response.payload.jsonData.error);
          } else {
            unexpectedError();
          }
        } else {
          unexpectedError();
        }
      });
    }
  }, [dispatch, params]);

  return (
    <>
      <GlobalError error={errors} />
      {data !== null ? (
        <>
          <Card className={"max-w-3xl mx-auto"}>
            <CardHeader>
              <CardTitle>Vevő</CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableBody>
                  <TableRow>
                    <TableCell>Azonosító</TableCell>
                    <TableCell>{data.id}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Név</TableCell>
                    <TableCell>{data.name}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Kapcsolattartó neve</TableCell>
                    <TableCell>
                      {data.contact_name ? data.contact_name : "N/A"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>E-mail cím</TableCell>
                    <TableCell>{data.email}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Telefonszám</TableCell>
                    <TableCell>
                      {data.phone_number ? data.phone_number : "N/A"}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Státusz</TableCell>
                    <TableCell>{data.status}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Létrehozta</TableCell>
                    <TableCell>{data.created_by}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Létrehozva</TableCell>
                    <TableCell>{formatDateToYMDHMS(data.created_at)}</TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>Frissítve</TableCell>
                    <TableCell>{formatDateToYMDHMS(data.updated_at)}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
              <div className="mt-8">
                <Button
                  className="mr-3"
                  type="submit"
                  variant="outline"
                  onClick={() => navigate(-1)}
                >
                  Vissza
                </Button>
              </div>
            </CardContent>
          </Card>
        </>
      ) : null}
    </>
  );
}
