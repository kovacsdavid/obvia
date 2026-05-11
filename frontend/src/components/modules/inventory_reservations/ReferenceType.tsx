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

interface ReferenceTypeProps {
    reference_type: string | null;
}

export default function ReferenceType({ reference_type }: ReferenceTypeProps) {
    switch (reference_type) {
        case "worksheets":
            return <Badge variant="secondary">Munkalap</Badge>;
        default:
            return (
                <Badge variant="destructive">Ismeretlen referencia típus</Badge>
            );
    }
}
