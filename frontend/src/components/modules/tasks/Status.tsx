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
        case "in_progress":
            return (
                <Badge className="bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-300">
                    Folyamatban
                </Badge>
            );
        case "waiting_for_check":
            return (
                <Badge className="bg-yellow-50 text-yellow-700 dark:bg-yellow-950 dark:text-yellow-300">
                    Bevizsgálásra vár
                </Badge>
            );
        case "waiting_for_supply":
            return (
                <Badge className="bg-orange-50 text-orange-700 dark:bg-orange-950 dark:text-orange-300">
                    Beszerzésre vár
                </Badge>
            );
        case "waiting_for_feedback":
            return (
                <Badge className="bg-purple-50 text-purple-700 dark:bg-purple-950 dark:text-purple-300">
                    Visszajelzésre vár
                </Badge>
            );
        case "ready_for_customer_pickup":
            return (
                <Badge className="bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-300">
                    Átvehető
                </Badge>
            );
        case "closed":
            return <Badge variant="secondary">Lezárt</Badge>;
        default:
            return <Badge variant="destructive">Ismeretlen státusz</Badge>;
    }
}
