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

import { Moon, Sun } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useTheme } from "@/hooks/use_theme";

export function ThemeToggle() {
    const { setTheme } = useTheme();

    const toggle = () => {
        const isDark = document.documentElement.classList.contains("dark");
        setTheme(isDark ? "light" : "dark");
    };

    return (
        <Button className="w-full" size="icon" onClick={toggle}>
            <Sun className="hidden h-4 w-4 dark:block" />
            <Moon className="h-4 w-4 dark:hidden" />
            <span className="sr-only">Toggle theme</span>
        </Button>
    );
}
