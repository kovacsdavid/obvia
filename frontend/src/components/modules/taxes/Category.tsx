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

import { Badge } from "@/components/ui/badge";

interface TaxCategoryProps {
    taxCategory: string;
}

const TAX_CATEGORY_LABELS: Record<string, string> = {
    AK: "Általános adókulcs",
    AAM: "Alanyi adómentes",
    TAM: "Tárgyi adómentes",
    KBAET: "Közösségen belüli adómentes termékértékesítés",
    KBAUK: "Új közlekedési eszköz Közösségen belüli értékesítése",
    EAM: "Adómentes termékexport (harmadik országba)",
    NAM: "Nemzetközi ügyletekhez kapcsolódó adómentesség",
    ATK: "Áfa tárgyi hatályán kívüli",
    EUFAD37: "Másik tagállamban teljesített szolgáltatás (37. § (1))",
    EUFADE: "Másik tagállamban teljesített, egyéb fordítottan adózó ügylet",
    EUE: "Másik tagállamban teljesített, nem fordítottan adózó ügylet",
    HO: "Harmadik országban teljesített ügylet",
    REFUNDABLE_VAT: "Áfát a kedvezményezett megtéríti",
    NONREFUNDABLE_VAT: "Áfát a kedvezményezett nem téríti meg",
};

export default function TaxCategory({ taxCategory }: TaxCategoryProps) {
    const label = TAX_CATEGORY_LABELS[taxCategory];

    if (!label) {
        return <Badge variant="destructive">Ismeretlen adókategória</Badge>;
    }

    return (
        <Badge
            title={label}
            className="bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-300"
        >
            {taxCategory}
        </Badge>
    );
}
