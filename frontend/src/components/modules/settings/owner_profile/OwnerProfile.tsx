/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2026 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

import React, { useEffect } from "react";
import {
    Button,
    Checkbox,
    FieldErrorV2,
    GlobalError,
    Input,
} from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import { useNavigate } from "react-router";
import { get, update } from "@/components/modules/settings/lib/slice.ts";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select.tsx";
import { Field, FieldGroup, FieldLabel, FieldSet } from "@/components/ui/field";
import Address from "@/components/modules/address/Address";
import {
    type Address as AddressInterface,
    type AddressErrors,
} from "@/components/modules/address/lib/interface";
import {
    normalizeAddress,
    defaultAddress,
} from "@/components/modules/address/lib/utils";
import { useFormErrorV2 } from "@/hooks/use_form_error_v2";

export default function OwnerProfile() {
    const [ownerProfileType, setOwnerProfileType] = React.useState<
        string | undefined
    >("natural");
    const [name, setName] = React.useState("");
    const [contactName, setContactName] = React.useState("");
    const [email, setEmail] = React.useState("");
    const [phoneNumber, setPhoneNumber] = React.useState("");
    const [billingAddress, setBillingAddress] = React.useState<
        AddressInterface | null | undefined
    >(defaultAddress());
    const [mailingAddress, setMailingAddress] = React.useState<
        AddressInterface | null | undefined
    >(undefined);
    const dispatch = useAppDispatch();
    const navigate = useNavigate();
    const { errors, setErrors, unexpectedError } =
        useFormErrorV2<OwnerProfileErrors>();

    useEffect(() => {
        dispatch(get()).then(async (response) => {
            if (get.fulfilled.match(response)) {
                if (response.payload.statusCode === 200) {
                    if (
                        typeof response.payload.jsonData?.data !== "undefined"
                    ) {
                        const data = response.payload.jsonData.data;

                        setOwnerProfileType(data.customer_type);
                        setName(data.name);
                        setContactName(data.contact_name ?? "");
                        setEmail(data.email);
                        setPhoneNumber(data.phone_number ?? "");
                        setBillingAddress(
                            // NOTE: this is needed to show address fields on update for
                            // customers created before address was added to the system.
                            data.billing_address === null
                                ? defaultAddress()
                                : normalizeAddress(data.billing_address),
                        );
                        setMailingAddress(
                            normalizeAddress(data.mailing_address),
                        );
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
    }, [dispatch, setErrors, unexpectedError]);

    const handleUpdate = (e: React.SubmitEvent) => {
        e.preventDefault();
        dispatch(
            update({
                name,
                contactName,
                email,
                phoneNumber,
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

    const handleNeedMailingAddressChange = () => {
        if (mailingAddress === null || typeof mailingAddress === "undefined") {
            setMailingAddress(defaultAddress());
        } else {
            setMailingAddress(undefined);
        }
    };

    return (
        <>
            <GlobalError error={errors} />
            <form
                onSubmit={handleUpdate}
                className="space-y-4"
                autoComplete={"off"}
            >
                <FieldSet>
                    <FieldGroup>
                        <Field
                            data-invalid={
                                typeof errors?.fields?.owner_profile_type ===
                                "string"
                            }
                        >
                            <FieldLabel htmlFor="owner_profile_type">
                                Típus
                            </FieldLabel>
                            <Select
                                value={ownerProfileType}
                                onValueChange={(val) => {
                                    setErrors((prev) => {
                                        if (!prev?.fields) return prev;

                                        return {
                                            ...prev,
                                            fields: {
                                                ...prev.fields,
                                                owner_profile_type: null,
                                            },
                                        };
                                    });
                                    setOwnerProfileType(val);
                                }}
                            >
                                <SelectTrigger
                                    className={"w-full"}
                                    aria-invalid={
                                        typeof errors?.fields
                                            ?.owner_profile_type === "string"
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
                                error={errors?.fields?.owner_profile_type}
                            />
                        </Field>
                        <Field
                            data-invalid={
                                typeof errors?.fields?.name === "string"
                            }
                        >
                            <FieldLabel htmlFor="name">
                                {ownerProfileType === "legal"
                                    ? "Jogi személy neve"
                                    : "Név"}
                            </FieldLabel>
                            <Input
                                id="name"
                                type="text"
                                placeholder={
                                    ownerProfileType === "legal"
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
                        {ownerProfileType === "legal" ? (
                            <>
                                <Field
                                    data-invalid={
                                        typeof errors?.fields?.contact_name ===
                                        "string"
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
                                                if (!prev?.fields) return prev;

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
                                {ownerProfileType === "legal"
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
                                    typeof errors?.fields?.email === "string"
                                }
                            />
                            <FieldErrorV2 error={errors?.fields?.email} />
                        </Field>
                        <Field
                            data-invalid={
                                typeof errors?.fields?.phone_number === "string"
                            }
                        >
                            <FieldLabel htmlFor="phone_number">
                                {ownerProfileType === "legal"
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
                    errors={errors?.fields?.mailing_address}
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
                        <Button type="submit">{"Mentés"}</Button>
                    </div>
                </Field>
            </form>
        </>
    );
}
