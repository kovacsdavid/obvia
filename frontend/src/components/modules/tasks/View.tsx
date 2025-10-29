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

import {useParams} from "react-router";
import React, {useEffect} from "react";
import {useAppDispatch} from "@/store/hooks.ts";
import {get_resolved} from "@/components/modules/tasks/lib/slice.ts";
import type {SimpleError} from "@/lib/interfaces/common.ts";
import {Table, TableBody, TableCell, TableRow} from "@/components/ui/table.tsx";
import {Card, CardContent, CardHeader, CardTitle,} from "@/components/ui/card.tsx"
import {GlobalError} from "@/components/ui";
import {formatDateToYMDHMS} from "@/lib/utils.ts";
import type {TaskResolved} from "@/components/modules/tasks/lib/interface.ts";


export default function View() {
  const [data, setData] = React.useState<TaskResolved | null>(null);
  const [errors, setErrors] = React.useState<SimpleError | null>(null);
  const dispatch = useAppDispatch();
  const params = useParams();

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
            setErrors(response.payload.jsonData.error)
          } else {
            unexpectedError();
          }
        } else {
          unexpectedError();
        }
      })
    }
  }, [dispatch, params]);

  return (
    <>
      <GlobalError error={errors}/>
      {data !== null ? (
        <>
          <Card className={"max-w-lg mx-auto"}>
            <CardHeader>
              <CardTitle>Feladat</CardTitle>
            </CardHeader>
            <CardContent>
              <Table>
                <TableBody>
                  <TableRow>
                    <TableCell>
                      Azonosító
                    </TableCell>
                    <TableCell>
                      {data.id}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Munkalap
                    </TableCell>
                    <TableCell>
                      {data.worksheet} ({data.worksheet_id})
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Szolgáltatás
                    </TableCell>
                    <TableCell>
                      {data.service} ({data.service_id})
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Leírás
                    </TableCell>
                    <TableCell>
                      {data.description ?? ''}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Pénznem
                    </TableCell>
                    <TableCell>
                      {data.currency_code}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Munkaóra
                    </TableCell>
                    <TableCell>
                      {data.quantity ?? ''}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Ár
                    </TableCell>
                    <TableCell>
                      {data.price ?? ''}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Adó
                    </TableCell>
                    <TableCell>
                      {data.tax} ({data.tax_id})
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Státusz
                    </TableCell>
                    <TableCell>
                      {data.status}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Prioritás
                    </TableCell>
                    <TableCell>
                      {data.priority ?? ''}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Határidő
                    </TableCell>
                    <TableCell>
                      {formatDateToYMDHMS(data.due_date ?? '')}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Létrehozta
                    </TableCell>
                    <TableCell>
                      {data.created_by} ({data.created_by_id})
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Létrehozva
                    </TableCell>
                    <TableCell>
                      {formatDateToYMDHMS(data.created_at)}
                    </TableCell>
                  </TableRow>
                  <TableRow>
                    <TableCell>
                      Frissítve
                    </TableCell>
                    <TableCell>
                      {formatDateToYMDHMS(data.updated_at)}
                    </TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </CardContent>
          </Card>
        </>
      ) : null}
    </>
  )
}
