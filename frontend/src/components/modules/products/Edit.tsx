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
    select_list,
    update,
} from "@/components/modules/products/lib/slice.ts";
import { type SelectOptionList } from "@/lib/interface.ts";
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
import type { Product, ProductError } from "./lib/interface";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";

interface EditProps {
    showCard?: boolean;
    createOnly?: boolean;
    onSuccess?: (products: Product) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    createOnly = false,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [name, setName] = React.useState("");
    const [description, setDescription] = React.useState("");
    const [unitOfMeasureId, setUnitOfMeasureId] = React.useState("");
    const [newUnitOfMeasure, setNewUnitOfMeasure] = React.useState("");
    const [unitsOfMeasureList, setUnitsOfMeasureList] =
        React.useState<SelectOptionList>([]);
    const [status, setStatus] = React.useState("active");
    const [errors, setErrors] = React.useState<ProductError | null>(null);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const id = createOnly ? null : (params["id"] ?? null);

    const handleCreate = async () => {
        const result = await dispatch(
            create({
                input: {
                    id,
                    name,
                    description,
                    unitOfMeasureId,
                    newUnitOfMeasure,
                    status,
                },
            }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: ({ data }) => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                if (onSuccess && data !== undefined) onSuccess(data);
                else navigate("/termek/lista");
            },
            onFieldErrors: setErrors,
        });
    };

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

    const handleUpdate = async () => {
        const result = await dispatch(
            update({
                input: {
                    id,
                    name,
                    description,
                    unitOfMeasureId,
                    newUnitOfMeasure,
                    status,
                },
            }),
        ).unwrap();
        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/termek/lista");
            },
            onFieldErrors: setErrors,
        });
    };

    useEffect(() => {
        const listRequest = dispatch(select_list({ list: "units_of_measure" }));
        listRequest
            .unwrap()
            .then((result) => {
                handleApiResult(result, {
                    onSuccess: ({ data }) => setUnitsOfMeasureList(data ?? []),
                });
            })
            .catch(() => undefined);
        const getRequest =
            typeof id === "string" ? dispatch(get({ uuid: id })) : null;
        getRequest
            ?.unwrap()
            .then((result) => {
                handleApiResult(result, {
                    onSuccess: ({ data }) => {
                        if (data) {
                            setName(data.name);
                            setDescription(data.description ?? "");
                            setUnitOfMeasureId(data.unit_of_measure_id);
                            setStatus(data.status);
                        }
                    },
                });
            })
            .catch(() => undefined);
        return () => {
            listRequest.abort();
            getRequest?.abort();
        };
    }, [dispatch, id]);

    const resetError = (field: keyof ProductError) => {
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : previous,
        );
    };
    const isInvalidField = (field: keyof ProductError) =>
        typeof errors?.[field] === "string";

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        try {
            if (typeof id === "string") await handleUpdate();
            else await handleCreate();
        } catch {
            return;
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
                                <FieldErrorV2 error={errors?.name} />
                            </Field>
                            <Field data-invalid={isInvalidField("description")}>
                                <FieldLabel htmlFor="description">
                                    Leírás
                                </FieldLabel>
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
                                <FieldErrorV2 error={errors?.description} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField(
                                    "unit_of_measure_id",
                                )}
                            >
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
                                        id="unit_of_measure_id"
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "unit_of_measure_id",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        {unitsOfMeasureList.map(
                                            (unit_of_measure) => {
                                                return (
                                                    <SelectItem
                                                        key={
                                                            unit_of_measure.value
                                                        }
                                                        value={
                                                            unit_of_measure.value
                                                        }
                                                    >
                                                        {unit_of_measure.title}
                                                    </SelectItem>
                                                );
                                            },
                                        )}
                                        <SelectItem value="other">
                                            Egyéb
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2
                                    error={errors?.unit_of_measure_id}
                                />
                            </Field>

                            {unitOfMeasureId === "other" ? (
                                <>
                                    <Field
                                        data-invalid={isInvalidField(
                                            "new_unit_of_measure",
                                        )}
                                    >
                                        <FieldLabel htmlFor="new_unit_of_measure">
                                            Egyéb mértékegység
                                        </FieldLabel>
                                        <Input
                                            id="new_unit_of_measure"
                                            type="text"
                                            value={newUnitOfMeasure}
                                            onChange={(e) => {
                                                resetError(
                                                    "new_unit_of_measure",
                                                );
                                                setNewUnitOfMeasure(
                                                    e.target.value,
                                                );
                                            }}
                                            aria-invalid={isInvalidField(
                                                "new_unit_of_measure",
                                            )}
                                        />
                                        <FieldErrorV2
                                            error={errors?.new_unit_of_measure}
                                        />
                                    </Field>
                                </>
                            ) : null}
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
