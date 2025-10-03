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

import React, {useEffect} from "react";
import {Button, FieldError, GlobalError, Input, Label} from "@/components/ui";
import {useAppDispatch} from "@/store/hooks.ts";
import {create, select_list} from "@/components/worksheets/slice.ts";
import {type SelectOptionList} from "@/lib/interfaces/common.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useNavigate} from "react-router-dom";

export default function Create() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [projectId, setProjectId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [status, setStatus] = React.useState("active");
  const [projectsList, setProjectsList] = React.useState<SelectOptionList>([]);
  const {errors, setErrors, unexpectedError} = useFormError();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();


  useEffect(() => {
    dispatch(select_list("projects")).then(async (response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setProjectsList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setListResponse, setErrors, unexpectedError]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      description,
      projectId,
      status,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          navigate("/munkalap/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
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
        <Label htmlFor="description">Leírás</Label>
        <Input
          id="description"
          type="text"
          value={description}
          onChange={e => setDescription(e.target.value)}
        />
        <FieldError error={errors} field={"description"}/>
        <Label htmlFor="project_id">Project ID</Label>
        <Select
          value={projectId}
          onValueChange={val => setProjectId(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            {projectsList.map(project => {
              return <SelectItem key={project.value} value={project.value}>{project.title}</SelectItem>
            })}
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"project_id"}/>
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
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}