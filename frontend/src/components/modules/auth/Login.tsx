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

import React, {useState} from "react";
import {useAppDispatch, useAppSelector} from "@/store/hooks.ts";
import type {RootState} from "@/store";
import {Button, GlobalError, Input, Label} from "@/components/ui";
import {useNavigate} from 'react-router-dom'
import {useAuth} from "@/context/AuthContext.tsx";
import {loginUser} from "@/components/modules/auth/lib/slice.ts";
import {type FormError} from "@/lib/interfaces/common.ts";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card.tsx";
import {isLoginResponse} from "@/components/modules/auth/lib/guards.ts";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const status = useAppSelector((state: RootState) => state.auth.login.status);
  const [errors, setErrors] = useState<FormError | null>(null);
  const navigate = useNavigate();
  const {login} = useAuth();
  const dispatch = useAppDispatch();
  const loading = status === "loading";

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    login(email, password).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        // TODO: redirect here only if the user doesn't have any tenants yet!
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200: {
              if (isLoginResponse(responseData)) {
                const claims = responseData?.data?.claims;
                const user = responseData?.data?.user;
                const token = responseData?.data?.token;
                if (
                  typeof user !== "undefined"
                  && typeof token !== "undefined"
                  && typeof claims !== "undefined"
                ) {
                  dispatch(loginUser({token, user, claims}))
                  navigate('/adatbazis/letrehozas');
                } else {
                  setErrors({
                    message: "Váratlan hiba történt a feldolgozás során!",
                    fields: {}
                  });
                }
              }
              break;
            }
            case 401: {
              setEmail("");
              setPassword("");
              const message = responseData?.error?.message;
              if (typeof message !== "undefined") {
                setErrors({
                  message,
                  fields: {}
                });
              } else {
                setErrors({
                  message: "Váratlan hiba történt a feldolgozás során!",
                  fields: {}
                });
              }
            }
          }
        } catch {
          setErrors({
            message: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Bejelentkezés</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
            <Label htmlFor="email">Email</Label>
            <Input
              type="text"
              autoComplete="email"
              value={email}
              onFocus={() => setErrors(null)}
              onChange={e => setEmail(e.target.value)}
            />
            <Label htmlFor="password">Jelszó</Label>
            <Input
              type="password"
              autoComplete="current-password"
              onFocus={() => setErrors(null)}
              value={password}
              onChange={e => setPassword(e.target.value)}
            />
            <Button type="submit" disabled={loading}>
              {loading ? "Bejelentkezés..." : "Bejelentkezés"}
            </Button>
          </form>
        </CardContent>
      </Card>
    </>
  );
}
