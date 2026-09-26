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
    Card,
    CardHeader,
    CardContent,
    CardTitle,
} from "@/components/ui/card.tsx";
import MfaSettings from "@/components/modules/settings/MfaSettings";
import SettingsSection from "@/components/modules/settings/SettingsSection";

export default function Settings() {
    return (
        <>
            <Card>
                <CardHeader>
                    <CardTitle>Beállítások</CardTitle>
                </CardHeader>
                <CardContent>
                    <SettingsSection
                        title="Kétlépcsős azonosítás"
                        description="Kapcsold be a kétlépcsős hitelesítést a nagyobb biztonság érdekében."
                    >
                        <MfaSettings />
                    </SettingsSection>
                </CardContent>
            </Card>
        </>
    );
}
