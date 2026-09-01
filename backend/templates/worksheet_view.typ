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

// Page setup: margins, page numbering, header and footer
#set page(
  margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm),
  numbering: "1 / 1",
  header: [
    #grid(
      columns: (1fr, auto),
      align(left)[#text(12pt, weight: "bold")[Obvia ERP]],
      align(right)[#text(10pt)[Munkalap]],
    )
    #line(length: 100%)
  ],
  footer: [
    #line(length: 100%)
    #align(left)[#text(9pt, fill: rgb("000000"))[https://obvia.hu]]
  ],
)

// Helper: show fallback if value is empty or none
#let display-value(value, fallback) = {
  if value == none or value == "" {
    fallback
  } else {
    value
  }
}

// Helper: render a two-column label/value row
#let row(label, value) = [
  #grid(
    columns: (4.6cm, 1fr),
    gutter: 0.35cm,
    [#text(weight: "bold")[#label:]],
    [#display-value(value, "-")],
  )
]

// Helper: render a gray section heading block
#let section(title) = [
  #v(0.35cm)
  #block(
    fill: rgb("EAEAEA"),
    inset: (x: 10pt, y: 6pt),
    radius: 4pt,
  )[
    #text(size: 11pt, weight: "bold")[#title]
  ]
  #v(0.2cm)
]

// Extract data from json
#let worksheet = json(bytes(sys.inputs.at("payload", default: "{}")))
#let customer = worksheet.customer
#let tasks = worksheet.tasks
#let materials = worksheet.materials

// Default table styling
#set table(
  stroke: (paint: rgb("D9D9D9"), thickness: 0.6pt),
  inset: 6pt,
)

// Document title
#align(center)[
  #text(size: 18pt, weight: "bold")[Munkalap]
]

#v(0.2cm)

// Subtitle
#align(center)[
  #text(10pt, fill: rgb("000000"))[
    Nyilvántartási és ügyfélkapcsolati adatlap
  ]
]

#v(0.6cm)

// Worksheet details section
#section("Munkalap adatai")

#row("Munkalap azonosító", worksheet.id)
#row("Munkalap neve", worksheet.name)
#row("Leírás", worksheet.description)
#row("Ügyfél", customer.name)
#row("Állapot", worksheet.status)

#v(0.25cm)

// Cost summary rows
#row("Nettó anyagköltség", worksheet.net_material_cost)
#row("Bruttó anyagköltség", worksheet.gross_material_cost)
#row("Nettó munkadíj", worksheet.net_work_cost)
#row("Bruttó munkadíj", worksheet.gross_work_cost)

#v(0.25cm)

// Customer details section
#section("Ügyfél adatai")

#row("Ügyfél azonosító", customer.id)
#row("Ügyfél neve", customer.name)
#row("Kapcsolattartó neve", customer.contact_name)
#row("E-mail cím", customer.email)
#row("Telefonszám", customer.phone_number)
#row("Ügyféltípus", customer.customer_type)
#row("Állapot", customer.status)

#v(0.25cm)

// Performed services section
#section("Elvégzett szolgáltatások")

// Render tasks
#table(
  columns: (0.8cm, 5.8cm, 2.2cm, 2.2cm, 2.2cm),
  table.header(
    [*\#*],
    [*Szolgáltatás megnevezése*],
    [*Mennyiség*],
    [*Egységár*],
    [*Összesen*],
  ),

  ..for (i, task) in tasks.enumerate() {
    (
      [#(i + 1)],
      [#task.service.name],
      [#task.quantity],
      [#task.price],
      [#(float(task.quantity) * float(task.price))],
    )
  },
)

#v(0.25cm)

// Performed services section
#section("Felhasznált anyagok")

// Render materials
#table(
  columns: (0.8cm, 5.8cm, 2.2cm, 2.2cm, 2.2cm),
  table.header(
    [*\#*],
    [*Anyag megnevezése*],
    [*Mennyiség*],
    [*Egységár*],
    [*Összesen*],
  ),

  ..for (i, material) in materials.enumerate() {
    (
      [#(i + 1)],
      [#material.inventory.product.name],
      [#material.quantity],
      [#material.unit_price],
      [#material.total_price],
    )
  },
)

#v(0.25cm)

// Totals summary aligned to the right
#align(right)[
  #table(
    columns: (3.5cm, 2.8cm),
    [*Szolgáltatások összesen*], [#worksheet.net_work_cost],
    [*Anyagköltség*], [#display-value(worksheet.net_material_cost, "-")],
    [*Végösszeg*], [#(float(worksheet.net_work_cost) + float(worksheet.net_material_cost))],
  )
]

// Notes section
#section("Megjegyzés")

// Placeholder note box with example text
#block(
  stroke: (paint: rgb("CCCCCC"), thickness: 0.8pt),
  inset: 10pt,
  radius: 4pt,
  width: 100%,
  height: 3cm,
)[
  #text(9pt, fill: rgb("000000"))[
    Példa megjegyzés:
    A készülék túlmelegedési problémával érkezett. A tisztítást és az újrapasztázást követően
    a hőmérsékleti értékek stabilizálódtak. Az SSD-csere és a rendszer újratelepítése sikeresen megtörtént.
  ]
]

#v(0.8cm)

// Signature fields for customer and service provider
#grid(
  columns: (1fr, 1fr),
  gutter: 1.5cm,
  [
    #v(1.2cm)
    #line(length: 100%)
    #align(center)[#text(9pt)[Ügyfél aláírása]]
  ],
  [
    #v(1.2cm)
    #line(length: 100%)
    #align(center)[#text(9pt)[Szerviz aláírása]]
  ],
)

#v(0.8cm)

// Footer note about document origin
#block(
  stroke: (paint: rgb("CCCCCC"), thickness: 0.8pt),
  inset: 10pt,
  radius: 4pt,
)[
  #text(9pt, fill: rgb("000000"))[
    Megjegyzés: Ez a dokumentum az Obvia ERP rendszerből előállított munkalapnézet.
  ]
]
