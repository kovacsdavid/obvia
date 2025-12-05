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
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import { create } from "@/components/modules/users/lib/slice.ts";
import { type FormError } from "@/lib/interfaces/common.ts";

export default function Create() {
  const [email, setEmail] = React.useState("");
  const [lastName, setLastName] = React.useState("");
  const [firstName, setFirstName] = React.useState("");
  const [phone, setPhone] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [errors, setErrors] = useState<FormError | null>(null);
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(
      create({
        email,
        lastName,
        firstName,
        phone,
        status,
      }),
    ).then((response) => {
      setErrors({ message: "Not implemented yet!", fields: {} });
      console.log(response);
    });
  };

  return (
    <>
      <GlobalError error={errors} />
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="email">E-mail cím</Label>
        <Input
          id="email"
          type="text"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <FieldError error={errors} field={"email"} />
        <Label htmlFor="last_name">Vezetéknév</Label>
        <Input
          id="last_name"
          type="text"
          value={lastName}
          onChange={(e) => setLastName(e.target.value)}
        />
        <FieldError error={errors} field={"last_name"} />
        <Label htmlFor="fist_name">Keresztnév</Label>
        <Input
          id="fist_name"
          type="text"
          value={firstName}
          onChange={(e) => setFirstName(e.target.value)}
        />
        <FieldError error={errors} field={"fist_name"} />
        <Label htmlFor="phone">Telefonszám</Label>
        <Input
          id="phone"
          type="text"
          value={phone}
          onChange={(e) => setPhone(e.target.value)}
        />
        <FieldError error={errors} field={"phone"} />
        <Label htmlFor="status">Státusz</Label>
        <Input
          id="status"
          type="text"
          value={status}
          onChange={(e) => setStatus(e.target.value)}
        />
        <FieldError error={errors} field={"status"} />
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}
