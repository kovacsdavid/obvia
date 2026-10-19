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

import {
    Boxes,
    ClipboardList,
    Database,
    HandCoins,
    MessageSquareText,
    Package,
    Printer,
    ShieldCheck,
    UsersRound,
    Warehouse,
    type LucideIcon,
} from "lucide-react";

export type LandingFeatureStatus = "available" | "beta" | "coming_soon";

export interface LandingFeature {
    id: string;
    title: string;
    description: string;
    icon: LucideIcon;
    status?: LandingFeatureStatus;
}

export const FEATURE_STATUS_LABELS: Record<LandingFeatureStatus, string> = {
    available: "Elérhető",
    beta: "Béta",
    coming_soon: "Hamarosan",
};

export const LANDING_FEATURES: readonly LandingFeature[] = [
    {
        id: "multi-tenant",
        title: "Több szervezet, külön adatbázis",
        description:
            "Minden szervezet saját, elkülönített adatbázist kap. Az adatok így biztonságosan elválnak egymástól.",
        icon: Database,
    },
    {
        id: "auth",
        title: "Biztonságos hitelesítés",
        description:
            "Regisztráció e-mail megerősítéssel, elfelejtett jelszó visszaállítása és kétlépcsős azonosítás (OTP).",
        icon: ShieldCheck,
    },
    {
        id: "customers",
        title: "Vevők",
        description: "Vevők és elérhetőségeik egy helyen, gyorsan kereshetően.",
        icon: UsersRound,
    },
    {
        id: "products-services",
        title: "Termékek és szolgáltatások",
        description: "Kezeld termékeidet és szolgáltatásaidat egy helyen.",
        icon: Package,
    },
    {
        id: "inventory",
        title: "Raktárak és készletkezelés",
        description:
            "Több raktár, készletszintek, készletmozgások és foglalások nyomon követése.",
        icon: Warehouse,
    },
    {
        id: "taxes",
        title: "Adók",
        description:
            "Országonkénti adókulcsok és adókategóriák kezelése, alapértelmezett adó beállításával.",
        icon: HandCoins,
    },
    {
        id: "tasks-worksheets",
        title: "Feladatok és munkalapok",
        description:
            "Munkalapok feladatokkal és a hozzájuk tartozó anyagmozgásokkal.",
        icon: ClipboardList,
    },
    {
        id: "comments-activity",
        title: "Megjegyzések és tevékenységnapló",
        description:
            "Megjegyzések fűzése a rekordokhoz, a változások pedig visszakereshetők a tevékenységnaplóban.",
        icon: MessageSquareText,
    },
    {
        id: "pdf-print",
        title: "PDF nyomtatás",
        description:
            "A rekordok egy kattintással nyomtatható PDF-be exportálhatók.",
        icon: Printer,
    },
    {
        id: "modular",
        title: "Folyamatosan bővülő modulok",
        description: "Új modulok folyamatosan érkeznek a rendszerbe.",
        icon: Boxes,
        status: "coming_soon",
    },
];
