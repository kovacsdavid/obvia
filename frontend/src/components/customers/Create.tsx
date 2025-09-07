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
import {create} from "@/store/slices/customers.ts";
import { type ErrorContainerWithFields } from "@/lib/interfaces.ts";

export default function Create() {
  const [customerType, setCustomerType] = React.useState("");
  const [name, setName] = React.useState("");
  const [contactName, setContactName] = React.useState("");
  const [email, setEmail] = React.useState("");
  const [phoneNumber, setPhoneNumber] = React.useState("");
  const [status, setStatus] = React.useState("");
  const dispatch = useAppDispatch();
  const [errors, setErrors] = useState<ErrorContainerWithFields | null>(null);

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
      console.log(response)
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/vevo/lista";
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
        <Label htmlFor="customer_type">Típus</Label>
        <Input
          id="customer_type"
          type="text"
          value={customerType}
          onChange={e => setCustomerType(e.target.value)}
        />
        <FieldError error={errors} field={"customer_type"}/>
        <Label htmlFor="name">Név</Label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <FieldError error={errors} field={"name"}/>
        <Label htmlFor="contact_name">Kapcsolattartó neve</Label>
        <Input
          id="contact_name"
          type="text"
          value={contactName}
          onChange={e => setContactName(e.target.value)}
        />
        <FieldError error={errors} field={"contact_name"}/>
        <Label htmlFor="email">E-mail cím</Label>
        <Input
          id="email"
          type="text"
          value={email}
          onChange={e => setEmail(e.target.value)}
        />
        <FieldError error={errors} field={"email"}/>
        <Label htmlFor="phone_number">Telefonszám</Label>
        <Input
          id="phone_number"
          type="text"
          value={phoneNumber}
          onChange={e => setPhoneNumber(e.target.value)}
        />
        <FieldError error={errors} field={"phone_number"}/>
        <Label htmlFor="status">Státusz</Label>
        <Input
          id="status"
          type="text"
          value={status}
          onChange={e => setStatus(e.target.value)}
        />
        <FieldError error={errors} field={"status"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}