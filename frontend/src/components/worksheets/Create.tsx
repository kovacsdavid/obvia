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
import {create} from "@/store/slices/worksheets.ts";
import { type FormError } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {select_list} from "@/store/slices/worksheets.ts";
import {
  type ProjectsSelectListItem,
  isProjectsSelectsListResponse
} from "@/services/worksheets.ts";

export default function Create() {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [projectId, setProjectId] = React.useState("239b22ad-5db9-4c9c-851b-ba76885c2dae");
  const [status, setStatus] = React.useState("active");
  const [projectsList, setProjectsList] = React.useState<ProjectsSelectListItem[]>([]);
  const [errors, setErrors] = useState<FormError | null>(null);
  const dispatch = useAppDispatch();


  useEffect(() => {
    dispatch(select_list("projects")).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 200:
              if (isProjectsSelectsListResponse(responseData)) {
                setProjectsList(responseData.data);
              } else {
                setErrors({
                  global: "Váratlan hiba történt a feldolgozás során!",
                  fields: {}
                });
              }
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      description,
      projectId,
      status,
    })).then(async (response) => {
      console.log(response)
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/munkalap/lista";
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
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
              return <SelectItem key={project.id} value={project.id}>{project.name}</SelectItem>
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