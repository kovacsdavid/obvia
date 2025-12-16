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
import { useAppDispatch, useAppSelector } from "@/store/hooks.ts";
import type { RootState } from "@/store";
import { Button, GlobalError, Input, Label } from "@/components/ui";
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "@/context/AuthContext.tsx";
import { loginUser } from "@/components/modules/auth/lib/slice.ts";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card.tsx";
import { loginUserRequest } from "@/components/modules/auth/lib/slice.ts";
import { useFormError } from "@/hooks/use_form_error.ts";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const status = useAppSelector((state: RootState) => state.auth.login.status);
  const { errors, setErrors, unexpectedError } = useFormError();
  const navigate = useNavigate();
  const { login, isLoggedIn } = useAuth();
  const dispatch = useAppDispatch();
  const loading = status === "loading";

  useEffect(() => {
    if (isLoggedIn) {
      navigate("/vezerlopult");
    }
  }, [isLoggedIn, navigate]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    login(email, password).then(async (response) => {
      if (loginUserRequest.fulfilled.match(response)) {
        if (
          response.payload.statusCode === 200 &&
          typeof response.payload.jsonData?.data !== "undefined"
        ) {
          dispatch(loginUser({
            token: response.payload.jsonData.data.token,
            user: response.payload.jsonData.data.user,
            claims: response.payload.jsonData.data.claims
          }))
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setPassword("");
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  };

  return (
    <>
      <GlobalError error={errors} />
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Bejelentkezés</CardTitle>
        </CardHeader>
        <CardContent>
          <form
            onSubmit={handleSubmit}
            className="space-y-4"
            autoComplete={"off"}
          >
            <Label htmlFor="email">Email</Label>
            <Input
              type="text"
              autoComplete="email"
              value={email}
              onFocus={() => setErrors(null)}
              onChange={(e) => setEmail(e.target.value)}
            />
            <Label htmlFor="password">Jelszó</Label>
            <Input
              type="password"
              autoComplete="current-password"
              onFocus={() => setErrors(null)}
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
            <Button type="submit" disabled={loading}>
              {loading ? "Bejelentkezés..." : "Bejelentkezés"}
            </Button>
          </form>
        </CardContent>
      </Card>
      <div className="text-center mt-3">
        <Link to="/elfelejtett_jelszo">
          <span className="text-xs">Elfelejtetted a jelszavad?</span>
        </Link>
      </div>
    </>
  );
}
