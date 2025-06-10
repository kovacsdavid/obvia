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
  Input,
  Label,
} from "@/components/ui";
import { registerUser } from "@/store/slices/auth";
import { useAppDispatch } from "@/store/hooks";
import { useNavigate } from "react-router-dom";
import { useAppSelector } from "@/store/hooks";
import type { RootState } from "@/store";

export default function Register() {
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [passwordConfirm, setPasswordConfirm] = useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const error = useAppSelector((state: RootState) => state.auth.register.error);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    dispatch(registerUser({ firstName, lastName, email, password, passwordConfirm })).then((response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        navigate("/login");
      }
    });
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto mt-20 space-y-4">
      <Label htmlFor="last_name">Vezetéknév</Label>
      <Input
        id="last_name"
        type="text"
        value={lastName}
        onChange={e => setLastName(e.target.value)}
      />
      {error?.fields?.last_name && <div className="text-red-600">{error.fields.last_name}</div>}
      <Label htmlFor="first_name">Keresztnév</Label>
      <Input
        id="first_name"
        type="text"
        value={firstName}
        onChange={e => setFirstName(e.target.value)}
      />
      {error?.fields?.first_name && <div className="text-red-600">{error.fields.first_name}</div>}
      <Label htmlFor="email">Email</Label>
      <Input
        id="email"
        type="text"
        autoComplete="email"
        value={email}
        onChange={e => setEmail(e.target.value)}
      />
      {error?.fields?.email && <div className="text-red-600">{error.fields.email}</div>}
      <Label htmlFor="password">Jelszó</Label>
      <Input
        id="password"
        type="password"
        autoComplete="new-password"
        value={password}
        onChange={e => setPassword(e.target.value)}
      />
      {error?.fields?.password && <div className="text-red-600">{error.fields.password}</div>}
      <Label htmlFor="password_confirm">Jelszó megerősítése</Label>
      <Input
        id="password_confirm"
        type="password"
        autoComplete="new-password"
        value={passwordConfirm}
        onChange={e => setPasswordConfirm(e.target.value)}
      />
      {error?.fields?.password_confirm && <div className="text-red-600">{error.fields.password_confirm}</div>}
      <Button type="submit">Regisztráció</Button>
      {error?.global && <div className="text-red-600">{error.global}</div>}
    </form>
  );
}
