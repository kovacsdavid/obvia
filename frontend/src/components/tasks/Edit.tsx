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

import React, {useCallback, useEffect} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create, get, select_list, update} from "@/components/tasks/slice.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select";
import {useNavigate} from "react-router-dom";
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useFormError} from "@/hooks/use_form_error.ts";
import type {SelectOptionList} from "@/lib/interfaces/common.ts";
import {useParams} from "react-router";
import {Card, CardContent, CardHeader, CardTitle} from "@/components/ui/card";
import {formatDateToYMDHMS} from "@/lib/utils.ts";

export default function Edit() {
  const [worksheetId, setWorksheetId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [title, setTitle] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [priority, setPriority] = React.useState("normal");
  const [dueDate, setDueDate] = React.useState("");
  const [worksheetList, setWorksheetList] = React.useState<SelectOptionList>([]);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      worksheetId,
      title,
      description,
      status,
      priority,
      dueDate
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/feladat/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, dispatch, dueDate, id, navigate, priority, setErrors, status, title, unexpectedError, worksheetId]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      worksheetId,
      title,
      description,
      status,
      priority,
      dueDate
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/feladat/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, dispatch, dueDate, id, navigate, priority, setErrors, status, title, unexpectedError, worksheetId]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setWorksheetId(data.worksheet_id);
              setTitle(data.title);
              setDescription(data.description ?? "");
              setPriority(data.priority ?? "");
              setDueDate(formatDateToYMDHMS(data.due_date ?? ""));
            }
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors({message: response.payload.jsonData.error.message, fields: {}})
          } else {
            unexpectedError();
          }
        } else {
          unexpectedError();
        }
      });
    }
  }, [dispatch, id, setErrors, unexpectedError]);

  useEffect(() => {
    dispatch(select_list("worksheets")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setWorksheetList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setListResponse, setErrors, unexpectedError]);


  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (typeof id === "string") {
      handleUpdate();
    } else {
      handleCreate();
    }
  };

  return (
    <>
      <GlobalError error={errors}/>
      <Card className={"max-w-lg mx-auto"}>
        <CardHeader>
          <CardTitle>Feladat</CardTitle>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
            <Label htmlFor="worksheet_id">Munkalap ID</Label>
            <Select
              value={worksheetId}
              onValueChange={val => setWorksheetId(val)}
            >
              <SelectTrigger className={"w-full"}>
                <SelectValue/>
              </SelectTrigger>
              <SelectContent>
                {worksheetList.map((worksheet) => {
                  return <SelectItem key={worksheet.value} value={worksheet.value}>{worksheet.title}</SelectItem>
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
        </CardContent>
      </Card>
    </>
  );
}