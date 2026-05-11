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

import { Link } from "lucide-react";
import { NavLink } from "react-router";
import { useMemo } from "react";

interface ReferenceIdProps {
    reference_id: string | null;
    reference_type: string | null;
}

export default function ReferenceId({
    reference_id,
    reference_type,
}: ReferenceIdProps) {
    const referenceUrl = useMemo(() => {
        if (reference_id === null) {
            return;
        }
        switch (reference_type) {
            case "worksheets":
                return `/munkalap/reszletek/${reference_id}`;
            default:
                return;
        }
    }, [reference_id, reference_type]);

    if (!referenceUrl) {
        return reference_id;
    } else {
        return (
            <NavLink to={referenceUrl}>
                {reference_id} <Link size={15} className="inline" />
            </NavLink>
        );
    }
}
