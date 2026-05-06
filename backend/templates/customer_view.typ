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

#let customer_resolved_id = sys.inputs.at("customer_resolved_id")
#let customer_resolved_name = sys.inputs.at("customer_resolved_name")
#let customer_resolved_contact_name = sys.inputs.at("customer_resolved_contact_name")
#let customer_resolved_email = sys.inputs.at("customer_resolved_email")
#let customer_resolved_phone_number = sys.inputs.at("customer_resolved_phone_number")
#let customer_resolved_status = sys.inputs.at("customer_resolved_status")
#let customer_resolved_customer_type = sys.inputs.at("customer_resolved_customer_type")
#let customer_resolved_created_by = sys.inputs.at("customer_resolved_created_by")
#let customer_resolved_created_at = sys.inputs.at("customer_resolved_created_at")
#let customer_resolved_updated_at = sys.inputs.at("customer_resolved_updated_at")

#set table(
  fill: (_, y) => if calc.odd(y) { rgb("D9D9D9") },
  stroke: none,
)

#table(
  columns: (1fr, 2fr),
  [*Azonosító*], [#customer_resolved_id],
  [*Típus*], [#customer_resolved_customer_type],
  [*Kapcsolattartó neve*], [#customer_resolved_contact_name],
  [*E-mail cím*], [#customer_resolved_email],
  [*Telefonszám*], [#customer_resolved_phone_number],
  [*Státusz*], [#customer_resolved_status],
  [*Létrehozta*], [#customer_resolved_created_by],
  [*Létrehozva*], [#customer_resolved_created_at],
  [*Frissítve*], [#customer_resolved_updated_at],
)
