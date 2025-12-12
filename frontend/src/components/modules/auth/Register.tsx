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

import React, { useState, useEffect } from "react";
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
import { registerUserRequest } from "@/components/modules/auth/lib/slice.ts";
import { useAppDispatch } from "@/store/hooks.ts";
import { useNavigate } from "react-router-dom";
import { type ProcessedResponse } from "@/lib/interfaces/common.ts";
import { type RegisterResponse } from "@/components/modules/auth/lib/interface.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { useAuth } from "@/context/AuthContext.tsx";

export default function Register() {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { errors, setErrors, unexpectedError } = useFormError();
  const { isLoggedIn } = useAuth();

  const handleRegistrationResponse = async (
    response: ProcessedResponse<RegisterResponse>,
  ) => {
    if (response.statusCode === 201) {
      navigate("/bejelentkezes");
    } else if (typeof response.jsonData?.error !== "undefined") {
      setErrors(response.jsonData.error);
    } else {
      unexpectedError();
    }
  };

  useEffect(() => {
    if (isLoggedIn) {
      navigate("/adatbazis/letrehozas");
    }
  }, [isLoggedIn, navigate]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const response = await dispatch(
      registerUserRequest({
        firstName,
        lastName,
        email,
        password,
        passwordConfirm,
      }),
    );
    if (registerUserRequest.fulfilled.match(response)) {
      await handleRegistrationResponse(response.payload);
    } else {
      unexpectedError();
    }
  };

  return (
    <>
      <GlobalError error={errors} />
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Regisztráció</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={handleSubmit}
            className="space-y-4"
            autoComplete={"off"}
          >
            <Label htmlFor="last_name">Vezetéknév</Label>
            <Input
              id="last_name"
              type="text"
              value={lastName}
              onChange={(e) => setLastName(e.target.value)}
            />
            <FieldError error={errors} field={"last_name"} />
            <Label htmlFor="first_name">Keresztnév</Label>
            <Input
              id="first_name"
              type="text"
              value={firstName}
              onChange={(e) => setFirstName(e.target.value)}
            />
            <FieldError error={errors} field={"first_name"} />
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="text"
              autoComplete="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
            <FieldError error={errors} field={"email"} />
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
            <Button type="submit">Regisztráció</Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
