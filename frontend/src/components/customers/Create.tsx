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

import React from "react";
import {Button, Input, Label} from "@/components/ui";

export default function Create() {
  const [type, setType] = React.useState("");
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [email, setEmail] = React.useState("");
  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [status, setStatus] = React.useState("");
  const handleSubmit = async (e: React.FormEvent) => {
    console.log(e);
    throw Error("not implemented yet!");
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
      <Label htmlFor="type">Típus</Label>
      <Input
        id="type"
        type="text"
        value={type}
        onChange={e => setType(e.target.value)}
      />
      <Label htmlFor="name">Név</Label>
      <Input
        id="name"
        type="text"
        value={name}
        onChange={e => setName(e.target.value)}
      />
      <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
      <Input
        id="contact_name"
        type="text"
        value={contactName}
        onChange={e => setContactName(e.target.value)}
      />
      <Label htmlFor="email">E-mail cím</Label>
      <Input
        id="email"
        type="text"
        value={email}
        onChange={e => setEmail(e.target.value)}
      />
      <Label htmlFor="phone_number">Telefonszám</Label>
      <Input
        id="phone_number"
        type="text"
        value={phoneNumber}
        onChange={e => setPhoneNumber(e.target.value)}
      />
      <Label htmlFor="status">Státusz</Label>
      <Input
        id="status"
        type="text"
        value={status}
        onChange={e => setStatus(e.target.value)}
      />
      <Button type="submit">Létrehozás</Button>
    </form>
  );
}