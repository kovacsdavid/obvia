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
    align(left)[*Vevő*],
    align(right)[https://obvia.hu],
  ),
  numbering: "1/1",
)

#let id = sys.inputs.at("id")
#let name = sys.inputs.at("name")
#let contact_name = sys.inputs.at("contact_name")
#let email = sys.inputs.at("email")
#let phone_number = sys.inputs.at("phone_number")
#let status = sys.inputs.at("status")
#let customer_type = sys.inputs.at("customer_type")
#let created_by = sys.inputs.at("created_by")
#let created_at = sys.inputs.at("created_at")
#let updated_at = sys.inputs.at("updated_at")

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("D9D9D9") },
  stroke: none,
)

#table(
  columns: (1fr, 2fr),
  [*Azonosító*], [#id],
  [*Típus*], [#customer_type],
  [*Kapcsolattartó neve*], [#contact_name],
  [*E-mail cím*], [#email],
  [*Telefonszám*], [#phone_number],
  [*Státusz*], [#status],
  [*Létrehozta*], [#created_by],
  [*Létrehozva*], [#created_at],
  [*Frissítve*], [#updated_at],
)
