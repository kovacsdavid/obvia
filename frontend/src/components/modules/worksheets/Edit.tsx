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
} from "@/components/modules/worksheets/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useSelectList } from "@/hooks/use_select_list.ts";
import { useNavigate } from "react-router-dom";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { Plus } from "lucide-react";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import CustomersEdit from "@/components/modules/customers/Edit.tsx";
import type { Worksheet } from "./lib/interface";
import type { Customer } from "@/components/modules/customers/lib/interface.ts";
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (worksheet: Worksheet) => void;
  onCancel?: () => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
  onCancel = undefined,
}: EditProps) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [customerId, setCustomerId] = React.useState("");
  const [status, setStatus] = React.useState("active");
  const [customersList, setCustomersList] = React.useState<SelectOptionList>(
    [],
  );
  const { errors, setErrors, unexpectedError, isInvalidField } = useFormError();
  const [openNewCustomerDialog, setOpenNewCustomerDialog] =
    React.useState(false);
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
        customerId,
        projectId: "",
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
            navigate("/munkalap/lista");
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
    dispatch,
    id,
    name,
    description,
    customerId,
    status,
    onSuccess,
    navigate,
    setErrors,
    unexpectedError,
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
        customerId,
        projectId: "",
        status,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/munkalap/lista");
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
    customerId,
    description,
    dispatch,
    id,
    name,
    navigate,
    setErrors,
    status,
    unexpectedError,
  ]);

  const loadLists = useCallback(async () => {
    return Promise.all([
      dispatch(select_list("customers")).then((response) => {
        if (select_list.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            setListResponse(response.payload, setCustomersList, setErrors);
          } else {
            unexpectedError(response.payload.statusCode);
          }
        } else {
          unexpectedError();
        }
      }),
    ]);
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
                setCustomerId(data.customer_id);
                setStatus(data.status ?? "");
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
  }, [dispatch, id, setErrors, unexpectedError, setListResponse, loadLists]);

  const handleSubmit = async (e: React.SubmitEvent) => {
    e.preventDefault();
    if (typeof id === "string") {
      handleUpdate();
    } else {
      handleCreate();
    }
  };

  const handleEditCustomersSuccess = (customer: Customer) => {
    loadLists().then(() => {
      setTimeout(() => {
        setCustomerId(customer.id);
      }, 0);
      setOpenNewCustomerDialog(false);
    });
  };

  return (
    <>
      <GlobalError error={errors} />
      <Dialog
        open={openNewCustomerDialog}
        onOpenChange={setOpenNewCustomerDialog}
      >
        <DialogContent>
          <DialogTitle>Vevő létrehozása</DialogTitle>
          <CustomersEdit
            showCard={false}
            onSuccess={handleEditCustomersSuccess}
            onCancel={() => setOpenNewCustomerDialog(false)}
          />
        </DialogContent>
      </Dialog>
      <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <FieldSet>
            <FieldLegend>
              {`Munkalap ${id ? "módosítás" : "létrehozás"}`}
            </FieldLegend>
            <FieldGroup>
              <Field data-invalid={isInvalidField(errors, "name")}>
                <FieldLabel htmlFor="name">Név</FieldLabel>
                <Input
                  id="name"
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  aria-invalid={isInvalidField(errors, "name")}
                />
                <FieldError error={errors} field={"name"} />
              </Field>
              <Field data-invalid={isInvalidField(errors, "description")}>
                <FieldLabel htmlFor="description">Leírás</FieldLabel>
                <Input
                  id="description"
                  type="text"
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  aria-invalid={isInvalidField(errors, "description")}
                />
                <FieldError error={errors} field={"description"} />
              </Field>
              <Field data-invalid={isInvalidField(errors, "customer_id")}>
                <div className="flex items-center w-full">
                  <div className="flex flex-1 items-center">
                    <FieldLabel htmlFor="customer_id">Vevő</FieldLabel>
                  </div>
                  <div className="flex items-center">
                    <Button
                      type="button"
                      variant="outline"
                      onClick={() => setOpenNewCustomerDialog(true)}
                    >
                      <Plus />
                    </Button>
                  </div>
                </div>
                <Select
                  value={customerId}
                  onValueChange={(val) => setCustomerId(val)}
                >
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField(errors, "customer_id")}
                  >
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {customersList.map((customer) => {
                      return (
                        <SelectItem key={customer.value} value={customer.value}>
                          {customer.title}
                        </SelectItem>
                      );
                    })}
                  </SelectContent>
                </Select>
                <FieldError error={errors} field={"customer_id"} />
              </Field>
              <Field data-invalid={isInvalidField(errors, "status")}>
                <FieldLabel htmlFor="status">Státusz</FieldLabel>
                <Select value={status} onValueChange={(val) => setStatus(val)}>
                  <SelectTrigger
                    className={"w-full"}
                    aria-invalid={isInvalidField(errors, "status")}
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
