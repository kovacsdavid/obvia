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

interface MovementTypeProps {
    movementType: string;
}

export default function MovementType({ movementType }: MovementTypeProps) {
    switch (movementType) {
        case "in":
            return (
                <Badge className="bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-300">
                    Bevétel
                </Badge>
            );
        case "out":
            return (
                <Badge className="bg-red-50 text-red-700 dark:bg-red-950 dark:text-red-300">
                    Kiadás
                </Badge>
            );
        default:
            return <Badge variant="destructive">N/A</Badge>;
    }
}
