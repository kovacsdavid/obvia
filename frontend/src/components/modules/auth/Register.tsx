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
import { Button, FieldError, GlobalError, Input } from "@/components/ui";
import { registerUserRequest } from "@/components/modules/auth/lib/slice.ts";
import { useAppDispatch } from "@/store/hooks.ts";
import { useNavigate } from "react-router-dom";
import { type ProcessedResponse } from "@/lib/interfaces/common.ts";
import { type RegisterResponse } from "@/components/modules/auth/lib/interface.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import { Card, CardContent } from "@/components/ui/card.tsx";
import { useAuth } from "@/hooks/use_auth";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

export default function Register() {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { errors, setErrors, unexpectedError, isInvalidField } = useFormError();
  const { isLoggedIn } = useAuth();

  const handleRegistrationResponse = async (
    response: ProcessedResponse<RegisterResponse>,
  ) => {
    if (response.statusCode === 201) {
      navigate("/bejelentkezes");
    } else if (typeof response.jsonData?.error !== "undefined") {
      setErrors(response.jsonData.error);
    } else {
      unexpectedError(response.statusCode);
    }
  };

  useEffect(() => {
    if (isLoggedIn) {
      navigate("/adatbazis/letrehozas");
    }
  }, [isLoggedIn, navigate]);

  const handleSubmit = async (e: React.SubmitEvent) => {
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
        <CardContent>
          <form
            onSubmit={handleSubmit}
            className="space-y-4"
            autoComplete={"off"}
          >
            <FieldSet>
              <FieldLegend>Regisztráció</FieldLegend>
              <FieldGroup>
                <Field data-invalid={isInvalidField(errors, "last_name")}>
                  <FieldLabel htmlFor="last_name">Vezetéknév</FieldLabel>
                  <Input
                    id="last_name"
                    type="text"
                    value={lastName}
                    onChange={(e) => setLastName(e.target.value)}
                    aria-invalid={isInvalidField(errors, "last_name")}
                  />
                  <FieldError error={errors} field={"last_name"} />
                </Field>
                <Field data-invalid={isInvalidField(errors, "first_name")}>
                  <FieldLabel htmlFor="first_name">Keresztnév</FieldLabel>
                  <Input
                    id="first_name"
                    type="text"
                    value={firstName}
                    onChange={(e) => setFirstName(e.target.value)}
                    aria-invalid={isInvalidField(errors, "first_name")}
                  />
                  <FieldError error={errors} field={"first_name"} />
                </Field>
                <Field data-invalid={isInvalidField(errors, "email")}>
                  <FieldLabel htmlFor="email">Email</FieldLabel>
                  <Input
                    id="email"
                    type="text"
                    autoComplete="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    aria-invalid={isInvalidField(errors, "email")}
                  />
                  <FieldError error={errors} field={"email"} />
                </Field>
                <Field data-invalid={isInvalidField(errors, "password")}>
                  <FieldLabel htmlFor="password">Jelszó</FieldLabel>
                  <Input
                    id="password"
                    type="password"
                    autoComplete="new-password"
                    value={password}
                    onChange={(e) => setPassword(e.target.value)}
                    aria-invalid={isInvalidField(errors, "password")}
                  />
                  <FieldError error={errors} field={"password"} />
                </Field>
                <Field
                  data-invalid={isInvalidField(errors, "password_confirm")}
                >
                  <FieldLabel htmlFor="password_confirm">
                    Jelszó megerősítése
                  </FieldLabel>
                  <Input
                    id="password_confirm"
                    type="password"
                    autoComplete="new-password"
                    value={passwordConfirm}
                    onChange={(e) => setPasswordConfirm(e.target.value)}
                    aria-invalid={isInvalidField(errors, "password_confirm")}
                  />
                  <FieldError error={errors} field={"password_confirm"} />
                </Field>
              </FieldGroup>
            </FieldSet>
            <Field orientation="horizontal">
              <div className="text-right mt-8 w-full">
                <Button type="submit">Regisztráció</Button>
              </div>
            </Field>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
