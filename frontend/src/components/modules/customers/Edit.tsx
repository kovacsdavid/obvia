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

import React, { useCallback, useEffect, useMemo } from "react";
import { Button, Checkbox, FieldErrorV2, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    create,
    get_full,
    update,
} from "@/components/modules/customers/lib/slice.ts";
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
import type {
    Customer,
    CustomerErrors,
} from "@/components/modules/customers/lib/interface.ts";
import {
    Field,
    FieldGroup,
    FieldLabel,
    FieldLegend,
    FieldSet,
} from "@/components/ui/field";
import Address from "@/components/modules/address/Address";
import {
    type Address as AddressInterface,
    type AddressErrors,
} from "@/components/modules/address/lib/interface";
import {
    normalizeAddress,
    defaultAddress,
} from "@/components/modules/address/lib/utils";
import { handleApiResult } from "@/lib/api/handleApiResult";
import { toast } from "@/components/ui/toast-lib";

interface EditProps {
    showCard?: boolean;
    onSuccess?: (customer: Customer) => void;
    onCancel?: () => void;
}

export default function Edit({
    showCard = true,
    onSuccess = undefined,
    onCancel = undefined,
}: EditProps) {
    const [customerType, setCustomerType] = React.useState<string | undefined>(
        "natural",
    );
    const [name, setName] = React.useState("");
    const [contactName, setContactName] = React.useState("");
    const [email, setEmail] = React.useState("");
    const [phoneNumber, setPhoneNumber] = React.useState("");
    const [status, setStatus] = React.useState<string | undefined>("active");
    const [billingAddress, setBillingAddress] = React.useState<
        AddressInterface | null | undefined
    >(defaultAddress());
    const [mailingAddress, setMailingAddress] = React.useState<
        AddressInterface | null | undefined
    >(undefined);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const params = useParams();
    const [errors, setErrors] = React.useState<CustomerErrors | null>(null);
    const id = React.useMemo(() => params["id"] ?? null, [params]);

    const handleNeedMailingAddressChange = () => {
        if (mailingAddress === null || typeof mailingAddress === "undefined") {
            setMailingAddress(defaultAddress());
        } else {
            setMailingAddress(undefined);
        }
    };

    const handleCreate = async () => {
        const result = await dispatch(
            create({
                input: {
                    id,
                    name,
                    contactName,
                    email,
                    phoneNumber,
                    status,
                    customerType,
                    billingAddress,
                    mailingAddress,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: ({ data }) => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                if (
                    typeof onSuccess === "function" &&
                    typeof data !== "undefined"
                ) {
                    onSuccess(data);
                } else {
                    navigate("/vevo/lista");
                }
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
                    contactName,
                    email,
                    phoneNumber,
                    status,
                    customerType,
                    billingAddress,
                    mailingAddress,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: () => {
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
                navigate("/vevo/lista");
            },
            onFieldErrors: setErrors,
        });
    };

    useEffect(() => {
        if (typeof id === "string") {
            dispatch(get_full({ uuid: id }))
                .unwrap()
                .then(async (result) => {
                    handleApiResult(result, {
                        onSuccess: ({ data }) => {
                            if (typeof data !== "undefined") {
                                setCustomerType(data.customer_type);
                                setName(data.name);
                                setContactName(data.contact_name ?? "");
                                setEmail(data.email ?? "");
                                setPhoneNumber(data.phone_number ?? "");
                                setStatus(data.status);
                                setBillingAddress(
                                    // NOTE: this is needed to show address fields on update for
                                    // customers created before address was added to the system.
                                    data.billing_address === null
                                        ? defaultAddress()
                                        : normalizeAddress(
                                              data.billing_address,
                                          ),
                                );
                                setMailingAddress(
                                    normalizeAddress(data.mailing_address),
                                );
                            }
                        },
                    });
                });
        }
    }, [dispatch, id, setErrors]);

    const handleSubmit = async (e: React.SubmitEvent) => {
        e.preventDefault();
        if (typeof id === "string") {
            handleUpdate();
        } else {
            handleCreate();
        }
    };

    type AddressKey = "billing_address" | "mailing_address";

    const addressErrorSetter = useCallback(
        (key: AddressKey) =>
            <T extends keyof AddressErrors>(value: string | null, field: T) =>
                setErrors((prev) => {
                    if (!prev) return prev;
                    // already in the desired state → keep the same reference
                    if ((prev[key]?.[field] ?? null) === value) return prev;
                    return { ...prev, [key]: { ...prev[key], [field]: value } };
                }),
        [],
    );

    const setBillingAddressErrors = useMemo(
        () => addressErrorSetter("billing_address"),
        [addressErrorSetter],
    );
    const setMailingAddressErrors = useMemo(
        () => addressErrorSetter("mailing_address"),
        [addressErrorSetter],
    );

    type FlatErrorKey = Exclude<
        keyof CustomerErrors,
        "billing_address" | "mailing_address"
    >;

    const clearError = (field: FlatErrorKey) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    };

    const isInvalidField = (field: keyof CustomerErrors) =>
        typeof errors?.[field] === "string";

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
                            {`Vevő ${id ? "módosítás" : "létrehozás"}`}
                        </FieldLegend>
                        <FieldGroup>
                            <Field
                                data-invalid={isInvalidField("customer_type")}
                            >
                                <FieldLabel htmlFor="customer_type">
                                    Típus
                                </FieldLabel>
                                <Select
                                    value={customerType}
                                    onValueChange={(val) => {
                                        clearError("customer_type");
                                        setCustomerType(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={isInvalidField(
                                            "customer_type",
                                        )}
                                    >
                                        <SelectValue />
                                    </SelectTrigger>
                                    <SelectContent>
                                        <SelectItem value="natural">
                                            Természetes személy
                                        </SelectItem>
                                        <SelectItem value="legal">
                                            Jogi személy
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                                <FieldErrorV2 error={errors?.customer_type} />
                            </Field>
                            <Field data-invalid={isInvalidField("name")}>
                                <FieldLabel htmlFor="name">
                                    {customerType === "legal"
                                        ? "Jogi személy neve"
                                        : "Név"}
                                </FieldLabel>
                                <Input
                                    id="name"
                                    type="text"
                                    placeholder={
                                        customerType === "legal"
                                            ? "Példa Kft."
                                            : "Példa Béla"
                                    }
                                    value={name}
                                    onChange={(e) => {
                                        clearError("name");
                                        setName(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("name")}
                                />
                                <FieldErrorV2 error={errors?.name} />
                            </Field>
                            {customerType === "legal" ? (
                                <>
                                    <Field
                                        data-invalid={isInvalidField(
                                            "contact_name",
                                        )}
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
                                                clearError("contact_name");
                                                setContactName(e.target.value);
                                            }}
                                            aria-invalid={isInvalidField(
                                                "contact_name",
                                            )}
                                        />

                                        <FieldErrorV2
                                            error={errors?.contact_name}
                                        />
                                    </Field>
                                </>
                            ) : null}
                            <Field data-invalid={isInvalidField("email")}>
                                <FieldLabel htmlFor="email">
                                    {customerType === "legal"
                                        ? "Kapcsolattartó e-mail címe"
                                        : "E-mail cím"}
                                </FieldLabel>
                                <Input
                                    id="email"
                                    type="text"
                                    placeholder="pelda@kovacsdavid.dev"
                                    value={email}
                                    onChange={(e) => {
                                        clearError("email");
                                        setEmail(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField("email")}
                                />
                                <FieldErrorV2 error={errors?.email} />
                            </Field>
                            <Field
                                data-invalid={isInvalidField("phone_number")}
                            >
                                <FieldLabel htmlFor="phone_number">
                                    {customerType === "legal"
                                        ? "Kapcsolattartó telefonszáma"
                                        : "Telefonszám"}
                                </FieldLabel>
                                <Input
                                    id="phone_number"
                                    type="text"
                                    placeholder="+36301234567"
                                    value={phoneNumber}
                                    onChange={(e) => {
                                        clearError("phone_number");
                                        setPhoneNumber(e.target.value);
                                    }}
                                    aria-invalid={isInvalidField(
                                        "phone_number",
                                    )}
                                />
                                <FieldErrorV2 error={errors?.phone_number} />
                            </Field>
                            <Field data-invalid={isInvalidField("status")}>
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
                                <Select
                                    value={status}
                                    onValueChange={(val) => {
                                        clearError("status");
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
                                        <SelectItem value="lead">
                                            Érdeklődő
                                        </SelectItem>
                                        <SelectItem value="prospect">
                                            Lehetséges vevő
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
                    <div className="mt-8">
                        <Address
                            label="Számlázási cím"
                            value={billingAddress}
                            onChange={setBillingAddress}
                            errors={errors?.billing_address}
                            setErrors={setBillingAddressErrors}
                        />
                    </div>

                    <FieldGroup className="mt-8">
                        <Field orientation="horizontal">
                            <Checkbox
                                id="need_mailing_address"
                                checked={
                                    !(
                                        mailingAddress === null ||
                                        typeof mailingAddress === "undefined"
                                    )
                                }
                                onCheckedChange={() =>
                                    handleNeedMailingAddressChange()
                                }
                            />
                            <FieldLabel htmlFor="need_mailing_address">
                                Levelezési cím eltér
                            </FieldLabel>
                        </Field>
                    </FieldGroup>

                    <Address
                        label="Levelezési cím"
                        value={mailingAddress}
                        onChange={setMailingAddress}
                        errors={errors?.mailing_address}
                        setErrors={setMailingAddressErrors}
                    />
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
