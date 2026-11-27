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
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { create } from "@/components/modules/databases/lib/slice.ts";
import { useActivateDatabase } from "@/hooks/use_activate_database.ts";
import { useNavigate } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import { useParams } from "react-router";
import type {
    Database,
    DatabaseError,
} from "@/components/modules/databases/lib/interface.ts";
import { handleApiResult } from "@/lib/api/handleApiResult";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";

interface EditProps {
    showCard?: boolean;
    onSuccess?: (database: Database) => void;
}

export default function Edit({
    showCard = true,
    onSuccess = undefined,
}: EditProps) {
    const [form, setForm] = React.useState({ name: "" });
    const [errors, setErrors] = React.useState<DatabaseError | null>(null);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const id = React.useMemo(() => params["id"] ?? null, [params]);

    const activateDatabase = useActivateDatabase();

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        const response = await dispatch(create({ input: form }));
        if (!create.fulfilled.match(response)) return;
        const result = response.payload;
        const successfulDatabase = { value: null as Database | null };
        handleApiResult(result, {
            onSuccess: ({ data }) => {
                if (
                    result.kind === "success" &&
                    result.status === 201 &&
                    data
                ) {
                    successfulDatabase.value = data;
                }
            },
            onFieldErrors: setErrors,
        });
        const database = successfulDatabase.value;
        if (database) {
            if (await activateDatabase(database.id)) {
                if (typeof onSuccess === "function") onSuccess(database);
                else navigate("/adatbazis/lista");
            }
        }
    };

    return (
        <>
            <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
                <form
                    onSubmit={handleSubmit}
                    className="max-w-sm mx-auto space-y-4"
                    autoComplete={"off"}
                >
                    <FieldSet>
                        <FieldLegend>
                            {`Adatbázis ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field
                                data-invalid={typeof errors?.name === "string"}
                            >
                                <FieldLabel htmlFor="name">
                                    Adatbázis neve
                                </FieldLabel>
                                <Input
                                    id="name"
                                    type="text"
                                    placeholder="Példa Kft."
                                    value={form.name}
                                    onChange={(e) => {
                                        setErrors((previous) =>
                                            previous
                                                ? { ...previous, name: null }
                                                : previous,
                                        );
                                        setForm((previous) => ({
                                            ...previous,
                                            name: e.target.value,
                                        }));
                                    }}
                                    aria-invalid={
                                        typeof errors?.name === "string"
                                    }
                                />
                                <FieldErrorV2 error={errors?.name} />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <Field orientation="horizontal">
                        <div className="text-right mt-8 w-full">
                            <Button
                                className="mr-3"
                                variant="outline"
                                onClick={(e: React.MouseEvent) => {
                                    e.preventDefault();
                                    navigate(-1);
                                }}
                            >
                                Mégse
                            </Button>
                            <Button type="submit">
                                {id ? "Módosítás" : "Létrehozás"}
                            </Button>
                        </div>
                    </Field>
                </form>
            </ConditionalCard>
        </>
    );
}
