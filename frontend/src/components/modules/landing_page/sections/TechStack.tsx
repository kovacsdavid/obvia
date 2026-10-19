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

const TECH_ITEMS: readonly { name: string; description: string }[] = [
    { name: "Rust", description: "Gyors és biztonságos backend" },
    { name: "React", description: "Modern, reszponzív felület" },
    { name: "PostgreSQL", description: "Megbízható adattárolás" },
    { name: "AGPL-3.0", description: "Licenc" },
];

export default function TechStack() {
    return (
        <section aria-labelledby="landing-tech-title" className="bg-muted/50">
            <div className="mx-auto max-w-6xl px-4 py-12">
                <h2
                    id="landing-tech-title"
                    className="text-center text-2xl font-bold tracking-tight"
                >
                    Szabad szoftver, modern technológia
                </h2>
                <p className="mx-auto mt-3 max-w-2xl text-center text-muted-foreground">
                    Az Obvia ERP Rust backenddel, React frontenddel és
                    PostgreSQL adatbázissal készül. A forráskód az AGPL-3.0
                    licenc alatt elérhető.
                </p>
                <ul className="mt-8 grid grid-cols-2 gap-4 text-center md:grid-cols-4">
                    {TECH_ITEMS.map((item) => (
                        <li
                            key={item.name}
                            className="rounded-lg border bg-background p-4"
                        >
                            <p className="font-semibold">{item.name}</p>
                            <p className="text-sm text-muted-foreground">
                                {item.description}
                            </p>
                        </li>
                    ))}
                </ul>
            </div>
        </section>
    );
}
