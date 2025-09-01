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

export default function List() {
  const handleSubmit = async (e: React.FormEvent) => {
    console.log(e);
    throw Error("not implemented yet!");
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
      <Label htmlFor="email">E-mail cím</Label>
      <Input
        id="email"
        type="text"
      />
      <Label htmlFor="last_name">Vezetéknév</Label>
      <Input
        id="last_name"
        type="text"
      />
      <Label htmlFor="fist_name">Keresztnév</Label>
      <Input
        id="fist_name"
        type="text"
      />
      <Label htmlFor="phone">Telefonszám</Label>
      <Input
        id="phone"
        type="text"
      />
      <Label htmlFor="status">Státusz</Label>
      <Input
        id="status"
        type="text"
      />
      <Button type="submit">Létrehozás</Button>
    </form>
  );
}