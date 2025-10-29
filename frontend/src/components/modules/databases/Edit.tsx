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
  Alert,
  AlertDescription,
  AlertTitle,
  Button,
  Checkbox,
  FieldError,
  GlobalError,
  Input,
  Label
} from "@/components/ui";
import {create} from "@/components/modules/databases/lib/slice.ts";
import {AlertCircle, Terminal} from "lucide-react";
import {useActivateDatabase} from "@/hooks/use_activate_database.ts";
import {useFormError} from "@/hooks/use_form_error.ts";
import {useNavigate} from "react-router-dom";
import {ConditionalCard} from "@/components/ui/card.tsx";
import {useParams} from "react-router";
import type {Database} from "@/components/modules/databases/lib/interface.ts";

interface EditProps {
  showCard?: boolean;
  onSuccess?: (database: Database) => void;
}

export default function Edit({showCard = true, onSuccess = undefined}: EditProps) {
  const [name, setName] = React.useState("");
  const [dbIsSelfHosted, setDbIsSelfHosted] = React.useState<boolean | "indeterminate">(false);
  const [dbHost, setDbHost] = React.useState("");
  const [dbPort, setDbPort] = React.useState("");
  const [dbName, setDbName] = React.useState("");
  const [dbUser, setDbUser] = React.useState("");
  const [dbPassword, setDbPassword] = React.useState("");
  const dispatch = useAppDispatch();
  const navigate = useNavigate();
  const {errors, setErrors, unexpectedError} = useFormError();
  const params = useParams();
  const id = React.useMemo(() => params["id"] ?? null, [params]);

  const activateDatabase = useActivateDatabase();

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
      if (create.fulfilled.match(response)) {
        if (response.payload.statusCode === 201) {
          if (typeof response.payload.jsonData.data !== "undefined") {
            activateDatabase(response.payload.jsonData.data.id).then((isOk) => {
              if (isOk) {
                if (
                  typeof onSuccess === "function"
                  && typeof response.payload.jsonData.data !== "undefined"
                ) {
                  onSuccess(response.payload.jsonData.data);
                } else {
                  navigate("/adatbazis/lista");
                }
              } else {
                unexpectedError();
              }
            })
          }
        } else if (typeof response.payload.jsonData?.error !== "undefined") {
          setErrors(response.payload.jsonData.error)
        } else {
          unexpectedError();
        }
      } else {
        unexpectedError();
      }
    });
  };

  return (
    <>
      <GlobalError error={errors}/>
      <ConditionalCard
        showCard={showCard}
        title={`Adatbázis ${id ? "módosítás" : "létrehozás"}`}
        className={"max-w-lg mx-auto"}
      >
        <form onSubmit={handleSubmit} className="max-w-sm mx-auto space-y-4" autoComplete={"off"}>
          <Label htmlFor="name">Adatbázis neve</Label>
          <Input
            id="name"
            type="text"
            value={name}
            onChange={e => setName(e.target.value)}
          />
          <FieldError error={errors} field={"name"}/>
          <div className="flex items-start gap-3 mt-7 mb-5">
            <Checkbox id="self_hosted_db" checked={dbIsSelfHosted} onCheckedChange={setDbIsSelfHosted}/>
            <Label htmlFor="self_hosted_db">Saját adatbázis kiszolgálót használok (haladó)</Label>
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
                  Ügyeljen arra, hogy kifejezetten erre a célra létrehozott adatbázis felhasználót adjon meg, mely
                  csak a
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
                  Amennyiben a rendszer sikeresen csatlakozni tud a megadott adatbázishoz az adatszerkezet
                  kialakítását
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
                  Biztonsági okokból a rendszerhez hozzáadott adatbázis nevének tartalmaznia kell a "tenant_"
                  előtagot.
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
                  illetve számokat tartalmazhat <br/>(Ezt a jelszót, amíg az adatbázis hozzá van rendelve a
                  rendszerhez
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
          <Button type="submit">{id ? "Módosítás" : "Létrehozás"}</Button>
        </form>
      </ConditionalCard>
    </>
  );
}
