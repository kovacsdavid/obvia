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
    align(left)[*Munkalap*],
    align(right)[https://obvia.hu],
  ),
  numbering: "1/1",
)

#let row(label, value) = ([*#label*], [#value])

#let field(obj, key, default: "-") = {
  let value = obj.at(key, default: none)
  if value == none or value == "" { default } else { value }
}

#let worksheets = json(bytes(sys.inputs.at("payload", default: "[]")))

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("F2F2F2") },
  stroke: none,
  inset: 8pt,
)

#for worksheet in worksheets [
  #v(0.5cm)

  #align(center)[
    #text(size: 16pt, weight: "bold")[Munkalap adatai]
  ]

  #v(0.5cm)

  #table(
    columns: (1fr, 2fr),
    table.header([*Mező*], [*Érték*]),

    ..row("Azonosító", field(worksheet, "id")),
    ..row("Név", field(worksheet, "name")),
    ..row("Leírás", field(worksheet, "description")),
    ..row("Vevő", field(worksheet, "customer")),
    ..row("Nettó anyagköltség", field(worksheet, "net_material_cost")),
    ..row("Bruttó anyagköltség", field(worksheet, "gross_material_cost")),
    ..row("Nettó munkadíj", field(worksheet, "net_work_cost")),
    ..row("Bruttó munkadíj", field(worksheet, "gross_work_cost")),
    ..row("Státusz", field(worksheet, "status")),
    ..row("Létrehozta", field(worksheet, "created_by")),
    ..row("Létrehozva", field(worksheet, "created_at")),
    ..row("Frissítve", field(worksheet, "updated_at")),
  )

  #pagebreak(weak: true)
]
