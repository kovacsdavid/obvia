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

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
    FEATURE_STATUS_LABELS,
    LANDING_FEATURES,
    type LandingFeature,
} from "@/components/modules/landing_page/features";

function FeatureCard({ feature }: { feature: LandingFeature }) {
    const Icon = feature.icon;
    const status = feature.status ?? "available";
    return (
        <li>
            <Card className="h-full">
                <CardHeader>
                    <div className="flex items-start justify-between gap-2">
                        <Icon
                            className="size-6 text-primary"
                            aria-hidden="true"
                        />
                        {status !== "available" && (
                            <span className="rounded-full bg-muted px-2 py-0.5 text-xs text-muted-foreground">
                                {FEATURE_STATUS_LABELS[status]}
                            </span>
                        )}
                    </div>
                    <CardTitle>
                        <h3 className="text-base font-semibold">
                            {feature.title}
                        </h3>
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <p className="text-sm text-muted-foreground">
                        {feature.description}
                    </p>
                </CardContent>
            </Card>
        </li>
    );
}

export default function Features() {
    return (
        <section
            aria-labelledby="landing-features-title"
            className="mx-auto max-w-6xl px-4 py-16"
        >
            <h2
                id="landing-features-title"
                className="text-center text-3xl font-bold tracking-tight"
            >
                Funkciók
            </h2>
            <ul className="mt-10 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {LANDING_FEATURES.map((feature) => (
                    <FeatureCard key={feature.id} feature={feature} />
                ))}
            </ul>
        </section>
    );
}
