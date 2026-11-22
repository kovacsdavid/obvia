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
import { Card, CardContent } from "@/components/ui/card.tsx";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get,
    select_list,
} from "@/components/modules/inventory_reservations/lib/slice.ts";
import { useNavigate, useParams } from "react-router";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { type SelectOptionList } from "@/lib/interface.ts";
import { Dialog, DialogContent, DialogTitle } from "@/components/ui/dialog.tsx";
import InventoryEdit from "@/components/modules/inventory/Edit.tsx";
import type { Inventory } from "../inventory/lib/interface";
import type { InventoryReservationError } from "./lib/interface";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";
import { Plus } from "lucide-react";
import { useNumberInput } from "@/hooks/use_number_input.ts";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";

export default function Edit() {
    const params = useParams();
    const routeInventoryId = React.useMemo(
        () => params["inventoryId"] ?? "",
        [params],
    );
    const [inventoryId, setInventoryId] = React.useState(routeInventoryId);
    const [referenceType, setReferenceType] = React.useState<string | null>("");
    const [referenceId, setReferenceId] = React.useState<string | null>("");
    const [reservedUntil, setReservedUntil] = React.useState<string | null>("");
    const [status, setStatus] = React.useState("active");
    const [inventoryIdList, setInventoryIdList] =
        React.useState<SelectOptionList>([]);
    const [referenceIdList, setReferenceIdList] =
        React.useState<SelectOptionList>([]);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const [errors, setErrors] =
        React.useState<InventoryReservationError | null>(null);
    const id = React.useMemo(() => params["id"] ?? "", [params]);
    const [openNewInventoryDialog, setOpenNewInventoryDialog] =
        React.useState(false);
    const quantity = useNumberInput({
        showThousandSeparator: true,
        decimalPlaces: 2,
        allowEmpty: true,
    });

    const handleReferenceTypeChange = useCallback(
        async (newReferenceType: string) => {
            setReferenceType(newReferenceType);
            setReferenceIdList([]);
            const result = await dispatch(
                select_list({ list: newReferenceType }),
            ).unwrap();
            handleApiResult(result, {
                onSuccess: ({ data }) => setReferenceIdList(data ?? []),
            });
        },
        [dispatch],
    );

    const loadLists = useCallback(async () => {
        if (!routeInventoryId) {
            const result = await dispatch(
                select_list({ list: "inventory" }),
            ).unwrap();
            handleApiResult(result, {
                onSuccess: ({ data }) => setInventoryIdList(data ?? []),
            });
        }
    }, [dispatch, routeInventoryId]);

    const handleEditInventorySuccess = (inventory: Inventory) => {
        loadLists()
            .then(() => {
                setTimeout(() => {
                    setInventoryId(inventory.id);
                }, 0);
                setOpenNewInventoryDialog(false);
            })
            .catch(() => undefined);
    };

    useEffect(() => {
        loadLists()
            .then(() => {
                if (typeof id === "string" && id.length === 36) {
                    dispatch(get({ uuid: id }))
                        .unwrap()
                        .then((result) => {
                            handleApiResult(result, {
                                onSuccess: ({ data }) => {
                                    if (data) {
                                        setInventoryId(data.inventory_id);
                                        quantity.setValue(
                                            data.quantity
                                                ? data.quantity.toString()
                                                : "",
                                        );
                                        handleReferenceTypeChange(
                                            data.reference_type ?? "",
                                        )
                                            .then(() => {
                                                setReferenceId(
                                                    data.reference_id ?? "",
                                                );
                                            })
                                            .catch(() => undefined);
                                        setReservedUntil(data.reserved_until);
                                        setStatus(data.status);
                                    }
                                },
                            });
                        })
                        .catch(() => undefined);
                }
            })
            .catch(() => undefined);
        // quantity is intentionally omitted to avoid infinite loops
        // They are only used to set initial values and don't need to trigger re-runs
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [dispatch, handleReferenceTypeChange, id, loadLists]);

    const resetError = (field: keyof InventoryReservationError) => {
        setErrors((previous) =>
            previous ? { ...previous, [field]: null } : previous,
        );
    };
    const isInvalidField = (field: keyof InventoryReservationError) =>
        typeof errors?.[field] === "string";

    const handleSubmit = useCallback(
        (e: React.SubmitEvent) => {
            e.preventDefault();
            dispatch(
                create({
                    input: {
                        id: id || null,
                        inventoryId,
                        quantity: !isNaN(quantity.getNumericValue())
                            ? quantity.getNumericValue().toString()
                            : "",
                        referenceType,
                        referenceId,
                        reservedUntil,
                        status,
                    },
                }),
            )
                .unwrap()
                .then((result) => {
                    handleApiResult(result, {
                        onSuccess: () => {
                            toast.add({
                                type: "success",
                                title: "A mentés sikeresen megtörtént!",
                            });
                            navigate(-1);
                        },
                        onFieldErrors: setErrors,
                    });
                })
                .catch(() => undefined);
        },
        [
            dispatch,
            id,
            inventoryId,
            quantity,
            referenceType,
            referenceId,
            reservedUntil,
            status,
            navigate,
        ],
    );

    return (
        <>
            <Dialog
                open={openNewInventoryDialog}
                onOpenChange={setOpenNewInventoryDialog}
            >
                <DialogContent>
                    <DialogTitle>Raktárkészlet létrehozása</DialogTitle>
                    <InventoryEdit
                        showCard={false}
                        createOnly
                        onSuccess={handleEditInventorySuccess}
                        onCancel={() => setOpenNewInventoryDialog(false)}
                    />
                </DialogContent>
            </Dialog>
            <Card className={"max-w-lg mx-auto"}>
                <CardContent>
                    <form
                        onSubmit={handleSubmit}
                        className="space-y-4"
                        autoComplete={"off"}
                    >
                        <FieldSet>
                            <FieldLegend>Készlet foglalás</FieldLegend>
                            <FieldGroup>
                                {!routeInventoryId && (
                                    <>
                                        <Field
                                            data-invalid={isInvalidField(
                                                "inventory_id",
                                            )}
                                        >
                                            <div className="flex items-center w-full">
                                                <div className="flex flex-1 items-center">
                                                    <FieldLabel htmlFor="inventory_id">
                                                        Raktárkészlet
                                                    </FieldLabel>
                                                </div>
                                                <div className="flex items-center">
                                                    <Button
                                                        type="button"
                                                        variant="outline"
                                                        onClick={() =>
                                                            setOpenNewInventoryDialog(
                                                                true,
                                                            )
                                                        }
                                                    >
                                                        <Plus />
                                                    </Button>
                                                </div>
                                            </div>
                                            <Select
                                                disabled={
                                                    inventoryIdList.length === 0
                                                }
                                                value={inventoryId ?? ""}
                                                onValueChange={(val) => {
                                                    resetError("inventory_id");
                                                    setInventoryId(val);
                                                }}
                                            >
                                                <SelectTrigger
                                                    className={"w-full"}
                                                    aria-invalid={isInvalidField(
                                                        "inventory_id",
                                                    )}
                                                >
                                                    <SelectValue />
                                                </SelectTrigger>
                                                <SelectContent>
                                                    {inventoryIdList.map(
                                                        (
                                                            inventoryIdListItem,
                                                        ) => (
                                                            <SelectItem
                                                                key={
                                                                    inventoryIdListItem.value
                                                                }
                                                                value={
                                                                    inventoryIdListItem.value
                                                                }
                                                            >
                                                                {
                                                                    inventoryIdListItem.title
                                                                }
                                                            </SelectItem>
                                                        ),
                                                    )}
                                                </SelectContent>
                                            </Select>
                                            <FieldErrorV2
                                                error={errors?.inventory_id}
                                            />
                                        </Field>
                                    </>
                                )}

                                <Field
                                    data-invalid={isInvalidField("quantity")}
                                >
                                    <FieldLabel htmlFor="quantity">
                                        Mennyiség
                                    </FieldLabel>
                                    <Input
                                        id="quantity"
                                        type="text"
                                        placeholder="10"
                                        value={quantity.displayValue}
                                        onChange={(e) => {
                                            resetError("quantity");
                                            quantity.handleInputChangeWithCursor(
                                                e.target.value,
                                                e.target,
                                            );
                                        }}
                                        aria-invalid={isInvalidField(
                                            "quantity",
                                        )}
                                    />
                                    <FieldErrorV2 error={errors?.quantity} />
                                </Field>

                                <Field
                                    data-invalid={isInvalidField(
                                        "reference_type",
                                    )}
                                >
                                    <FieldLabel htmlFor="reference_type">
                                        Hivatkozás típusa
                                    </FieldLabel>
                                    <Select
                                        value={referenceType ?? ""}
                                        onValueChange={(val) => {
                                            resetError("reference_type");
                                            handleReferenceTypeChange(
                                                val,
                                            ).catch(() => undefined);
                                        }}
                                    >
                                        <SelectTrigger
                                            className={"w-full"}
                                            aria-invalid={isInvalidField(
                                                "reference_type",
                                            )}
                                        >
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="worksheets">
                                                Munkalap
                                            </SelectItem>
                                        </SelectContent>
                                    </Select>
                                    <FieldErrorV2
                                        error={errors?.reference_type}
                                    />
                                </Field>

                                <Field
                                    data-invalid={isInvalidField(
                                        "reference_id",
                                    )}
                                >
                                    <FieldLabel htmlFor="reference_id">
                                        Hivatkozás azonosító
                                    </FieldLabel>
                                    <Select
                                        disabled={referenceIdList.length === 0}
                                        value={referenceId ?? ""}
                                        onValueChange={(val) => {
                                            resetError("reference_id");
                                            setReferenceId(val);
                                        }}
                                    >
                                        <SelectTrigger
                                            className={"w-full"}
                                            aria-invalid={isInvalidField(
                                                "reference_id",
                                            )}
                                        >
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            {referenceIdList.map(
                                                (referenceIdListItem) => (
                                                    <SelectItem
                                                        key={
                                                            referenceIdListItem.value
                                                        }
                                                        value={
                                                            referenceIdListItem.value
                                                        }
                                                    >
                                                        {
                                                            referenceIdListItem.title
                                                        }
                                                    </SelectItem>
                                                ),
                                            )}
                                        </SelectContent>
                                    </Select>
                                    <FieldErrorV2
                                        error={errors?.reference_id}
                                    />
                                </Field>

                                <Field
                                    data-invalid={isInvalidField(
                                        "reserved_until",
                                    )}
                                >
                                    <FieldLabel htmlFor="reserved_until">
                                        Foglalás lejárata
                                    </FieldLabel>
                                    <Input
                                        id="reserved_until"
                                        type="date"
                                        value={reservedUntil ?? ""}
                                        onChange={(e) => {
                                            resetError("reserved_until");
                                            setReservedUntil(e.target.value);
                                        }}
                                        aria-invalid={isInvalidField(
                                            "reserved_until",
                                        )}
                                    />
                                    <FieldErrorV2
                                        error={errors?.reserved_until}
                                    />
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
                                            aria-invalid={isInvalidField(
                                                "status",
                                            )}
                                        >
                                            <SelectValue />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="active">
                                                Aktív
                                            </SelectItem>
                                            <SelectItem value="fulfilled">
                                                Teljesített
                                            </SelectItem>
                                            <SelectItem value="cancelled">
                                                Lemondott
                                            </SelectItem>
                                            <SelectItem value="expired">
                                                Lejárt
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
                </CardContent>
            </Card>
        </>
    );
}
