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
    align(left)[*Adó*],
    align(right)[https://obvia.hu],
  ),
  numbering: "1/1",
)

#let row(label, value) = ([*#label*], [#value])

#let field(obj, key, default: "-") = {
  let value = obj.at(key, default: none)
  if value == none or value == "" { default } else { value }
}

#let taxes = json(bytes(sys.inputs.at("payload", default: "[]")))

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("F2F2F2") },
  stroke: none,
  inset: 8pt,
)

#for tax in taxes [
  #v(0.5cm)

  #align(center)[
    #text(size: 16pt, weight: "bold")[Adó adatai]
  ]

  #v(0.5cm)

  #table(
    columns: (1fr, 2fr),
    table.header([*Mező*], [*Érték*]),

    ..row("Azonosító", field(tax, "id")),
    ..row("Adókulcs (%)", field(tax, "rate")),
    ..row("Leírás", field(tax, "description")),
    ..row("Ország", field(tax, "country")),
    ..row("Adó kategória", field(tax, "tax_category")),
    ..row("Jogi szöveg", field(tax, "legal_text")),
    ..row("Jelentési kód (NAV)", field(tax, "reporting_code")),
    ..row("Státusz", field(tax, "status")),
    ..row("Létrehozta", field(tax, "created_by")),
    ..row("Létrehozva", field(tax, "created_at")),
    ..row("Frissítve", field(tax, "updated_at")),
  )

  #pagebreak(weak: true)
]
