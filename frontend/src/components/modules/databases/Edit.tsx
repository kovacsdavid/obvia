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

import React from "react";
import { useAppDispatch } from "@/store/hooks.ts";
import { Button, FieldError, GlobalError, Input, Label } from "@/components/ui";
import { create } from "@/components/modules/databases/lib/slice.ts";
import { useActivateDatabase } from "@/hooks/use_activate_database.ts";
import { useFormError } from "@/hooks/use_form_error.ts";
import { useNavigate } from "react-router-dom";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { useParams } from "react-router";
import type { Database } from "@/components/modules/databases/lib/interface.ts";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (database: Database) => void;
}

export default function Edit({
  showCard = true,
  onSuccess = undefined,
}: EditProps) {
  const [name, setName] = React.useState("");
  const [dbIsSelfHosted, setDbIsSelfHosted] = React.useState<
    boolean | "indeterminate"
  >(false);
  const [dbHost, setDbHost] = React.useState("");
  const [dbPort, setDbPort] = React.useState("");
  const [dbName, setDbName] = React.useState("");
  const [dbUser, setDbUser] = React.useState("");
  const [dbPassword, setDbPassword] = React.useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const { errors, setErrors, unexpectedError } = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const activateDatabase = useActivateDatabase();

  React.useEffect(() => {
    setDbHost("");
    setDbPort("");
    setDbName("");
    setDbUser("");
    setDbPassword("");
  }, [setDbIsSelfHosted]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const dbPortNumber = parseInt(dbPort, 10);
    dispatch(
      create({
        name,
        dbIsSelfHosted: dbIsSelfHosted === true,
        dbHost,
        dbPort: dbPortNumber,
        dbName,
        dbUser,
        dbPassword,
      }),
    ).then(async (response) => {
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (typeof response.payload.jsonData?.data !== "undefined") {
            activateDatabase(response.payload.jsonData.data.id).then((isOk) => {
              if (isOk) {
                if (
                  typeof onSuccess === "function" &&
                  typeof response.payload.jsonData?.data !== "undefined"
                ) {
                  onSuccess(response.payload.jsonData.data);
                } else {
                  navigate("/adatbazis/lista");
                }
              } else {
                unexpectedError();
              }
            });
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
  };

  return (
    <>
      <GlobalError error={errors} />
      <ConditionalCard
        showCard={showCard}
        title={`Adatbázis ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form
          onSubmit={handleSubmit}
          className="max-w-sm mx-auto space-y-4"
          autoComplete={"off"}
        >
          <Label htmlFor="name">Adatbázis neve</Label>
          <Input
            id="name"
            type="text"
            placeholder="Példa Kft."
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"} />
          <div className="text-right mt-8">
            <Button
              className="mr-3"
              variant="outline"
              onClick={(e: React.FormEvent) => {
                e.preventDefault();
                navigate(-1);
              }}
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
