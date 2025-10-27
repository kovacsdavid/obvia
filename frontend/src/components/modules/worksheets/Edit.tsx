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
import {create, get, select_list, update} from "@/components/modules/worksheets/lib/slice.ts";
import {type SelectOptionList} from "@/lib/interfaces/common.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue,} from "@/components/ui/select.tsx";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useSelectList} from "@/hooks/use_select_list.ts";
import {useNavigate} from "react-router-dom";
import {useParams} from "react-router";
import {ConditionalCard} from "@/components/ui/card.tsx";
import {Plus} from "lucide-react";
import {Dialog, DialogContent, DialogTitle} from "@/components/ui/dialog.tsx";
import ProjectsEdit from "@/components/modules/projects/Edit.tsx";
import type {Project} from "@/components/modules/projects/lib/interface.ts";
import type {Worksheet} from "./lib/interface";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (worksheet: Worksheet) => void;
}

export default function Edit({showCard = true, onSuccess = undefined}: EditProps) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [projectId, setProjectId] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [projectsList, setProjectsList] = React.useState<SelectOptionList>([]);
  const {errors, setErrors, unexpectedError} = useFormError();
  const [openNewProjectDialog, setOpenNewProjectDialog] = React.useState(false);
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {setListResponse} = useSelectList();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(create({
      id,
      name,
      description,
      projectId,
      status,
    })).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function"
            && typeof response.payload.jsonData.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/munkalap/lista");
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, onSuccess, dispatch, id, name, navigate, projectId, setErrors, status, unexpectedError]);

  const handleUpdate = useCallback(() => {
    dispatch(update({
      id,
      name,
      description,
      projectId,
      status,
    })).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
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
  }, [description, dispatch, id, name, navigate, projectId, setErrors, status, unexpectedError]);

  const loadLists = useCallback(async () => {
    return dispatch(select_list("projects")).then((response) => {
      if (select_list.fulfilled.match(response)) {
        setListResponse(response.payload, setProjectsList, setErrors);
      } else {
        unexpectedError();
      }
    });
  }, [dispatch, setErrors, setListResponse, unexpectedError]);

  useEffect(() => {
    loadLists().then(() => {
      if (typeof id === "string") {
        dispatch(get(id)).then(async (response) => {
          if (get.fulfilled.match(response)) {
            if (response.payload.statusCode === 200) {
              if (typeof response.payload.jsonData.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setName(data.name);
                setDescription(data.description ?? "");
                setProjectId(data.project_id);
                setStatus(data.status ?? "");
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
    });
  }, [dispatch, id, setErrors, unexpectedError, setListResponse, loadLists]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (typeof id === "string") {
      handleUpdate();
    } else {
      handleCreate();
    }
  };

  const handleEditProjectsSuccess = (project: Project) => {
    loadLists().then(() => {
      setTimeout(() => {
        setProjectId(project.id);
      }, 0);
      setOpenNewProjectDialog(false);
    })
  };

  return (
    <>
      <GlobalError error={errors}/>
      <Dialog open={openNewProjectDialog} onOpenChange={setOpenNewProjectDialog}>
        <DialogContent>
          <DialogTitle>Új projekt létrehozása</DialogTitle>
          <ProjectsEdit showCard={false} onSuccess={handleEditProjectsSuccess}/>
        </DialogContent>
      </Dialog>
      <ConditionalCard
        showCard={showCard}
        title={`Feladat ${id ? "létrehozás" : "módosítás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form onSubmit={handleSubmit} className="space-y-4" autoComplete={"off"}>
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
          <Button type="button" variant="outline" onClick={() => setOpenNewProjectDialog(true)}>
            <Plus/> Új projekt
          </Button>
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
          <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
        </form>
      </ConditionalCard>
    </>
  );
}
