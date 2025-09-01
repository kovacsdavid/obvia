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
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [unitOfMeasure, setUnitOfMeasure] = React.useState("");
  const [cost, setCost] = React.useState("");
  const [price, setPrice] = React.useState("");
  const [currency_id, setCurrencyId] = React.useState("");
  const [isActive, setIsActive] = React.useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    console.log(e);
    throw Error("not implemented yet!");
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
      <Label htmlFor="name">Név</Label>
      <Input
        id="name"
        type="text"
        value={name}
        onChange={e => setName(e.target.value)}
      />
      <Label htmlFor="description">Leírás</Label>
      <Input
        id="description"
        type="text"
        value={description}
        onChange={e => setDescription(e.target.value)}
      />
      <Label htmlFor="unit_of_measure">Mértékegység</Label>
      <Input
        id="unit_of_measure"
        type="text"
        value={unitOfMeasure}
        onChange={e => setUnitOfMeasure(e.target.value)}
      />
      <Label htmlFor="cost">Bekerülési költség</Label>
      <Input
        id="cost"
        type="text"
        value={cost}
        onChange={e => setCost(e.target.value)}
      />
      <Label htmlFor="price">Fogyasztói ár</Label>
      <Input
        id="price"
        type="text"
        value={price}
        onChange={e => setPrice(e.target.value)}
      />
      <Label htmlFor="currency_id">Pénznem</Label>
      <Input
        id="currency_id"
        type="text"
        value={currency_id}
        onChange={e => setCurrencyId(e.target.value)}
      />
      <Label htmlFor="is_active">Aktív</Label>
      <Input
        id="is_active"
        type="text"
        value={isActive}
        onChange={e => setIsActive(e.target.value)}
      />
      <Button type="submit">Létrehozás</Button>
    </form>
  );
}