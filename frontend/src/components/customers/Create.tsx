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
import {create} from "@/components/customers/slice.ts";
import { type FormError } from "@/lib/interfaces/common.ts";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import {isCreateCustomerResponse} from "@/components/customers/interface.ts";

export default function Create() {
  const [customerType, setCustomerType] = React.useState<string | undefined>("natural");
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [email, setEmail] = React.useState("");
  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [status, setStatus] = React.useState<string| undefined>("active");
  const dispatch = useAppDispatch();
  const [errors, setErrors] = useState<FormError | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    dispatch(create({
      name,
      contactName,
      email,
      phoneNumber,
      status,
      customerType,
    })).then(async (response) => {
      const unexpectedError = () => {
        setErrors({
          message: "Váratlan hiba történt a feldolgozás során!",
          fields: {}
        });
      }
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              if (isCreateCustomerResponse(responseData)) {
                window.location.href = "/vevo/lista";
              } else {
                unexpectedError();
              }
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              unexpectedError();

          }
        } catch {
          unexpectedError();
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>

      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4" autoComplete={"off"}>
        <Label htmlFor="customer_type">Típus</Label>
        <Select
          value={customerType}
          onValueChange={val => setCustomerType(val)}
        >
          <SelectTrigger className={"w-full"}>
            <SelectValue/>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="natural">Természetes személy</SelectItem>
            <SelectItem value="legal">Jogi személy</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"customer_type"}/>
        <Label htmlFor="name">{customerType === "legal" ? "Jogi személy neve" : "Név"}</Label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <FieldError error={errors} field={"name"}/>
        {customerType === "legal" ? (
          <>
            <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
            <Input
              id="contact_name"
              type="text"
              value={contactName}
              onChange={e => setContactName(e.target.value)}
            />
            <FieldError error={errors} field={"contact_name"}/>
          </>
        ) : null}
        <Label htmlFor="email">{customerType === "legal" ? "Kapcsolattartó e-mail címe" : "E-mail cím"}</Label>
        <Input
          id="email"
          type="text"
          value={email}
          onChange={e => setEmail(e.target.value)}
        />
        <FieldError error={errors} field={"email"}/>
        <Label htmlFor="phone_number">{customerType === "legal" ? "Kapcsolattartó telefonszáma" : "Telefonszám"}</Label>
        <Input
          id="phone_number"
          type="text"
          value={phoneNumber}
          onChange={e => setPhoneNumber(e.target.value)}
        />
        <FieldError error={errors} field={"phone_number"}/>


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
            <SelectItem value="lead">Érdeklődő</SelectItem>
            <SelectItem value="prospect">Lehetséges vevő</SelectItem>
          </SelectContent>
        </Select>
        <FieldError error={errors} field={"status"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}