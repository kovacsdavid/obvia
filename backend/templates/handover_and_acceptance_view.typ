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

// Extract data from JSON
#let handover = json(bytes(sys.inputs.at("payload", default: "{}")))
#let customer = handover.customer
#let owner_profile = handover.owner_profile

#let display-value(value, fallback: "-") = {
  if value == none or value == "" {
    fallback
  } else {
    value
  }
}

#set page(
  margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm),
  numbering: "1 / 1",
  header: [
    #grid(
      columns: (1fr, auto),
      align(left)[#text(10pt)[
        #display-value(
          if owner_profile.name == "" {
            owner_profile.email
          } else if owner_profile.email == "" {
            owner_profile.name
          } else {
            owner_profile.name + " <" + owner_profile.email + ">"
          },
          fallback: "Beállítások / Saját adatok / Név",
        )
      ]],
      align(right)[#text(10pt)[Átadás-átvételi adatlap]],
    )
    #line(length: 100%)
  ],
  footer: [
    #line(length: 100%)
    #grid(
      columns: (1fr, auto),
      align(left)[#text(9pt, fill: rgb("000000"))[
        #display-value(
          owner_profile.website,
          fallback: "Beállítások / Saját adatok / Weboldal",
        )
      ]],
      align(right)[#text(9pt, fill: rgb("000000"))[
        #display-value(
          owner_profile.billing_address,
          fallback: "Beállítások / Saját adatok / Számlázási cím",
        )
      ]],
    )
  ],
)

#let row(label, value) = [
  #grid(
    columns: (4.6cm, 1fr),
    gutter: 0.35cm,
    [#text(weight: "bold")[#label:]],
    [#display-value(value)],
  )
]

#let section(title) = [
  #v(0.35cm)
  #block(
    fill: rgb("EAEAEA"),
    inset: (x: 10pt, y: 6pt),
    radius: 4pt,
    width: 100%,
  )[
    #text(size: 11pt, weight: "bold")[#title]
  ]
  #v(0.2cm)
]

#align(center)[
  #text(size: 18pt, weight: "bold")[Átadás-átvételi adatlap]
]

#v(0.2cm)

#align(center)[
  #text(10pt)[Átvételi és ügyfélkapcsolati adatok]
]

#v(0.6cm)

#section("Átvétel adatai")

#row("Azonosító", handover.display_id)
#row("Átvétel időpontja", handover.received_at)
#row("Átvevő", handover.accepted_by)

#section("Ügyfél adatai")

#row("Ügyfél azonosító", customer.id)
#row("Ügyfél neve", customer.name)
#row("Kapcsolattartó neve", customer.contact_name)
#row("E-mail cím", customer.email)
#row("Telefonszám", customer.phone_number)
#row("Ügyféltípus", customer.customer_type)
#row("Számlázási cím", customer.billing_address)
#row("Levelezési cím", customer.mailing_address)

#section("Átvett eszköz adatai")

#row("Gyártó", handover.item_make)
#row("Modell", handover.item_model)
#row("Azonosító típusa", handover.item_identifier_type)
#row("Azonosító", handover.item_identifier_value)
#row("Bejelentett hiba", handover.reported_issue)
#row("Állapot átvételkor", handover.condition_at_receipt)
#row(
  "Átadott tartozékok",
  if handover.included_accessories == none or handover.included_accessories.len() == 0 {
    none
  } else {
    handover.included_accessories.join(", ")
  },
)

#section("Engedélyezett munkák")

#row("Engedélyezett munka", handover.authorized_work)
#row("Javítási értékhatár", handover.repair_limit)
#row("Feltételek verziója", handover.terms_version)

#section("Nyilvántartási adatok")

#row("Létrehozva", handover.created_at)

#v(0.8cm)

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
    #align(center)[#text(9pt)[Átvevő aláírása]]
  ],
)
