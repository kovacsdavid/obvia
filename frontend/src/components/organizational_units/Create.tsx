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
import {useAppDispatch, useAppSelector} from "@/store/hooks.ts";
import {Button, Input, Label} from "@/components/ui";
import type {RootState} from "@/store";
import {create} from "@/store/slices/organizational_unit.ts";

export default function Create() {
    const [name, setName] = React.useState("");
    const [dbHost, setDbHost] = React.useState("");
    const [dbPort, setDbPort] = React.useState("");
    const [dbName, setDbName] = React.useState("");
    const [dbUser, setDbUser] = React.useState("");
    const [dbPassword, setDbPassword] = React.useState("");
    const dispatch = useAppDispatch();
    const error = useAppSelector(
        (state: RootState) => state.organizationalUnits.error
    );
    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        const dbPortNumber = parseInt(dbPort, 10);
        dispatch(create({
            name,
            dbHost,
            dbPort: dbPortNumber,
            dbName,
            dbUser,
            dbPassword
        })).then((response) => {
            console.log(response)
            if (response?.meta?.requestStatus === "fulfilled") {
                window.location.href = "/szervezeti_egysegek/lista";
            }
        });
    };

    const placeholder = "A funkció jelenleg még nem elérhető";

    return (
        <form onSubmit={handleSubmit} className="max-w-sm mx-auto mt-20 space-y-4">
            <Label htmlFor="name">Szervezeti egység neve</Label>
            <Input
                id="name"
                type="text"
                value={name}
                onChange={e => setName(e.target.value)}
            />
            {error?.fields?.name && <div className="text-red-600">{error.fields.name}</div>}
            <Label htmlFor="db_host">Adatbázis kiszolgáló</Label>
            <Input
                id="db_host"
                type="text"
                placeholder={placeholder}
                disabled={true}
                value={dbHost}
                onChange={e => setDbHost(e.target.value)}
            />
            {error?.fields?.db_host && <div className="text-red-600">{error.fields.db_host}</div>}
            <Label htmlFor="db_port">Adatbázis port</Label>
            <Input
                id="db_port"
                type="text"
                placeholder={placeholder}
                disabled={true}
                value={dbPort}
                onChange={e => setDbPort(e.target.value)}
            />
            {error?.fields?.db_port && <div className="text-red-600">{error.fields.db_port}</div>}
            <Label htmlFor="db_name">Adatbázis név</Label>
            <Input
                id="db_name"
                type="text"
                placeholder={placeholder}
                disabled={true}
                value={dbName}
                onChange={e => setDbName(e.target.value)}
            />
            {error?.fields?.db_name && <div className="text-red-600">{error.fields.db_name}</div>}
            <Label htmlFor="db_user">Adatbázis felhasználó</Label>
            <Input
                id="db_user"
                type="text"
                placeholder={placeholder}
                disabled={true}
                value={dbUser}
                onChange={e => setDbUser(e.target.value)}
            />
            {error?.fields?.db_user && <div className="text-red-600">{error.fields.db_user}</div>}
            <Label htmlFor="db_password">Adatbázis jelszó</Label>
            <Input
                id="db_password"
                type="text"
                placeholder={placeholder}
                disabled={true}
                value={dbPassword}
                onChange={e => setDbPassword(e.target.value)}
            />
            {error?.fields?.db_password && <div className="text-red-600">{error.fields.db_password}</div>}
            <Button type="submit">Létrehozás</Button>
            {error?.global && <div className="text-red-600">{error.global}</div>}
        </form>
    );
}