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

import { type ReactNode } from "react";
import { Separator } from "@/components/ui/separator";

type SettingsSectionProps = {
    title: string;
    description?: string;
    children: ReactNode;
};

export default function SettingsSection({
    title,
    description,
    children,
}: SettingsSectionProps) {
    return (
        <section className="space-y-4">
            <div className="space-y-1">
                <h3 className="text-lg font-medium">{title}</h3>
                {description ? (
                    <p className="text-sm text-muted-foreground">
                        {description}
                    </p>
                ) : null}
            </div>

            <div className="mt-5 mb-8">{children}</div>

            <Separator className="mt-6" />
        </section>
    );
}
