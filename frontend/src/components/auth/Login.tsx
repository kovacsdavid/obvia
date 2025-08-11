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

import { useState } from "react";
import { useAppSelector } from "@/store/hooks";
import type  { RootState } from "@/store";
import {
  Button,
  Input,
  Label
} from "@/components/ui";
import { useNavigate } from 'react-router-dom'
import { useAuth } from "@/context/AuthContext";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const status = useAppSelector((state: RootState) => state.auth.login.status);
  const error = useAppSelector((state: RootState) => state.auth.login.error);
  const navigate = useNavigate();
  const { login } = useAuth();

  const loading = status === "loading";

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    login( email, password ).then((response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        // TODO: redirect here only if the user doesn't have any tenants yet!
        navigate('/szervezeti_egyseg/letrehozas');
      }
    });
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto mt-20 space-y-4">
      <Label htmlFor="email">Email</Label>
      <Input
        type="text"
        autoComplete="email"
        value={email}
        onChange={e => setEmail(e.target.value)}
      />
      <Label htmlFor="password">Jelszó</Label>
      <Input
        type="password"
        autoComplete="current-password"
        value={password}
        onChange={e => setPassword(e.target.value)}
      />
      {error?.global && <div className="text-red-600">{error.global}</div>}
      <Button type="submit" disabled={loading}>
        {loading ? "Bejelentkezés..." : "Bejelentkezés"}
      </Button>
    </form>
  );
}
