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

import { Link } from "react-router";
import { Button } from "@/components/ui";
import { useAuth } from "@/hooks/use_auth";

export default function Hero() {
    const { isLoggedIn } = useAuth();
    return (
        <section
            aria-labelledby="landing-hero-title"
            className="mx-auto max-w-6xl px-4 py-16 text-center sm:py-24"
        >
            <h1
                id="landing-hero-title"
                className="text-4xl font-bold tracking-tight sm:text-5xl"
            >
                Obvia ERP
            </h1>
            <p className="mx-auto mt-4 max-w-2xl text-lg text-muted-foreground">
                Modern, nyílt forráskódú vállalatirányítási rendszer: vevők,
                termékek, raktárak és munkalapok egy helyen, biztonságosan.
            </p>
            <div className="mt-8 flex flex-col justify-center gap-3 sm:flex-row">
                {isLoggedIn ? (
                    <Button asChild size="lg">
                        <Link to="/vezerlopult">Vezérlőpult</Link>
                    </Button>
                ) : (
                    <>
                        <Button asChild size="lg">
                            <Link to="/regisztracio">Regisztráció</Link>
                        </Button>
                        <Button asChild size="lg" variant="outline">
                            <Link to="/bejelentkezes">Bejelentkezés</Link>
                        </Button>
                    </>
                )}
            </div>
        </section>
    );
}
