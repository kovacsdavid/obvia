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
import {
    Button,
    Checkbox,
    FieldErrorV2,
    GlobalError,
    Input,
} from "@/components/ui";
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
import { useFormErrorV2 } from "@/hooks/use_form_error_v2";

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
    const defaultAddress = () => ({
        id: "",
        type: "full_address",
        country_code: "HU",
        postal_code: "",
        settlement: "",
        mailbox: "",
        topographic_number: "",
        name_of_public_space: "",
        type_of_public_space: "",
        house_number: "",
        building: "",
        stairway: "",
        floor: "",
        door: "",
    });
    const [billingAddress, setBillingAddress] = React.useState<
        AddressInterface | null | undefined
    >(defaultAddress());
    const [mailingAddress, setMailingAddress] = React.useState<
        AddressInterface | null | undefined
    >(undefined);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { errors, setErrors, unexpectedError } =
        useFormErrorV2<CustomerErrors>();
    const params = useParams();
    const id = React.useMemo(() => params["id"] ?? null, [params]);

    const handleNeedMailingAddressChange = () => {
        if (typeof mailingAddress === "undefined") {
            setMailingAddress(defaultAddress());
        } else {
            setMailingAddress(undefined);
        }
    };

    const handleCreate = () => {
        dispatch(
            create({
                id,
                name,
                contactName,
                email,
                phoneNumber,
                status,
                customerType,
                billingAddress,
                mailingAddress,
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
                        navigate("/vevo/lista");
                    }
                } else if (
                    typeof response.payload.jsonData?.error !== "undefined"
                ) {
                    setErrors(response.payload.jsonData.error);
                } else {
                    unexpectedError(response.payload.statusCode);
                }
            } else {
                unexpectedError();
            }
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

    const handleUpdate = () => {
        dispatch(
            update({
                id,
                name,
                contactName,
                email,
                phoneNumber,
                status,
                customerType,
                billingAddress,
                mailingAddress,
            }),
        ).then(async (response) => {
            if (update.fulfilled.match(response)) {
                if (response.payload.statusCode === 200) {
                    navigate("/vevo/lista");
                } else if (
                    typeof response.payload.jsonData?.error !== "undefined"
                ) {
                    setErrors(response.payload.jsonData.error);
                } else {
                    unexpectedError(response.payload.statusCode);
                }
            } else {
                unexpectedError();
            }
        });
    };

    useEffect(() => {
        if (typeof id === "string") {
            dispatch(get_full(id)).then(async (response) => {
                if (get_full.fulfilled.match(response)) {
                    if (response.payload.statusCode === 200) {
                        if (
                            typeof response.payload.jsonData?.data !==
                            "undefined"
                        ) {
                            const data = response.payload.jsonData.data;
                            setCustomerType(data.customer_type);
                            setName(data.name);
                            setContactName(data.contact_name ?? "");
                            setEmail(data.email);
                            setPhoneNumber(data.phone_number ?? "");
                            setStatus(data.status);
                            setBillingAddress(data.billing_address);
                            setMailingAddress(data.mailing_address);
                        }
                    } else if (
                        typeof response.payload.jsonData?.error !== "undefined"
                    ) {
                        setErrors(response.payload.jsonData?.error);
                    } else {
                        unexpectedError(response.payload.statusCode);
                    }
                } else {
                    unexpectedError();
                }
            });
        }
    }, [dispatch, id, setErrors, unexpectedError]);

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
            <GlobalError error={errors} />
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
                                data-invalid={
                                    typeof errors?.fields?.customer_type ===
                                    "string"
                                }
                            >
                                <FieldLabel htmlFor="customer_type">
                                    Típus
                                </FieldLabel>
                                <Select
                                    value={customerType}
                                    onValueChange={(val) => {
                                        setErrors((prev) => {
                                            if (!prev?.fields) return prev;

                                            return {
                                                ...prev,
                                                fields: {
                                                    ...prev.fields,
                                                    customer_type: null,
                                                },
                                            };
                                        });
                                        setCustomerType(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={
                                            typeof errors?.fields
                                                ?.customer_type === "string"
                                        }
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
                                <FieldErrorV2
                                    error={errors?.fields?.customer_type}
                                />
                            </Field>
                            <Field
                                data-invalid={
                                    typeof errors?.fields?.name === "string"
                                }
                            >
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
                                        setErrors((prev) => {
                                            if (!prev?.fields) return prev;

                                            return {
                                                ...prev,
                                                fields: {
                                                    ...prev.fields,
                                                    name: null,
                                                },
                                            };
                                        });
                                        setName(e.target.value);
                                    }}
                                    aria-invalid={
                                        typeof errors?.fields?.name === "string"
                                    }
                                />
                                <FieldErrorV2 error={errors?.fields?.name} />
                            </Field>
                            {customerType === "legal" ? (
                                <>
                                    <Field
                                        data-invalid={
                                            typeof errors?.fields
                                                ?.contact_name === "string"
                                        }
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
                                                setErrors((prev) => {
                                                    if (!prev?.fields)
                                                        return prev;

                                                    return {
                                                        ...prev,
                                                        fields: {
                                                            ...prev.fields,
                                                            contact_name: null,
                                                        },
                                                    };
                                                });
                                                setContactName(e.target.value);
                                            }}
                                            aria-invalid={
                                                typeof errors?.fields
                                                    ?.contact_name === "string"
                                            }
                                        />

                                        <FieldErrorV2
                                            error={errors?.fields?.contact_name}
                                        />
                                    </Field>
                                </>
                            ) : null}
                            <Field
                                data-invalid={
                                    typeof errors?.fields?.email === "string"
                                }
                            >
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
                                        setErrors((prev) => {
                                            if (!prev?.fields) return prev;

                                            return {
                                                ...prev,
                                                fields: {
                                                    ...prev.fields,
                                                    email: null,
                                                },
                                            };
                                        });
                                        setEmail(e.target.value);
                                    }}
                                    aria-invalid={
                                        typeof errors?.fields?.email ===
                                        "string"
                                    }
                                />
                                <FieldErrorV2 error={errors?.fields?.email} />
                            </Field>
                            <Field
                                data-invalid={
                                    typeof errors?.fields?.phone_number ===
                                    "string"
                                }
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
                                        setErrors((prev) => {
                                            if (!prev?.fields) return prev;

                                            return {
                                                ...prev,
                                                fields: {
                                                    ...prev.fields,
                                                    phone_number: null,
                                                },
                                            };
                                        });
                                        setPhoneNumber(e.target.value);
                                    }}
                                    aria-invalid={
                                        typeof errors?.fields?.phone_number ===
                                        "string"
                                    }
                                />
                                <FieldErrorV2
                                    error={errors?.fields?.phone_number}
                                />
                            </Field>
                            <Field
                                data-invalid={
                                    typeof errors?.fields?.status === "string"
                                }
                            >
                                <FieldLabel htmlFor="status">
                                    Státusz
                                </FieldLabel>
                                <Select
                                    value={status}
                                    onValueChange={(val) => {
                                        setErrors((prev) => {
                                            if (!prev?.fields) return prev;

                                            return {
                                                ...prev,
                                                fields: {
                                                    ...prev.fields,
                                                    status: null,
                                                },
                                            };
                                        });
                                        setStatus(val);
                                    }}
                                >
                                    <SelectTrigger
                                        className={"w-full"}
                                        aria-invalid={
                                            typeof errors?.fields?.status ===
                                            "string"
                                        }
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
                                <FieldErrorV2 error={errors?.fields?.status} />
                            </Field>
                        </FieldGroup>
                    </FieldSet>
                    <div className="mt-8">
                        <Address
                            label="Számlázási cím"
                            value={billingAddress}
                            onChange={setBillingAddress}
                            errors={errors?.fields?.billing_address}
                            setErrors={<T extends keyof AddressErrors>(
                                value: string | null,
                                field: T,
                            ) => {
                                setErrors((prev) => {
                                    if (!prev?.fields) return prev;

                                    return {
                                        ...prev,
                                        fields: {
                                            ...prev.fields,
                                            billing_address: {
                                                ...prev.fields.billing_address,
                                                [field]: value,
                                            },
                                        },
                                    };
                                });
                            }}
                        />
                    </div>

                    <FieldGroup className="mt-8">
                        <Field orientation="horizontal">
                            <Checkbox
                                id="need_mailing_address"
                                checked={typeof mailingAddress !== "undefined"}
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
                        setErrors={<T extends keyof AddressErrors>(
                            value: string | null,
                            field: T,
                        ) => {
                            setErrors((prev) => {
                                if (!prev?.fields) return prev;

                                return {
                                    ...prev,
                                    fields: {
                                        ...prev.fields,
                                        mailing_address: {
                                            ...prev.fields.mailing_address,
                                            [field]: value,
                                        },
                                    },
                                };
                            });
                        }}
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
