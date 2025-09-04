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
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create} from "@/store/slices/warehouses.ts";
import { type Errors } from "@/lib/interfaces.ts";

export default function List() {
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [contactPhone, setContactPhone] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [errors, setErrors] = useState<Errors | null>(null);
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      contactName,
      contactPhone,
      status,
    })).then((response) => {
      setErrors({global: "Not implemented yet!"});
      console.log(response)
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="name">Név</Label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <FieldError error={errors} field={"name"}/>
        <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
        <Input
          id="contact_name"
          type="text"
          value={contactName}
          onChange={e => setContactName(e.target.value)}
        />
        <FieldError error={errors} field={"contact_name"}/>
        <Label htmlFor="contact_phone">Kapcsolattartó telefonszáma</Label>
        <Input
          id="contact_phone"
          type="text"
          value={contactPhone}
          onChange={e => setContactPhone(e.target.value)}
        />
        <FieldError error={errors} field={"contact_phone"}/>
        <Label htmlFor="status">Státusz</Label>
        <Input
          id="status"
          type="text"
          value={status}
          onChange={e => setStatus(e.target.value)}
        />
        <FieldError error={errors} field={"status"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}