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

import type {ReactNode} from "react";
import {Footer} from "./Footer";
import {AppSidebar} from "@/components/app-sidebar"
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb"
import {Separator} from "@/components/ui/separator"
import {SidebarInset, SidebarProvider, SidebarTrigger,} from "@/components/ui/sidebar"
import {useLocation} from "react-router-dom";
import {Alert, AlertDescription} from "@/components/ui";
import {FlaskConical} from "lucide-react";

export function Layout({children}: { children: ReactNode }) {
  const breadcrumbMap: Record<string, string> = {
    "/": "Kezdőoldal",
    "/bejelentkezes": "Bejelentkezés",
    "/regisztracio": "Regisztráció",
    "/vezerlopult": "Vezérlőpult",
    "/adatbazis/szerkesztes": "Adatbázis létrehozása",
    "/adatbazis/lista": "Adatbázis lista",
    "/vevo/szerkesztes": "Vevő létrehozása",
    "/vevo/lista": "Vevő lista",
    "/leltar/szerkesztes": "Leltár létrehozása",
    "/leltar/lista": "Leltár lista",
    "/termek/szerkesztes": "Termék létrehozása",
    "/termek/lista": "Termék lista",
    "/projekt/szerkesztes": "Projekt létrehozása",
    "/projekt/lista": "Projekt lista",
    "/cimke/szerkesztes": "Címke létrehozása",
    "/cimke/lista": "Címke lista",
    "/feladat/szerkesztes": "Feladat létrehozása",
    "/feladat/lista": "Feladat lista",
    "/felhasznalo/szerkesztes": "Felhasználó létrehozása",
    "/felhasznalo/lista": "Felhasználó lista",
    "/raktar/szerkesztes": "Raktár létrehozása",
    "/raktar/lista": "Raktár lista",
    "/munkalap/szerkesztes": "Munkalap létrehozása",
    "/munkalap/lista": "Munkalap lista",
  };
  const {pathname} = useLocation();
  const currentTitle = breadcrumbMap[pathname] || "Ismeretlen oldal";
  return (
    <SidebarProvider>
      <AppSidebar/>
      <SidebarInset>
        <Alert className={"mr-auto ml-auto mt-5 mb-5 max-w-7xl"}>
          <FlaskConical color={"orange"}/>
          <AlertDescription className={"text-orange-400"}>
            Zárt béta verzió: A rendszer jelenleg zártkörű tesztelési fázisban van. A tárolt adatok bármikor törlésre
            kerülhetnek. Ne használd a rendszert valós adatokkal!
          </AlertDescription>
        </Alert>
        <header className="flex h-16 shrink-0 items-center gap-2 border-b px-4">
          <SidebarTrigger className="-ml-1"/>
          <Separator
            orientation="vertical"
            className="mr-2 data-[orientation=vertical]:h-4"
          />
          <Breadcrumb>
            <BreadcrumbList>
              <BreadcrumbItem className="hidden md:block">
                <BreadcrumbLink href="#">
                  obvia
                </BreadcrumbLink>
              </BreadcrumbItem>
              <BreadcrumbSeparator className="hidden md:block"/>
              <BreadcrumbItem>
                <BreadcrumbPage>{currentTitle}</BreadcrumbPage>
              </BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
        </header>
        <div className="flex flex-1 flex-col gap-4 p-4">
          <main className="flex-1 container mx-auto px-4 py-6">{children}</main>
          <Footer/>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
