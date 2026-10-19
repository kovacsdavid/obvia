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

export default function CallToAction() {
    const { isLoggedIn } = useAuth();
    return (
        <section
            aria-labelledby="landing-cta-title"
            className="mx-auto max-w-6xl px-4 py-16 text-center"
        >
            <h2
                id="landing-cta-title"
                className="text-3xl font-bold tracking-tight"
            >
                {isLoggedIn ? "Folytasd a munkát" : "Próbáld ki most"}
            </h2>
            <p className="mx-auto mt-3 max-w-xl text-muted-foreground">
                {isLoggedIn
                    ? "Lépj tovább a vezérlőpultra."
                    : "Hozd létre a fiókodat és a szervezeted adatbázisát néhány perc alatt."}
            </p>
            <div className="mt-8">
                <Button asChild size="lg">
                    {isLoggedIn ? (
                        <Link to="/vezerlopult">Vezérlőpult</Link>
                    ) : (
                        <Link to="/regisztracio">Regisztráció</Link>
                    )}
                </Button>
            </div>
        </section>
    );
}
