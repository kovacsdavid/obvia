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
import {create} from "@/store/slices/tasks.ts";
import { type ErrorContainerWithFields } from "@/lib/interfaces.ts";

export default function Create() {
  const [worksheetId, setWorksheetId] = React.useState("");
  const [title, setTitle] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [status, setStatus] = React.useState("");
  const [priority, setPriority] = React.useState("");
  const [dueDate, setDueDate] = React.useState("");
  const [errors, setErrors] = useState<ErrorContainerWithFields | null>(null);
  const dispatch = useAppDispatch();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      worksheetId,
      title,
      description,
      status,
      priority,
      dueDate
    })).then((response) => {
      setErrors({global: "Not implemented yet!"});
      console.log(response)
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="worksheet_id">Munkalap ID</Label>
        <Input
          id="worksheet_id"
          type="text"
          value={worksheetId}
          onChange={e => setWorksheetId(e.target.value)}
        />
        <FieldError error={errors} field={"worksheet_id"}/>
        <Label htmlFor="title">Megnevezés</Label>
        <Input
          id="title"
          type="text"
          value={title}
          onChange={e => setTitle(e.target.value)}
        />
        <FieldError error={errors} field={"title"}/>
        <Label htmlFor="description">Leírás</Label>
        <Input
          id="description"
          type="text"
          value={description}
          onChange={e => setDescription(e.target.value)}
        />
        <FieldError error={errors} field={"description"}/>
        <Label htmlFor="status">Státusz</Label>
        <Input
          id="status"
          type="text"
          value={status}
          onChange={e => setStatus(e.target.value)}
        />
        <FieldError error={errors} field={"status"}/>
        <Label htmlFor="priority">Prioritás</Label>
        <Input
          id="priority"
          type="text"
          value={priority}
          onChange={e => setPriority(e.target.value)}
        />
        <FieldError error={errors} field={"priority"}/>
        <Label htmlFor="due_date">Határidő</Label>
        <Input
          id="due_date"
          type="text"
          value={dueDate}
          onChange={e => setDueDate(e.target.value)}
        />
        <FieldError error={errors} field={"due_date"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}