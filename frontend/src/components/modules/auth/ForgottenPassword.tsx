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

import React, { useState } from "react";
import {
  Button,
  FieldError,
  GlobalError,
  GlobalSuccess,
  GlobalNotification,
  Input,
  Label,
} from "@/components/ui";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { useAppDispatch } from "@/store/hooks";
import {
  forgottenPassword,
  newPassword,
} from "@/components/modules/auth/lib/slice.ts";
import { useFormError } from "@/hooks/use_form_error";
import { useParams } from "react-router";

export default function ForgottenPassword() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const { errors, setErrors, unexpectedError } = useFormError();
  const [success, setSuccess] = useState<boolean | null>(null);
  const dispatch = useAppDispatch();
  const params = useParams();

  const handleSubmitNewPassword = async (e: React.FormEvent) => {
    e.preventDefault();
    const token = params["id"];
    if (typeof token === "string" && token.length === 36) {
      const response = await dispatch(
        newPassword({
          token,
          password,
          password_confirm: passwordConfirm,
        }),
      );
      if (newPassword.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          setSuccess(true);
          setErrors(null);
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setSuccess(false);
          setErrors(response.payload.jsonData.error);
        } else {
          setSuccess(false);
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    }
  };

  const handleSubmitForgottenPassword = async (e: React.FormEvent) => {
    e.preventDefault();

    const response = await dispatch(
      forgottenPassword({
        email,
      }),
    );
    if (forgottenPassword.fulfilled.match(response)) {
      if (response.payload.statusCode === 200) {
        setSuccess(true);
        setErrors(null);
      } else if (typeof response.payload.jsonData?.error !== "undefined") {
        setSuccess(false);
        setErrors(response.payload.jsonData.error);
      } else {
        setSuccess(false);
        unexpectedError();
      }
    } else {
      unexpectedError();
    }
  };

  if (typeof params["id"] === "string" && params["id"].length === 36) {
    return (
      <>
        {success ? (
          <GlobalSuccess
            success={{
              message: "A jelszó megváltoztatása sikeresen megtörtént.",
            }}
          />
        ) : (
          <GlobalError error={errors} />
        )}
        <Card className={"max-w-lg mx-auto"}>
          <CardHeader>
            <CardTitle>Add meg az új jelszót!</CardTitle>
          </CardHeader>
          <CardContent>
            <GlobalNotification message="Ezt az űrlapot csak akkor töltsd ki, ha te kértél jelszóemlékeztetőt!" />
            <form
              onSubmit={handleSubmitNewPassword}
              className="space-y-4"
              autoComplete={"off"}
            >
              <Label htmlFor="password">Jelszó</Label>
              <Input
                id="password"
                type="password"
                autoComplete="new-password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
              <FieldError error={errors} field={"password"} />
              <Label htmlFor="password_confirm">Jelszó megerősítése</Label>
              <Input
                id="password_confirm"
                type="password"
                autoComplete="new-password"
                value={passwordConfirm}
                onChange={(e) => setPasswordConfirm(e.target.value)}
              />
              <FieldError error={errors} field={"password_confirm"} />
              <Button type="submit">Jelszó megváltoztatása</Button>
            </form>
          </CardContent>
        </Card>
      </>
    );
  } else {
    return (
      <>
        {success ? (
          <GlobalSuccess
            success={{
              message:
                "A jelszóemlékeztető e-mail kiküldése sikeresen megtörtént",
            }}
          />
        ) : (
          <GlobalError error={errors} />
        )}
        <Card className={"max-w-lg mx-auto"}>
          <CardHeader>
            <CardTitle>Elfelejtett jelszó</CardTitle>
          </CardHeader>
          <CardContent>
            <GlobalNotification message="A megadott e-mail címre kapsz majd egy hivatkozást, aminek a segítségével megváltoztathatod a jelenlegi jelszavadat." />
            <form
              onSubmit={handleSubmitForgottenPassword}
              className="space-y-4"
              autoComplete={"off"}
            >
              <Label htmlFor="email">E-mail</Label>
              <Input
                id="email"
                type="text"
                autoComplete="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
              />
              <FieldError error={errors} field={"email"} />
              <Button type="submit">Küldés</Button>
            </form>
          </CardContent>
        </Card>
      </>
    );
  }
}
