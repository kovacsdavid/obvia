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

import type { ReactNode } from "react";
import { Footer } from "./Footer";
import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Separator } from "@/components/ui/separator";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { useLocation } from "react-router-dom";
import { Alert, AlertDescription } from "@/components/ui";
import { FlaskConical } from "lucide-react";

export function Layout({ children }: { children: ReactNode }) {
  const breadcrumbMap: Record<string, string> = {
    "/bejelentkezes": "Bejelentkezés",
    "/regisztracio": "Regisztráció",
    "/vezerlopult": "Vezérlőpult",
    "/adatbazis/letrehozas": "Adatbázis létrehozása",
    "/adatbazis/modositas": "Adatbázis módosítás",
    "/adatbazis/lista": "Adatbázis lista",
    "/adatbazis/reszletek": "Adatbázis részletek",
    "/vevo/letrehozas": "Vevő létrehozása",
    "/vevo/modositas": "Vevő módosítás",
    "/vevo/lista": "Vevő lista",
    "/vevo/reszletek": "Vevő részletek",
    "/raktarkeszlet/letrehozas": "Raktárkészlet létrehozása",
    "/raktarkeszlet/modositas": "Raktárkészlet módosítás",
    "/raktarkeszlet/lista": "Raktárkészlet lista",
    "/raktarkeszlet/reszletek": "Raktárkészlet részletek",
    "/raktarkeszlet-mozgas/letrehozas": "Készletmozgás létrehozása",
    "/raktarkeszlet-mozgas/modositas": "Készletmozgás módosítás",
    "/raktarkeszlet-mozgas/lista": "Készletmozgás lista",
    "/raktarkeszlet-mozgas/reszletek": "Készletmozgás részletek",
    "/raktarkeszlet-foglalas/letrehozas": "Készletfoglalás létrehozása",
    "/raktarkeszlet-foglalas/modositas": "Készletfoglalás módosítás",
    "/raktarkeszlet-foglalas/lista": "Készletfoglalás lista",
    "/raktarkeszlet-foglalas/reszletek": "Készletfoglalás részletek",
    "/termek/letrehozas": "Termék létrehozása",
    "/termek/modositas": "Termék módosítás",
    "/termek/lista": "Termék lista",
    "/termek/reszletek": "Termék részletek",
    "/projekt/letrehozas": "Projekt létrehozása",
    "/projekt/modositas": "Projekt módosítás",
    "/projekt/lista": "Projekt lista",
    "/projekt/reszletek": "Projekt részletek",
    "/cimke/letrehozas": "Címke létrehozása",
    "/cimke/modositas": "Címke módosítás",
    "/cimke/lista": "Címke lista",
    "/cimke/reszletek": "Címke részletek",
    "/feladat/letrehozas": "Feladat létrehozása",
    "/feladat/modositas": "Feladat módosítás",
    "/feladat/lista": "Feladat lista",
    "/feladat/reszletek": "Feladat részletek",
    "/felhasznalo/letrehozas": "Felhasználó létrehozása",
    "/felhasznalo/modositas": "Felhasználó módosítás",
    "/felhasznalo/lista": "Felhasználó lista",
    "/felhasznalo/reszletek": "Felhasználó részletek",
    "/raktar/letrehozas": "Raktár létrehozása",
    "/raktar/modositas": "Raktár módosítás",
    "/raktar/lista": "Raktár lista",
    "/raktar/reszletek": "Raktár részletek",
    "/szolgaltatas/letrehozas": "Szolgáltatás létrehozása",
    "/szolgaltatas/modositas": "Szolgáltatás módosítás",
    "/szolgaltatas/lista": "Szolgáltatás lista",
    "/szolgaltatas/reszletek": "Szolgáltatás részletek",
    "/munkalap/letrehozas": "Munkalap létrehozása",
    "/munkalap/modositas": "Munkalap módosítás",
    "/munkalap/lista": "Munkalap lista",
    "/munkalap/reszletek": "Munkalap részletek",
    "/ado/letrehozas": "Adó létrehozása",
    "/ado/modositas": "Adó módosítás",
    "/ado/lista": "Adó lista",
    "/ado/reszletek": "Adó részletek",
    "/": "Kezdőoldal",
  };
  const { pathname } = useLocation();
  let currentTitle = "";
  const match = Object.keys(breadcrumbMap).filter((key) => {
    return pathname.includes(key);
  });
  if (match.length > 0) {
    currentTitle = breadcrumbMap[match[0]];
  }
  return (
    <SidebarProvider>
      <AppSidebar />
      <SidebarInset>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1" />
          <Separator
            orientation="vertical"
            className="mr-2 data-[orientation=vertical]:h-4"
          />
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem className="hidden md:block">
                <BreadcrumbLink href="#">obvia</BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator className="hidden md:block" />
              <BreadcrumbItem>
                <BreadcrumbPage>{currentTitle}</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </header>
        <Alert className={"mr-auto ml-auto mt-3 max-w-2xl"}>
          <FlaskConical color={"orange"} />
          <AlertDescription className={"text-orange-400"}>
            Zárt béta verzió: A rendszer jelenleg zártkörű tesztelési fázisban
            van. A tárolt adatok bármikor törlésre kerülhetnek. Ne használd a
            rendszert valós adatokkal!
          </AlertDescription>
        </Alert>
        <div className="flex flex-1 flex-col gap-4 p-4">
          <main className="flex-1 container mx-auto px-4 py-6">{children}</main>
          <Footer />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
