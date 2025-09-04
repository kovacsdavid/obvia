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
  Button, FieldError, GlobalError,
  Input,
  Label,
} from "@/components/ui";
import { registerUserRequest } from "@/store/slices/auth";
import { useAppDispatch } from "@/store/hooks";
import { useNavigate } from "react-router-dom";
import { type Errors } from "@/lib/interfaces.ts";

export default function Register() {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const [errors, setErrors] = useState<Errors | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    dispatch(registerUserRequest({ firstName, lastName, email, password, passwordConfirm })).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201: {
              navigate("/login");
              break;
            }
            case 422:
              setErrors(responseData.error);
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto mt-20 space-y-4">
        <Label htmlFor="last_name">Vezetéknév</Label>
        <Input
          id="last_name"
          type="text"
          value={lastName}
          onChange={e => setLastName(e.target.value)}
        />
        <FieldError error={errors} field={"last_name"}/>
        <Label htmlFor="first_name">Keresztnév</Label>
        <Input
          id="first_name"
          type="text"
          value={firstName}
          onChange={e => setFirstName(e.target.value)}
        />
        <FieldError error={errors} field={"first_name"}/>
        <Label htmlFor="email">Email</Label>
        <Input
          id="email"
          type="text"
          autoComplete="email"
          value={email}
          onChange={e => setEmail(e.target.value)}
        />
        <FieldError error={errors} field={"email"}/>
        <Label htmlFor="password">Jelszó</Label>
        <Input
          id="password"
          type="password"
          autoComplete="new-password"
          value={password}
          onChange={e => setPassword(e.target.value)}
        />
        <FieldError error={errors} field={"password"}/>
        <Label htmlFor="password_confirm">Jelszó megerősítése</Label>
        <Input
          id="password_confirm"
          type="password"
          autoComplete="new-password"
          value={passwordConfirm}
          onChange={e => setPasswordConfirm(e.target.value)}
        />
        <FieldError error={errors} field={"password_confirm"}/>
        <Button type="submit">Regisztráció</Button>
      </form>
    </>
  );
}
