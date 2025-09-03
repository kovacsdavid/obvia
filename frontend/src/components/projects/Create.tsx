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
import {useAppDispatch} from "@/store/hooks.ts";
import {create} from "@/store/slices/projects.ts";

export default function Create() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [startDate, setStartDate] = React.useState("");
  const [endDate, setEndDate] = React.useState("");
  const [status, setStatus] = React.useState("");
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      description,
      startDate,
      endDate,
      status,
    })).then((response) => {
      console.log(response)
    });
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
      <Label htmlFor="status">Státusz</Label>
      <Input
        id="status"
        type="text"
        value={startDate}
        onChange={e => setStartDate(e.target.value)}
      />
      <Label htmlFor="start_date">Kezdő dátum</Label>
      <Input
        id="start_date"
        type="text"
        value={endDate}
        onChange={e => setEndDate(e.target.value)}
      />
      <Label htmlFor="end_date">Határidő</Label>
      <Input
        id="end_date"
        type="text"
        value={endDate}
        onChange={e => setEndDate(e.target.value)}
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