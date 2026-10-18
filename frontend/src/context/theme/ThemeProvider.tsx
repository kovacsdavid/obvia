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

import { ThemeProviderContext, type Theme } from "@/context/theme/ThemeContext";
import * as React from "react";

type ResolvedTheme = "dark" | "light";

type ThemeProviderProps = {
    children: React.ReactNode;
    defaultTheme?: Theme;
    storageKey?: string;
    disableTransitionOnChange?: boolean;
};

const COLOR_SCHEME_QUERY = "(prefers-color-scheme: dark)";
const THEME_VALUES: Theme[] = ["dark", "light", "system"];

function isTheme(value: string | null): value is Theme {
    if (value === null) {
        return false;
    }

    return THEME_VALUES.includes(value as Theme);
}

function getSystemTheme(): ResolvedTheme {
    if (window.matchMedia(COLOR_SCHEME_QUERY).matches) {
        return "dark";
    }

    return "light";
}

function disableTransitionsTemporarily() {
    const style = document.createElement("style");
    style.appendChild(
        document.createTextNode(
            "*,*::before,*::after{-webkit-transition:none!important;transition:none!important}",
        ),
    );
    document.head.appendChild(style);

    return () => {
        window.getComputedStyle(document.body);
        requestAnimationFrame(() => {
            requestAnimationFrame(() => {
                style.remove();
            });
        });
    };
}

function isEditableTarget(target: EventTarget | null) {
    if (!(target instanceof HTMLElement)) {
        return false;
    }

    if (target.isContentEditable) {
        return true;
    }

    const editableParent = target.closest(
        "input, textarea, select, [contenteditable='true']",
    );
    if (editableParent) {
        return true;
    }

    return false;
}

export function ThemeProvider({
    children,
    defaultTheme = "system",
    storageKey = "theme",
    disableTransitionOnChange = true,
    ...props
}: ThemeProviderProps) {
    const [theme, setThemeState] = React.useState<Theme>(() => {
        const storedTheme = localStorage.getItem(storageKey);
        if (isTheme(storedTheme)) {
            return storedTheme;
        }

        return defaultTheme;
    });

    const setTheme = React.useCallback(
        (nextTheme: Theme) => {
            localStorage.setItem(storageKey, nextTheme);
            setThemeState(nextTheme);
        },
        [storageKey],
    );

    const applyTheme = React.useCallback(
        (nextTheme: Theme) => {
            const root = document.documentElement;
            const resolvedTheme =
                nextTheme === "system" ? getSystemTheme() : nextTheme;
            const restoreTransitions = disableTransitionOnChange
                ? disableTransitionsTemporarily()
                : null;

            root.classList.remove("light", "dark");
            root.classList.add(resolvedTheme);

            if (restoreTransitions) {
                restoreTransitions();
            }
        },
        [disableTransitionOnChange],
    );

    React.useEffect(() => {
        applyTheme(theme);

        if (theme !== "system") {
            return undefined;
        }

        const mediaQuery = window.matchMedia(COLOR_SCHEME_QUERY);
        const handleChange = () => {
            applyTheme("system");
        };

        mediaQuery.addEventListener("change", handleChange);

        return () => {
            mediaQuery.removeEventListener("change", handleChange);
        };
    }, [theme, applyTheme]);

    React.useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.repeat) {
                return;
            }

            if (event.metaKey || event.ctrlKey || event.altKey) {
                return;
            }

            if (isEditableTarget(event.target)) {
                return;
            }

            if (event.key.toLowerCase() !== "d") {
                return;
            }

            setThemeState((currentTheme) => {
                const nextTheme =
                    currentTheme === "dark"
                        ? "light"
                        : currentTheme === "light"
                          ? "dark"
                          : getSystemTheme() === "dark"
                            ? "light"
                            : "dark";

                localStorage.setItem(storageKey, nextTheme);
                return nextTheme;
            });
        };

        window.addEventListener("keydown", handleKeyDown);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, [storageKey]);

    React.useEffect(() => {
        const handleStorageChange = (event: StorageEvent) => {
            if (event.storageArea !== localStorage) {
                return;
            }

            if (event.key !== storageKey) {
                return;
            }

            if (isTheme(event.newValue)) {
                setThemeState(event.newValue);
                return;
            }

            setThemeState(defaultTheme);
        };

        window.addEventListener("storage", handleStorageChange);

        return () => {
            window.removeEventListener("storage", handleStorageChange);
        };
    }, [defaultTheme, storageKey]);

    const value = React.useMemo(
        () => ({
            theme,
            setTheme,
        }),
        [theme, setTheme],
    );

    return (
        <ThemeProviderContext.Provider {...props} value={value}>
            {children}
        </ThemeProviderContext.Provider>
    );
}
