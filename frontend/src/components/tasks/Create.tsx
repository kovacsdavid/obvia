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

import React, {useEffect, useState} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create} from "@/components/tasks/slice.ts";
import { type FormError } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {select_list} from "@/components/tasks/slice.ts";
import {isWorksheetListResponse, type WorksheetList} from "@/components/worksheets/interface.ts";

export default function Create() {
  const [worksheetId, setWorksheetId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [title, setTitle] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [priority, setPriority] = React.useState("normal");
  const [dueDate, setDueDate] = React.useState("");
  const [worksheetList, setWorksheetList] = React.useState<WorksheetList>([]);
  const [errors, setErrors] = useState<FormError | null>(null);
  const dispatch = useAppDispatch();

  const unexpectedError = () => {
    setErrors({
      global: "Váratlan hiba történt a feldolgozás során!",
      fields: {}
    });
  };

  useEffect(() => {
    dispatch(select_list("worksheets")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (
                isWorksheetListResponse(responseData)
                && typeof responseData.data !== "undefined"
              ) {
                setWorksheetList(responseData.data);
              } else {
                unexpectedError();
              }
              break;
            default:
              unexpectedError();
          }
        } catch {
          unexpectedError()
        }
      }
    });
  }, [dispatch]);


  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      worksheetId,
      title,
      description,
      status,
      priority,
      dueDate
    })).then(async (response) => {
      console.log(response)
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/feladat/lista";
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              unexpectedError()
          }
        } catch {
          unexpectedError()
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4" autoComplete={"off"}>
        <Label htmlFor="worksheet_id">Munkalap ID</Label>
        <Select
          value={worksheetId}
          onValueChange={val => setWorksheetId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {worksheetList.map(worksheet => {
              return <SelectItem key={worksheet.id} value={worksheet.id}>{worksheet.name}</SelectItem>
            })}
          </SelectContent>
        </Select>
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
        <Select
          value={status}
          onValueChange={val => setStatus(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="active">Aktív</SelectItem>
            <SelectItem value="inactive">Inaktív</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"status"}/>
        <Label htmlFor="priority">Prioritás</Label>
        <Select
          value={priority}
          onValueChange={val => setPriority(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="low">Alacsony</SelectItem>
            <SelectItem value="normal">Normál</SelectItem>
            <SelectItem value="high">Magas</SelectItem>
          </SelectContent>
        </Select>
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