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
    align(left)[*Készletmozgás*],
    align(right)[https://obvia.hu],
  ),
  numbering: "1/1",
)

#let row(label, value) = ([*#label*], [#value])

#let field(obj, key, default: "-") = {
  let value = obj.at(key, default: none)
  if value == none or value == "" { default } else { value }
}

#let inventory_movement = json(bytes(sys.inputs.at("payload", default: "{}")))
#let inventory = inventory_movement.inventory

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("F2F2F2") },
  stroke: none,
  inset: 8pt,
)

  #v(0.5cm)

  #align(center)[
    #text(size: 16pt, weight: "bold")[Készletmozgás adatai]
  ]

  #v(0.5cm)

  #table(
    columns: (1fr, 2fr),
    table.header([*Mező*], [*Érték*]),

    ..row("Azonosító", field(inventory_movement, "id")),
    ..row("Raktárkészlet", field(inventory, "id")),
    ..row("Típus", field(inventory_movement, "movement_type")),
    ..row("Mennyiség", field(inventory_movement, "quantity")),
    ..row("Egységár", field(inventory_movement, "unit_price")),
    ..row("Összeg", field(inventory_movement, "total_price")),
    ..row("Adó", field(inventory_movement, "tax")),
    ..row("Mozgás dátuma", field(inventory_movement, "movement_date")),
    ..row("Létrehozta", field(inventory_movement, "created_by")),
    ..row("Létrehozva", field(inventory_movement, "created_at")),
    ..row("Hivatkozás típusa", field(inventory_movement, "reference_type")),
    ..row("Hivatkozás azonosító", field(inventory_movement, "reference_id")),
  )

  #pagebreak(weak: true)
