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
import { Button, FieldErrorV2, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get,
    update,
} from "@/components/modules/warehouses/lib/slice.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { useNavigate } from "react-router";
import { useParams } from "react-router";
import { ConditionalCard } from "@/components/ui/card.tsx";
import type { Warehouse, WarehouseError } from "./lib/interface";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";

interface EditProps {
    showCard?: boolean;
    createOnly?: boolean;
    onSuccess?: (warehouse: Warehouse) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    createOnly = false,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [name, setName] = React.useState("");
    const [contactName, setContactName] = React.useState("");
    const [contactPhone, setContactPhone] = React.useState("");
    const [status, setStatus] = React.useState("active");
    const [errors, setErrors] = React.useState<WarehouseError | null>(null);
    const isInvalidField = (field: keyof WarehouseError) => !!errors?.[field];
    const resetError = (field: keyof WarehouseError) => {
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : null,
        );
    };
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const id = createOnly ? null : (params["id"] ?? null);

    const handleCreate = useCallback(async () => {
        const result = await dispatch(
            create({
                input: { id, name, contactName, contactPhone, status },
            }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: ({ data }) => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                if (typeof onSuccess === "function" && data) {
                    onSuccess(data);
                } else {
                    navigate("/raktar/lista");
                }
            },
            onFieldErrors: setErrors,
        });
    }, [
        contactName,
        contactPhone,
        dispatch,
        id,
        name,
        navigate,
        onSuccess,
        setErrors,
        status,
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

    const handleUpdate = useCallback(async () => {
        const result = await dispatch(
            update({
                input: { id, name, contactName, contactPhone, status },
            }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/raktar/lista");
            },
            onFieldErrors: setErrors,
        });
    }, [
        contactName,
        contactPhone,
        dispatch,
        id,
        name,
        navigate,
        setErrors,
        status,
    ]);

    useEffect(() => {
        if (typeof id === "string") {
            dispatch(get({ uuid: id }))
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (data) {
                                setName(data.name);
                                setContactName(data.contact_name ?? "");
                                setContactPhone(data.contact_phone ?? "");
                                setStatus(data.status ?? "");
                            }
                        },
                    });
                })
                .catch(() => undefined);
        }
    }, [dispatch, id]);

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
            <ConditionalCard showCard={showCard} className={"max-w-lg mx-auto"}>
                <form
                    onSubmit={handleSubmit}
                    className="space-y-4"
                    autoComplete={"off"}
                >
                    <FieldSet>
                        <FieldLegend>
                            {`Raktár ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field data-invalid={isInvalidField("name")}>
                                <FieldLabel htmlFor="name">Név</FieldLabel>
                                <Input
                                    id="name"
                                    placeholder="Sopron 4"
                                    type="text"
                                    value={name}
                                    onChange={(e) => {
                                        resetError("name");
                                        setName(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("name")}
                                />
                                <FieldErrorV2 error={errors?.name} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("contact_name")}
                            >
                                <FieldLabel htmlFor="contact_name">
                                    Kapcsolattartó neve
                                </FieldLabel>
                                <Input
                                    id="contact_name"
                                    type="text"
                                    placeholder="Példa Béla"
                                    value={contactName}
                                    onChange={(e) => {
                                        resetError("contact_name");
                                        setContactName(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField(
                                        "contact_name",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.contact_name} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("contact_phone")}
                            >
                                <FieldLabel htmlFor="contact_phone">
                                    Kapcsolattartó telefonszáma
                                </FieldLabel>
                                <Input
                                    id="contact_phone"
                                    type="text"
                                    placeholder="+36301234567"
                                    value={contactPhone}
                                    onChange={(e) => {
                                        resetError("contact_phone");
                                        setContactPhone(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField(
                                        "contact_phone",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.contact_phone} />
                            </Field>
                            <Field data-invalid={isInvalidField("status")}>
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
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
                                        <SelectItem value="active">
                                            Aktív
                                        </SelectItem>
                                        <SelectItem value="inactive">
                                            Inaktív
                                        </SelectItem>
                                        <SelectItem value="maintenance">
                                            Karbantartás alatt
                                        </SelectItem>
                                        <SelectItem value="closed">
                                            Véglegesen bezárt
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.status} />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <Field orientation="horizontal">
                        <div className="text-right mt-8 w-full">
                            <Button
                                className="mr-3"
                                variant="outline"
                                onClick={handleCancel}
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
