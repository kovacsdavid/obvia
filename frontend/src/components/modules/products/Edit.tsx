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
import { Button, FieldError, GlobalError, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
  create,
  get,
  select_list,
  update,
} from "@/components/modules/products/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router-dom";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Product } from "./lib/interface";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (products: Product) => void;
  onCancel?: () => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  onCancel = undefined,
}: EditProps) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [unitOfMeasureId, setUnitOfMeasureId] = React.useState(
    "239b22ad-5db9-4c9c-851b-ba76885c2dae",
  );
  const [newUnitOfMeasure, setNewUnitOfMeasure] = React.useState("");
  const [unitsOfMeasureList, setUnitsOfMeasureList] =
    React.useState<SelectOptionList>([]);
  const [status, setStatus] = React.useState("active");
  const { errors, setErrors, unexpectedError, isInvalidField, resetError } =
    useFormError();
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { setListResponse } = useSelectList();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(
      create({
        id,
        name,
        description,
        unitOfMeasureId,
        newUnitOfMeasure,
        status,
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
            navigate("/termek/lista");
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
    newUnitOfMeasure,
    onSuccess,
    setErrors,
    status,
    unexpectedError,
    unitOfMeasureId,
  ]);

  const handleCancel = useCallback(
    (e: React.MouseEvent) => {
      e.preventDefault();
      if (typeof onCancel === "function") {
        onCancel();
      } else {
        navigate(-1);
      }
    },
    [navigate, onCancel],
  );

  const handleUpdate = useCallback(() => {
    dispatch(
      update({
        id,
        name,
        description,
        unitOfMeasureId,
        newUnitOfMeasure,
        status,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/termek/lista");
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
    newUnitOfMeasure,
    setErrors,
    status,
    unexpectedError,
    unitOfMeasureId,
  ]);

  const loadLists = useCallback(async () => {
    return dispatch(select_list("units_of_measure")).then((response) => {
      if (select_list.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          setListResponse(response.payload, setUnitsOfMeasureList, setErrors);
        } else {
          unexpectedError(response.payload.statusCode);
        }
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
              if (typeof response.payload.jsonData?.data !== "undefined") {
                const data = response.payload.jsonData.data;
                setName(data.name);
                setDescription(data.description ?? "");
                setUnitOfMeasureId(data.unit_of_measure_id);
                setStatus(data.status);
              }
            } else if (
              typeof response.payload.jsonData?.error !== "undefined"
            ) {
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
    });
  }, [dispatch, id, setErrors, unexpectedError, loadLists]);

  const handleSubmit = async (e: React.SubmitEvent) => {
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
      <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <FieldSet>
            <FieldLegend>
              {`Termék ${id ? "módosítás" : "létrehozás"}`}
            </FieldLegend>
            <FieldGroup>
              <Field data-invalid={isInvalidField("name")}>
                <FieldLabel htmlFor="name">Név</FieldLabel>
                <Input
                  id="name"
                  type="text"
                  value={name}
                  onChange={(e) => {
                    resetError("name");
                    setName(e.target.value);
                  }}
                  aria-invalid={isInvalidField("name")}
                />
                <FieldError error={errors} field={"name"} />
              </Field>
              <Field data-invalid={isInvalidField("description")}>
                <FieldLabel htmlFor="description">Leírás</FieldLabel>
                <Input
                  id="description"
                  type="text"
                  value={description}
                  onChange={(e) => {
                    resetError("description");
                    setDescription(e.target.value);
                  }}
                  aria-invalid={isInvalidField("description")}
                />
                <FieldError error={errors} field={"description"} />
              </Field>
              <Field data-invalid={isInvalidField("unit_of_measure_id")}>
                <FieldLabel htmlFor="unit_of_measure_id">
                  Mértékegység
                </FieldLabel>
                <Select
                  value={unitOfMeasureId}
                  onValueChange={(val) => {
                    resetError("unit_of_measure_id");
                    setUnitOfMeasureId(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("unit_of_measure_id")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {unitsOfMeasureList.map((unit_of_measure) => {
                      return (
                        <SelectItem
                          key={unit_of_measure.value}
                          value={unit_of_measure.value}
                        >
                          {unit_of_measure.title}
                        </SelectItem>
                      );
                    })}
                    <SelectItem value="other">Egyéb</SelectItem>
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"unit_of_measure_id"} />
              </Field>

              {unitOfMeasureId === "other" ? (
                <>
                  <Field data-invalid={isInvalidField("new_unit_of_measure")}>
                    <FieldLabel htmlFor="new_unit_of_measure">
                      Egyéb mértékegység
                    </FieldLabel>
                    <Input
                      id="new_unit_of_measure"
                      type="text"
                      value={newUnitOfMeasure}
                      onChange={(e) => {
                        resetError("new_unit_of_measure");
                        setNewUnitOfMeasure(e.target.value);
                      }}
                      aria-invalid={isInvalidField("new_unit_of_measure")}
                    />
                    <FieldError error={errors} field={"new_unit_of_measure"} />
                  </Field>
                </>
              ) : null}
              <Field data-invalid={isInvalidField("status")}>
                <FieldLabel htmlFor="status">Státusz</FieldLabel>
                <Select
                  value={status}
                  onValueChange={(val) => {
                    resetError("status");
                    setStatus(val);
                  }}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField("status")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="active">Aktív</SelectItem>
                    <SelectItem value="inactive">Inaktív</SelectItem>
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"status"} />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <div className="text-right mt-8 w-full">
              <Button className="mr-3" variant="outline" onClick={handleCancel}>
                Mégse
              </Button>
              <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
            </div>
          </Field>
        </form>
      </ConditionalCard>
    </>
  );
}
