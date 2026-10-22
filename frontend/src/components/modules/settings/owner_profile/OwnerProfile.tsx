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

import React, { useCallback, useEffect, useMemo } from "react";
import { Button, Checkbox, FieldErrorV2, Input } from "@/components/ui";
import { useAppDispatch } from "@/store/hooks.ts";
import {
    get_full,
    update,
} from "@/components/modules/settings/owner_profile/lib/slice.ts";
import {
    type OwnerProfileErrors,
    type OwnerProfileFull,
} from "@/components/modules/settings/owner_profile/lib/interface";
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
import {
    InputGroup,
    InputGroupAddon,
    InputGroupInput,
    InputGroupText,
} from "@/components/ui/input-group";
import { toast } from "@/components/ui/toast-lib";
import { handleApiResult } from "@/lib/api/handleApiResult";

export default function OwnerProfile() {
    const [id, setId] = React.useState<string>("");
    const [ownerProfileType, setOwnerProfileType] = React.useState<
        string | undefined
    >("natural");
    const [name, setName] = React.useState("");
    const [contactName, setContactName] = React.useState("");
    const [email, setEmail] = React.useState("");
    const [website, setWebsite] = React.useState("");
    const [documentPrefix, setDocumentPrefix] = React.useState("");
    const [phoneNumber, setPhoneNumber] = React.useState("");
    const [billingAddress, setBillingAddress] = React.useState<
        AddressInterface | null | undefined
    >(defaultAddress());
    const [mailingAddress, setMailingAddress] = React.useState<
        AddressInterface | null | undefined
    >(undefined);
    const dispatch = useAppDispatch();
    const [errors, setErrors] = React.useState<OwnerProfileErrors | null>(null);

    const WEBSITE_PREFIX = "https://";

    // Strips any protocol the user types or pastes (http://, https://)
    const stripProtocol = (value: string) =>
        value.trim().replace(/^https?:\/\//i, "");

    const setData = useCallback((data: OwnerProfileFull | undefined) => {
        if (typeof data === "undefined") return;
        setId(data.id);
        setOwnerProfileType(data.owner_profile_type);
        setName(data.name);
        setContactName(data.contact_name ?? "");
        setEmail(data.email);
        setWebsite(data.website ?? "");
        setDocumentPrefix(data.document_prefix ?? "");
        setPhoneNumber(data.phone_number ?? "");
        setBillingAddress(
            data.billing_address === null
                ? defaultAddress()
                : normalizeAddress(data.billing_address),
        );
        setMailingAddress(normalizeAddress(data.mailing_address));
    }, []);

    useEffect(() => {
        const promise = dispatch(get_full());

        promise.then((action) => {
            if (!get_full.fulfilled.match(action)) return;

            const result = action.payload;
            if (result.kind === "success") {
                setData(result.data.data);
            }
        });

        return () => promise.abort();
    }, [dispatch, setData]);

    const handleUpdate = async (e: React.SubmitEvent) => {
        e.preventDefault();
        setErrors(null);

        const result = await dispatch(
            update({
                input: {
                    id,
                    name,
                    contactName,
                    email,
                    website,
                    documentPrefix,
                    phoneNumber,
                    ownerProfileType,
                    billingAddress,
                    mailingAddress,
                },
            }),
        ).unwrap();

        handleApiResult(result, {
            onSuccess: ({ data }) => {
                setData(data);
                toast.add({
                    type: "success",
                    title: "A mentés sikeresen megtörtént!",
                });
            },
            onFieldErrors: setErrors,
        });
    };

    const handleNeedMailingAddressChange = () => {
        if (mailingAddress === null || typeof mailingAddress === "undefined") {
            setMailingAddress(defaultAddress());
        } else {
            setMailingAddress(undefined);
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
        keyof OwnerProfileErrors,
        "billing_address" | "mailing_address"
    >;

    const clearError = useCallback((field: FlatErrorKey) => {
        setErrors((prev) =>
            prev && prev[field] != null ? { ...prev, [field]: null } : prev,
        );
    }, []);

    return (
        <>
            <form
                onSubmit={handleUpdate}
                className="space-y-4"
                autoComplete={"off"}
            >
                <FieldSet>
                    <FieldGroup>
                        <Field
                            data-invalid={
                                typeof errors?.owner_profile_type === "string"
                            }
                        >
                            <FieldLabel htmlFor="owner_profile_type">
                                Típus
                            </FieldLabel>
                            <Select
                                value={ownerProfileType}
                                onValueChange={(val) => {
                                    clearError("owner_profile_type");
                                    setOwnerProfileType(val);
                                }}
                            >
                                <SelectTrigger
                                    id="owner_profile_type"
                                    className={"w-full"}
                                    aria-invalid={
                                        typeof errors?.owner_profile_type ===
                                        "string"
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
                            <FieldErrorV2 error={errors?.owner_profile_type} />
                        </Field>
                        <Field data-invalid={typeof errors?.name === "string"}>
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
                                    clearError("name");
                                    setName(e.target.value);
                                }}
                                aria-invalid={typeof errors?.name === "string"}
                            />
                            <FieldErrorV2 error={errors?.name} />
                        </Field>
                        {ownerProfileType === "legal" ? (
                            <>
                                <Field
                                    data-invalid={
                                        typeof errors?.contact_name === "string"
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
                                            clearError("contact_name");
                                            setContactName(e.target.value);
                                        }}
                                        aria-invalid={
                                            typeof errors?.contact_name ===
                                            "string"
                                        }
                                    />

                                    <FieldErrorV2
                                        error={errors?.contact_name}
                                    />
                                </Field>
                            </>
                        ) : null}
                        <Field data-invalid={typeof errors?.email === "string"}>
                            <FieldLabel htmlFor="email">
                                {ownerProfileType === "legal"
                                    ? "Kapcsolattartó e-mail címe"
                                    : "E-mail cím"}
                            </FieldLabel>
                            <Input
                                id="email"
                                type="text"
                                placeholder="pelda@obvia.hu"
                                value={email}
                                onChange={(e) => {
                                    clearError("email");
                                    setEmail(e.target.value);
                                }}
                                aria-invalid={typeof errors?.email === "string"}
                            />
                            <FieldErrorV2 error={errors?.email} />
                        </Field>
                        <Field
                            data-invalid={typeof errors?.website === "string"}
                        >
                            <FieldLabel htmlFor="website">Weboldal</FieldLabel>
                            <InputGroup>
                                <InputGroupAddon>
                                    <InputGroupText>
                                        {WEBSITE_PREFIX}
                                    </InputGroupText>
                                </InputGroupAddon>
                                <InputGroupInput
                                    id="website"
                                    type="text"
                                    inputMode="url"
                                    autoComplete="url"
                                    placeholder=""
                                    className="!pl-0.5"
                                    value={stripProtocol(website)}
                                    onChange={(e) => {
                                        clearError("website");
                                        const domain = stripProtocol(
                                            e.target.value,
                                        );
                                        setWebsite(
                                            domain
                                                ? `${WEBSITE_PREFIX}${domain}`
                                                : "",
                                        );
                                    }}
                                    aria-invalid={
                                        typeof errors?.website === "string"
                                    }
                                />
                            </InputGroup>{" "}
                            <FieldErrorV2 error={errors?.website} />
                        </Field>
                        <Field
                            data-invalid={
                                typeof errors?.document_prefix === "string"
                            }
                        >
                            <FieldLabel htmlFor="document_prefix">
                                Dokumentum előtag
                            </FieldLabel>
                            <Input
                                id="document_prefix"
                                type="text"
                                placeholder="KD"
                                value={documentPrefix}
                                onChange={(e) => {
                                    clearError("document_prefix");
                                    setDocumentPrefix(e.target.value);
                                }}
                                aria-invalid={
                                    typeof errors?.document_prefix === "string"
                                }
                            />
                            <FieldErrorV2 error={errors?.document_prefix} />
                        </Field>
                        <Field
                            data-invalid={
                                typeof errors?.phone_number === "string"
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
                                    clearError("phone_number");
                                    setPhoneNumber(e.target.value);
                                }}
                                aria-invalid={
                                    typeof errors?.phone_number === "string"
                                }
                            />
                            <FieldErrorV2 error={errors?.phone_number} />
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
                        <Button type="submit">{"Mentés"}</Button>
                    </div>
                </Field>
            </form>
        </>
    );
}
