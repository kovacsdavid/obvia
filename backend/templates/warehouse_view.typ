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

#set page(
  header: grid(
    columns: (1fr, 1fr),
    align(left)[*Raktár*],
    align(right)[https://obvia.hu],
  ),
  numbering: "1/1",
)

#let row(label, value) = ([*#label*], [#value])

#let field(obj, key, default: "-") = {
  let value = obj.at(key, default: none)
  if value == none or value == "" { default } else { value }
}

#let warehouses = json(bytes(sys.inputs.at("payload", default: "[]")))

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("F2F2F2") },
  stroke: none,
  inset: 8pt,
)

#for warehouse in warehouses [
  #v(0.5cm)

  #align(center)[
    #text(size: 16pt, weight: "bold")[Raktár adatai]
  ]

  #v(0.5cm)

  #table(
    columns: (1fr, 2fr),
    table.header([*Mező*], [*Érték*]),

    ..row("Azonosító", field(warehouse, "id")),
    ..row("Név", field(warehouse, "name")),
    ..row("Kapcsolattartó neve", field(warehouse, "contact_name")),
    ..row("Kapcsolattartó telefonszáma", field(warehouse, "contact_phone")),
    ..row("Státusz", field(warehouse, "status")),
    ..row("Létrehozta", field(warehouse, "created_by")),
    ..row("Létrehozva", field(warehouse, "created_at")),
    ..row("Frissítve", field(warehouse, "updated_at")),
  )

  #pagebreak(weak: true)
]
