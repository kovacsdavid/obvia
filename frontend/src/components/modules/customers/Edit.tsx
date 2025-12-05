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
import {
  create,
  get,
  update,
} from "@/components/modules/customers/lib/slice.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router-dom";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Customer } from "@/components/modules/customers/lib/interface.ts";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (customer: Customer) => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
}: EditProps) {
  const [customerType, setCustomerType] = React.useState<string | undefined>(
    "natural",
  );
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [email, setEmail] = React.useState("");
  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [status, setStatus] = React.useState<string | undefined>("active");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { errors, setErrors, unexpectedError } = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const handleCreate = useCallback(() => {
    dispatch(
      create({
        id,
        name,
        contactName,
        email,
        phoneNumber,
        status,
        customerType,
      }),
    ).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (
            typeof onSuccess === "function" &&
            typeof response.payload.jsonData.data !== "undefined"
          ) {
            onSuccess(response.payload.jsonData.data);
          } else {
            navigate("/vevo/lista");
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    contactName,
    customerType,
    dispatch,
    email,
    id,
    name,
    navigate,
    onSuccess,
    phoneNumber,
    setErrors,
    status,
    unexpectedError,
  ]);

  const handleUpdate = useCallback(() => {
    dispatch(
      update({
        id,
        name,
        contactName,
        email,
        phoneNumber,
        status,
        customerType,
      }),
    ).then(async (response) => {
      if (update.fulfilled.match(response)) {
        if (response.payload.statusCode === 200) {
          navigate("/vevo/lista");
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error);
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  }, [
    contactName,
    customerType,
    dispatch,
    email,
    id,
    name,
    navigate,
    phoneNumber,
    setErrors,
    status,
    unexpectedError,
  ]);

  useEffect(() => {
    if (typeof id === "string") {
      dispatch(get(id)).then(async (response) => {
        if (get.fulfilled.match(response)) {
          if (response.payload.statusCode === 200) {
            if (typeof response.payload.jsonData.data !== "undefined") {
              const data = response.payload.jsonData.data;
              setCustomerType(data.customer_type);
              setName(data.name);
              setContactName(data.contact_name ?? "");
              setEmail(data.email);
              setPhoneNumber(data.phone_number ?? "");
              setStatus(data.status);
            }
          } else if (typeof response.payload.jsonData?.error !== "undefined") {
            setErrors({
              message: response.payload.jsonData.error.message,
              fields: {},
            });
          } else {
            unexpectedError();
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
        title={`Vevő ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="customer_type">Típus</Label>
          <Select
            value={customerType}
            onValueChange={(val) => setCustomerType(val)}
          >
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="natural">Természetes személy</SelectItem>
              <SelectItem value="legal">Jogi személy</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"customer_type"} />
          <Label htmlFor="name">
            {customerType === "legal" ? "Jogi személy neve" : "Név"}
          </Label>
          <Input
            id="name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"} />
          {customerType === "legal" ? (
            <>
              <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
              <Input
                id="contact_name"
                type="text"
                value={contactName}
                onChange={(e) => setContactName(e.target.value)}
              />
              <FieldError error={errors} field={"contact_name"} />
            </>
          ) : null}
          <Label htmlFor="email">
            {customerType === "legal"
              ? "Kapcsolattartó e-mail címe"
              : "E-mail cím"}
          </Label>
          <Input
            id="email"
            type="text"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
          <FieldError error={errors} field={"email"} />
          <Label htmlFor="phone_number">
            {customerType === "legal"
              ? "Kapcsolattartó telefonszáma"
              : "Telefonszám"}
          </Label>
          <Input
            id="phone_number"
            type="text"
            value={phoneNumber}
            onChange={(e) => setPhoneNumber(e.target.value)}
          />
          <FieldError error={errors} field={"phone_number"} />

          <Label htmlFor="status">Státusz</Label>
          <Select value={status} onValueChange={(val) => setStatus(val)}>
            <SelectTrigger className={"w-full"}>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="active">Aktív</SelectItem>
              <SelectItem value="lead">Érdeklődő</SelectItem>
              <SelectItem value="prospect">Lehetséges vevő</SelectItem>
            </SelectContent>
          </Select>
          <FieldError error={errors} field={"status"} />
          <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
        </form>
      </ConditionalCard>
    </>
  );
}
