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
import {useAppDispatch} from "@/store/hooks.ts";
import {
  Button,
  Input,
  Label,
  Checkbox,
  Alert,
  AlertTitle,
  AlertDescription,
  FieldError,
  GlobalError
} from "@/components/ui";
import {create} from "@/components/tenants/slice.ts";
import {AlertCircle, Terminal} from "lucide-react";
import { type FormError } from "@/lib/interfaces/common.ts";

export default function Create() {
  const [name, setName] = React.useState("");
  const [dbIsSelfHosted, setDbIsSelfHosted] = React.useState<boolean | "indeterminate">(false);
  const [dbHost, setDbHost] = React.useState("");
  const [dbPort, setDbPort] = React.useState("");
  const [dbName, setDbName] = React.useState("");
  const [dbUser, setDbUser] = React.useState("");
  const [dbPassword, setDbPassword] = React.useState("");
  const dispatch = useAppDispatch();
  const [errors, setErrors] = React.useState<FormError | null>(null);

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
    dispatch(create({
      name,
      dbIsSelfHosted: dbIsSelfHosted === true,
      dbHost,
      dbPort: dbPortNumber,
      dbName,
      dbUser,
      dbPassword
    })).then(async (response) => {
      if (response?.meta?.requestStatus === "fulfilled") {
        const payload = response.payload as Response;
        try {
          const responseData = await payload.json();
          switch (payload.status) {
            case 201:
              window.location.href = "/szervezeti_egyseg/lista";
              break;
            case 422:
              setErrors(responseData.error);
              break;
            default:
              setErrors({
                global: "Váratlan hiba történt a feldolgozás során!",
                fields: {}
              });
          }
        } catch {
          setErrors({
            global: "Váratlan hiba történt a feldolgozás során!",
            fields: {}
          });
        }
      }
    });
  };

  return (
    <>
      <GlobalError error={errors} />
      <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4">
        <Label htmlFor="name">Szervezeti egység neve</Label>
        <Input
          id="name"
          type="text"
          value={name}
          onChange={e => setName(e.target.value)}
        />
        <FieldError error={errors} field={"name"}/>
        <div className="flex items-start gap-3 mt-7 mb-5">
          <Checkbox id="self_hosted_db" checked={dbIsSelfHosted} onCheckedChange={setDbIsSelfHosted}/>
          <Label htmlFor="self_hosted_db">Saját adatbázist használok (haladó)</Label>
        </div>
        {dbIsSelfHosted === true && (
          <>
            <Alert variant="destructive">
              <AlertCircle/>
              <AlertTitle>Figyelmeztetés</AlertTitle>
              <AlertDescription>
                Felhívjuk figyelmét, hogy bár mindent elkövetünk a zavartalan működés érdekében a saját üzemeltetésű
                adatbázisokért nem tudunk semmilyen felelősséget vállalni!
                Az adatbázis rendszeres biztonsági mentéséről, monitorozásáról és az összes kapcsolódó karbantartáis
                feladatról
                a felhasználónak vagy az általa megbízott szakembernek kell gondoskodnia!
                Ügyeljen arra, hogy kifejezetten erre a célra létrehozott adatbázis felhasználót adjon meg, mely csak a
                kifejezetten erre a célra létrehozott üres adatbázis felett rendelkezik jogosultságokkal!
              </AlertDescription>
            </Alert>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Biztonságos kapcsolat</AlertTitle>
              <AlertDescription>
                Az adatbázisnak érvényes TLS tanusítvánnyal kell rendelkeznie az adatátvitel biztonságának megőrzése
                érdekében!
              </AlertDescription>
            </Alert>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Adatszerkezet kialakítása</AlertTitle>
              <AlertDescription>
                Amennyiben a rendszer sikeresen csatlakozni tud a megadott adatbázishoz az adatszerkezet kialakítását
                automatikusan elvégzi.
              </AlertDescription>
            </Alert>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Adatszerkezet karbantartása</AlertTitle>
              <AlertDescription>
                Ha egy verziófrissítés az adatszerkezet módosítását teszi szükségessé, akkor azt a rendszer
                automatikusan elvégzi.
              </AlertDescription>
            </Alert>
            <Label htmlFor="db_host">Adatbázis kiszolgáló</Label>
            <Input
              id="db_host"
              type="text"
              value={dbHost}
              onChange={e => setDbHost(e.target.value)}
            />
            <FieldError error={errors} field={"db_host"}/>
            <Label htmlFor="db_port">Adatbázis port</Label>
            <Input
              id="db_port"
              type="text"
              value={dbPort}
              onChange={e => setDbPort(e.target.value)}
            />
            <FieldError error={errors} field={"db_port"}/>
            <Label htmlFor="db_name">Adatbázis név</Label>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Előtag szükséges!</AlertTitle>
              <AlertDescription>
                Biztonsági okokból a rendszerhez hozzáadott adatbázis nevének tartalmaznia kell a "tenant_" előtagot.
                Kérem, hogy az adatbázist ennek megfelelően nevezze el!
              </AlertDescription>
            </Alert>
            <div className={"flex"}>
              <div className="flex items-center justify-center px-3 border border-r-0 rounded-l bg-gray-50">
                {"tenant_"}
              </div>
              <Input
                id="db_name"
                type="text"
                className="rounded-l-none"
                value={dbName}
                onChange={e => setDbName(e.target.value)}
              />
            </div>
            <FieldError error={errors} field={"db_name"}/>
            <Label htmlFor="db_user">Adatbázis felhasználó</Label>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Előtag szükséges!</AlertTitle>
              <AlertDescription>
                Biztonsági okokból a rendszerhez hozzáadott adatbázis felhasználónak tartalmaznia kell a "tenant_"
                előtagot.
                Kérem, hogy az adatbázis felhasznált ennek megfelelően nevezze el!
              </AlertDescription>
            </Alert>
            <div className={"flex"}>
              <div className="flex items-center justify-center px-3 border border-r-0 rounded-l bg-gray-50">
                {"tenant_"}
              </div>
              <Input
                id="db_user"
                type="text"
                value={dbUser}
                onChange={e => setDbUser(e.target.value)}
              />
            </div>
            <FieldError error={errors} field={"db_user"}/>
            <Label htmlFor="db_password">Adatbázis jelszó</Label>
            <Alert variant="default">
              <Terminal/>
              <AlertTitle>Jelszó formátum</AlertTitle>
              <AlertDescription>
                Biztonsági okokból a jelszó 40-99 karakter hosszú lehet és az angol abc kis és nagy betűit
                illetve számokat tartalmazhat <br/>(Ezt a jelszót, amíg az adatbázis hozzá van rendelve a rendszerhez
                nem kell
                újra megadni!)
              </AlertDescription>
            </Alert>
            <Input
              id="db_password"
              type="text"
              value={dbPassword}
              onChange={e => setDbPassword(e.target.value)}
            />
          </>
        )}
        <FieldError error={errors} field={"db_password"}/>
        <Button type="submit">Létrehozás</Button>
      </form>
    </>
  );
}