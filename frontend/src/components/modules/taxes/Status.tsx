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

interface StatusProps {
    status: string;
}

export default function Status({ status }: StatusProps) {
    switch (status) {
        case "active":
            return (
                <Badge className="bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-300">
                    Aktív
                </Badge>
            );
        case "draft":
            return (
                <Badge className="bg-purple-50 text-purple-700 dark:bg-purple-950 dark:text-purple-300">
                    Vázlat
                </Badge>
            );
        case "inactive":
            return <Badge variant="secondary">Inaktív</Badge>;
        default:
            return <Badge variant="destructive">Ismeretlen státusz</Badge>;
    }
}
