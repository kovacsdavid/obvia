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

import React, { useCallback, useEffect } from "react";
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import { create, get, update } from "@/components/modules/tags/lib/slice.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useNavigate } from "react-router-dom";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Tag } from "./lib/interface";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (tag: Tag) => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
}: EditProps) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const { errors, setErrors, unexpectedError } = useFormError();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(
      create({
        id,
        name,
        description,
      }),
    ).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function" &&
            typeof response.payload.jsonData?.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/cimke/lista");
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    description,
    dispatch,
    id,
    name,
    navigate,
    onSuccess,
    setErrors,
    unexpectedError,
  ]);

  const handleUpdate = useCallback(() => {
    dispatch(
      update({
        id,
        name,
        description,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/cimke/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError(response.payload.statusCode);
        }
      } else {
        unexpectedError();
      }
    });
  }, [description, dispatch, id, name, navigate, setErrors, unexpectedError]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData?.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setName(data.name);
              setDescription(data.description ?? "");
            }
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors({
              message: response.payload.jsonData.error.message,
              fields: {},
            });
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      });
    }
  }, [dispatch, id, setErrors, unexpectedError]);

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
      <GlobalError error={errors} />
      <ConditionalCard
        showCard={showCard}
        title={`Címke ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="name">Név</Label>
          <Input
            id="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"} />
          <Label htmlFor="description">Leírás</Label>
          <Input
            id="description"
            type="text"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
          />
          <FieldError error={errors} field={"description"} />
          <div className="text-right mt-8">
            <Button
              className="mr-3"
              type="submit"
              variant="outline"
              onClick={() => navigate(-1)}
            >
              Mégse
            </Button>
            <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
          </div>
        </form>
      </ConditionalCard>
    </>
  );
}
