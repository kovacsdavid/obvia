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

import { useAppDispatch } from "@/store/hooks";
import React, { useEffect } from "react";
import { useParams } from "react-router";
import { verfiy_email } from "@/components/modules/auth/lib/slice";
import { CheckCircle2Icon, CircleEllipsisIcon } from "lucide-react";
import { Alert, AlertTitle, GlobalError } from "@/components/ui/alert";

export default function EmailVerification() {
  const params = useParams();
  const dispatch = useAppDispatch();
  const [success, setSuccess] = React.useState<boolean | null>(null);

  useEffect(() => {
    if (typeof params["id"] === "string") {
      dispatch(verfiy_email(params["id"])).then(async (response) => {
        if (verfiy_email.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setSuccess(true);
          } else {
            setSuccess(false);
          }
        } else {
          setSuccess(false);
        }
      });
    }
  }, [dispatch, params]);

  if (success === true) {
    return (
      <>
        <Alert>
          <CheckCircle2Icon />
          <AlertTitle>Az e-mail cím megerősítése sikeresen megtörtént.</AlertTitle>
        </Alert>
      </>
    )
  } else if (success === false) {
    return (
      <>
        <GlobalError error={{message: "Az e-mail cím megerősítése sikertelen." }} />
      </>
    )
  } else {
    return (
      <>
        <Alert>
          <CircleEllipsisIcon />
          <AlertTitle>Az e-mail cím megerősítése folyamatban...</AlertTitle>
        </Alert>
      </>
    )
  }
}
