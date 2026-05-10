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

interface PriorityProps {
    priority: string;
}

export default function Priority({ priority }: PriorityProps) {
    switch (priority) {
        case "low":
            return (
                <Badge className="bg-gray-50 text-gray-700 dark:bg-gray-950 dark:text-gray-300">
                    Alacsony
                </Badge>
            );
        case "normal":
            return (
                <Badge className="bg-blue-50 text-blue-700 dark:bg-blue-950 dark:text-blue-300">
                    Normál
                </Badge>
            );
        case "high":
            return (
                <Badge className="bg-red-50 text-red-700 dark:bg-red-950 dark:text-red-300">
                    Magas
                </Badge>
            );
        default:
            return <Badge variant="destructive">Ismeretlen státusz</Badge>;
    }
}
