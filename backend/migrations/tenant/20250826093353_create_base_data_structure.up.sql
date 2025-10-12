/*
 * This file is part of the Obvia ERP.
 *
 * Copyright (C) 2025 Kovács Dávid <kapcsolat@kovacsdavid.dev>
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

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE OR REPLACE FUNCTION update_updated_at()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';


create table users
(
    id                  uuid primary key,
    email               varchar(255) not null unique,
    first_name          varchar(255),
    last_name           varchar(255),
    phone               varchar(32),
    status              varchar(50)  not null default 'active',
    profile_picture_url text,
    locale              varchar(16)           default 'hu-HU',
    invited_by          uuid,
    email_verified_at   timestamptz,
    created_at          timestamptz  not null default now(),
    updated_at          timestamptz  not null default now(),
    deleted_at          timestamptz,
    foreign key (invited_by) references users (id)
);

CREATE INDEX idx_users_invited_by ON users (invited_by);
CREATE INDEX idx_users_created_at ON users (created_at);
CREATE INDEX idx_users_updated_at ON users (updated_at);
CREATE INDEX idx_users_deleted_at ON users (deleted_at);

CREATE TRIGGER update_updated_at_on_users_table
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table customers
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    contact_name  varchar(255),
    email         varchar(255) not null,
    phone_number  varchar(50),
    status        varchar(50)  not null,
    customer_type varchar(50)  not null,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by_id) references users (id),
    unique nulls not distinct (email, deleted_at)
);

CREATE INDEX idx_customers_created_by_id ON customers (created_by_id);
CREATE INDEX idx_customers_created_at ON customers (created_at);
CREATE INDEX idx_customers_updated_at ON customers (updated_at);
CREATE INDEX idx_customers_deleted_at ON customers (deleted_at);

CREATE TRIGGER update_updated_at_on_customers_table
    BEFORE UPDATE
    ON customers
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table comments
(
    id               uuid         not null primary key,
    commentable_type varchar(255) not null,
    commentable_id   uuid         not null,
    comment          text,
    created_by_id    uuid         not null,
    created_at       timestamptz  not null default now(),
    updated_at       timestamptz  not null default now(),
    deleted_at       timestamptz,
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_comments_created_by_id ON comments (created_by_id);
CREATE INDEX idx_comments_created_at ON comments (created_at);
CREATE INDEX idx_comments_updated_at ON comments (updated_at);
CREATE INDEX idx_comments_deleted_at ON comments (deleted_at);

CREATE TRIGGER update_updated_at_on_comments_table
    BEFORE UPDATE
    ON comments
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table countries
(
    code varchar(2)   not null primary key,
    name varchar(100) not null
);

INSERT INTO countries (code, name)
VALUES ('HU', 'Magyarország'),
       ('AD', 'Andorra'),
       ('AE', 'Egyesült Arab Emírségek'),
       ('AF', 'Afganisztán'),
       ('AG', 'Antigua'),
       ('AI', 'Anguilla'),
       ('AL', 'Albánia'),
       ('AM', 'Örményország'),
       ('AO', 'Angola'),
       ('AQ', 'Antarktisz'),
       ('AR', 'Argentína'),
       ('AS', 'Amerikai Szamoa'),
       ('AT', 'Ausztria'),
       ('AU', 'Ausztrália'),
       ('AW', 'Aruba'),
       ('AX', 'Aaland szigetek'),
       ('AZ', 'Azerbajdzsán'),
       ('BA', 'Bosznia-Hercegovina'),
       ('BB', 'Barbados'),
       ('BD', 'Banglades'),
       ('BE', 'Belgium'),
       ('BF', 'Burkina Faso'),
       ('BG', 'Bulgária'),
       ('BH', 'Bahrain'),
       ('BI', 'Burundi'),
       ('BJ', 'Benin'),
       ('BL', 'Saint Barthélémy'),
       ('BM', 'Bermuda'),
       ('BN', 'Brunei'),
       ('BO', 'Bolívia'),
       ('BQ', 'Bonaire'),
       ('BR', 'Brazília'),
       ('BS', 'Bahama-szigetek Nassau'),
       ('BT', 'Bhután'),
       ('BV', 'Bouvet-sziget'),
       ('BW', 'Botswana'),
       ('BY', 'Fehéroroszország'),
       ('BZ', 'Belize'),
       ('CA', 'Kanada'),
       ('CC', 'Kókusz (Keeling)-szigetek'),
       ('CD', 'Kongói Demokratikus Köztársaság'),
       ('CF', 'Közép-Afrikai Köztársaság'),
       ('CG', 'Kongó'),
       ('CH', 'Svájc'),
       ('CI', 'Elefántcsontpart'),
       ('CK', 'Cook-szigetek'),
       ('CL', 'Chile'),
       ('CM', 'Kamerun'),
       ('CN', 'Kína'),
       ('CO', 'Kolumbia'),
       ('CR', 'Costa Rica'),
       ('CU', 'Kuba'),
       ('CV', 'Zöldfoki Köztársaság'),
       ('CW', 'Curacao'),
       ('CX', 'Karácsony-sziget'),
       ('CY', 'Ciprus'),
       ('CZ', 'Csehország'),
       ('DE', 'Németország'),
       ('DJ', 'Dzsibuti'),
       ('DK', 'Dánia'),
       ('DM', 'Dominika'),
       ('DO', 'Dominikai Köztársaság'),
       ('DZ', 'Algéria'),
       ('EC', 'Equador'),
       ('EE', 'Észtország'),
       ('EG', 'Egyiptom'),
       ('EH', 'Nyugat-Szahara'),
       ('ER', 'Eritrea'),
       ('ES', 'Spanyolország'),
       ('ET', 'Etiópia'),
       ('FI', 'Finnország'),
       ('FJ', 'Fidzsi-szigetek'),
       ('FK', 'Falkland-szigetek'),
       ('FM', 'Mikronézia'),
       ('FO', 'Faroe szigetek'),
       ('FR', 'Franciaország'),
       ('FX', 'France, metropolitan'),
       ('GA', 'Gabon'),
       ('GB', 'Egyesült Királyság (Nagy Britannia)'),
       ('GD', 'Grenada'),
       ('GE', 'Grúzia'),
       ('GF', 'Francia Guiana'),
       ('GG', 'Guernsey'),
       ('GH', 'Ghana'),
       ('GI', 'Gibraltár'),
       ('GL', 'Grönland'),
       ('GM', 'Gambia'),
       ('GN', 'Guinea'),
       ('GP', 'Guadeloupe'),
       ('GQ', 'Egyenlítői Guinea'),
       ('GR', 'Görögország'),
       ('GS', 'Déli-Georgia és Déli-Sandwich-szigetek'),
       ('GT', 'Guatemala'),
       ('GU', 'Guam'),
       ('GW', 'Bissau-Guinea'),
       ('GY', 'Guyana'),
       ('HK', 'Hongkong'),
       ('HM', 'Heard-sziget és McDonalds-szigetek'),
       ('HN', 'Honduras'),
       ('HR', 'Horvátország'),
       ('HT', 'Haiti'),
       ('ID', 'Indonézia'),
       ('IE', 'Írország'),
       ('IL', 'Izrael'),
       ('IM', 'Man sziget'),
       ('IN', 'India'),
       ('IO', 'Brit Indiai-Óceániai Terület'),
       ('IQ', 'Irak'),
       ('IR', 'Irán'),
       ('IS', 'Izland'),
       ('IT', 'Olaszország'),
       ('JE', 'Jersey'),
       ('JM', 'Jamaica'),
       ('JO', 'Jordánia'),
       ('JP', 'Japán'),
       ('KE', 'Kenya'),
       ('KG', 'Kirgizisztán'),
       ('KH', 'Kambodzsa'),
       ('KI', 'Kiribati Köztársaság Tuvalu'),
       ('KM', 'Comore-szigetek'),
       ('KN', 'Saint Christopher és Nevis'),
       ('KP', 'Koreai NDK'),
       ('KR', 'Dél Korea'),
       ('KW', 'Kuwait'),
       ('KY', 'Kajmán-szigetek'),
       ('KZ', 'Kazahsztán'),
       ('LA', 'Laosz'),
       ('LB', 'Libanon'),
       ('LC', 'Saint Lucia'),
       ('LI', 'Liechtenstein'),
       ('LK', 'Sri Lanka'),
       ('LR', 'Liberia'),
       ('LS', 'Lesotho'),
       ('LT', 'Litvánia'),
       ('LU', 'Luxemburg'),
       ('LV', 'Lettország'),
       ('LY', 'Líbia'),
       ('MA', 'Marokkó'),
       ('MC', 'Monaco'),
       ('MD', 'Moldova'),
       ('ME', 'Montenegro'),
       ('MF', 'Saint Martin'),
       ('MG', 'Malgas Köztársaság'),
       ('MH', 'Marshall-szigetek'),
       ('MK', 'Észak-Macedónia'),
       ('ML', 'Mali'),
       ('MM', 'Mianmar'),
       ('MN', 'Mongólia'),
       ('MO', 'Macao'),
       ('MP', 'Északi-Mariana-szigetek'),
       ('MQ', 'Martinique'),
       ('MR', 'Mauritania'),
       ('MS', 'Montserrat'),
       ('MT', 'Málta'),
       ('MU', 'Mauritius'),
       ('MV', 'Maldív-szigetek'),
       ('MW', 'Malawi'),
       ('MX', 'Mexikó'),
       ('MY', 'Malajzia'),
       ('MZ', 'Mozambik'),
       ('NA', 'Namíbia'),
       ('NC', 'Új-Kaledónia'),
       ('NE', 'Niger'),
       ('NF', 'Norfolk szigetek'),
       ('NG', 'Nigéria'),
       ('NI', 'Nicaragua'),
       ('NL', 'Hollandia'),
       ('NO', 'Norvégia'),
       ('NP', 'Nepál'),
       ('NR', 'Nauru'),
       ('NU', 'Niue'),
       ('NZ', 'Új-Zéland'),
       ('OM', 'Omán'),
       ('PA', 'Panama'),
       ('PE', 'Peru'),
       ('PF', 'Francia Polinézia'),
       ('PG', 'Pápua Új-Ginea'),
       ('PH', 'Fülöp-szigetek'),
       ('PK', 'Pakisztán'),
       ('PL', 'Lengyelország'),
       ('PM', 'Saint Pierre és Miquelon'),
       ('PN', 'Pitcairn-sziget'),
       ('PR', 'Puerto Rico'),
       ('PS', 'Palesztína'),
       ('PT', 'Portugália'),
       ('PW', 'Palau'),
       ('PY', 'Paraguay'),
       ('QA', 'Quatar'),
       ('RE', 'Reunion'),
       ('RO', 'Románia'),
       ('RS', 'Szerbia'),
       ('RU', 'Oroszország'),
       ('RW', 'Ruanda'),
       ('SA', 'Szaud-Arábia'),
       ('SB', 'Solomon-szigetek'),
       ('SC', 'Seychelle-szigetek'),
       ('SD', 'Szudán'),
       ('SE', 'Svédország'),
       ('SG', 'Szingapúr'),
       ('SH', 'Szent Ilona'),
       ('SI', 'Szlovénia'),
       ('SJ', 'Svalbard és Jan Mayen'),
       ('SK', 'Szlovákia'),
       ('SL', 'Sierra Leone'),
       ('SM', 'San Marino'),
       ('SN', 'Szenegál'),
       ('SO', 'Szomália'),
       ('SR', 'Suriname'),
       ('SS', 'Dél-Szudán'),
       ('ST', 'Sao Tome és Principe'),
       ('SV', 'Salvador'),
       ('SX', 'St. Maarten'),
       ('SY', 'Szíria'),
       ('SZ', 'Szváziföld'),
       ('TC', 'Turks- és Caicos-szigetek'),
       ('TD', 'Csád'),
       ('TF', 'Francia Déli Területek'),
       ('TG', 'Togo'),
       ('TH', 'Thaiföld'),
       ('TJ', 'Tadzsikisztán'),
       ('TK', 'Tokelau-szigetek'),
       ('TL', 'Kelet-Timor'),
       ('TM', 'Türkmenisztán'),
       ('TN', 'Tunézia'),
       ('TO', 'Tonga'),
       ('TR', 'Törökország'),
       ('TT', 'Trinidad és Tobago'),
       ('TV', 'Tuvalu'),
       ('TW', 'Taiwan'),
       ('TZ', 'Tanzánia'),
       ('UA', 'Ukrajna'),
       ('UG', 'Uganda'),
       ('UM', 'Amerikai Csendes-óceáni-Szigetek'),
       ('US', 'Amerikai Egyesült Államok'),
       ('UY', 'Uruguay'),
       ('UZ', 'Üzbegisztán'),
       ('VA', 'Vatikán'),
       ('VC', 'Saint Vincent és Grenadines'),
       ('VE', 'Venezuela'),
       ('VG', 'Brit Virgin-szigetek'),
       ('VI', 'Amerikai Virgin-szigetek'),
       ('VN', 'Vietnámi Köztársaság'),
       ('VU', 'Vanuatu'),
       ('W1', 'Gáza és Jerikó'),
       ('WF', 'Wallis és Futuna'),
       ('WS', 'Nyugat-Szamoa'),
       ('XK', 'Koszovó'),
       ('YE', 'Jemeni Arab Köztársaság'),
       ('YT', 'Mayotte'),
       ('ZA', 'Dél-Afrikai Köztársaság'),
       ('ZM', 'Zambia'),
       ('ZW', 'Zimbabwe');

create table states
(
    id            uuid         not null primary key,
    name          varchar(100) not null,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_states_created_by_id ON states (created_by_id);
CREATE INDEX idx_states_created_at ON states (created_at);

CREATE TRIGGER update_updated_at_on_states_table
    BEFORE UPDATE
    ON states
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table postal_codes -- Entry can only be deleted if no data relies on it!
(
    id            uuid        not null primary key,
    postal_code   varchar(20) not null,
    created_by_id uuid        not null,
    created_at    timestamptz not null default now(),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_postal_codes_created_by_id ON postal_codes (created_by_id);
CREATE INDEX idx_postal_codes_created_at ON postal_codes (created_at);

create table cities -- Entry can only be deleted if no data relies on it!
(
    id            uuid         not null primary key,
    postal_code   uuid         not null,
    name          varchar(100) not null,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_cities_created_by_id ON cities (created_by_id);
CREATE INDEX idx_cities_created_at ON cities (created_at);

create table address
(
    id              uuid primary key      default uuid_generate_v4(),
    street_address  varchar(255) not null,
    city_id         uuid         not null,
    state_id        uuid         not null,
    country_code    varchar(2)   not null,
    additional_info text,
    created_by_id   uuid         not null,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz  not null default now(),
    deleted_at      timestamptz,
    foreign key (created_by_id) references users (id),
    foreign key (city_id) references cities (id),
    foreign key (state_id) references states (id),
    foreign key (country_code) references countries (code)
);

CREATE INDEX idx_address_city_id ON address (city_id);
CREATE INDEX idx_address_state_id ON address (state_id);
CREATE INDEX idx_address_country_code ON address (country_code);
CREATE INDEX idx_address_created_by_id ON address (created_by_id);
CREATE INDEX idx_address_created_at ON address (created_at);
CREATE INDEX idx_address_updated_at ON address (updated_at);
CREATE INDEX idx_address_deleted_at ON address (deleted_at);

CREATE TRIGGER update_updated_at_on_address_table
    BEFORE UPDATE
    ON address
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table address_connect
(
    id               uuid primary key default uuid_generate_v4(),
    address_id       uuid         not null,
    addressable_id   uuid         not null,
    addressable_type varchar(100) not null,
    created_by_id    uuid         not null,
    created_at       timestamptz      default now(),
    updated_at       timestamptz      default now(),
    deleted_at       timestamptz,
    foreign key (address_id) references address (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_address_connect_address_id ON address_connect (address_id);
CREATE INDEX idx_address_connect_addressable_type_id ON address_connect (addressable_type, addressable_id);
CREATE INDEX idx_address_connect_created_by_id ON address_connect (created_by_id);
CREATE INDEX idx_address_connect_created_at ON address_connect (created_at);
CREATE INDEX idx_address_connect_updated_at ON address_connect (updated_at);
CREATE INDEX idx_address_connect_deleted_at ON address_connect (deleted_at);

CREATE TRIGGER update_updated_at_on_address_connect_table
    BEFORE UPDATE
    ON address_connect
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table tags
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    description   text,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_tags_created_by_id ON tags (created_by_id);
CREATE INDEX idx_tags_created_at ON tags (created_at);
CREATE INDEX idx_tags_deleted_at ON tags (deleted_at);

create table tag_connect
(
    id            uuid         not null primary key,
    taggable_id   uuid         not null,
    taggable_type varchar(255) not null,
    tag_id        uuid,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (tag_id) references tags (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_tag_connect_tag_id ON tag_connect (tag_id);
CREATE INDEX idx_tag_connect_taggable_type_id ON tag_connect (taggable_type, taggable_id);
CREATE INDEX idx_tag_connect_created_by_id ON tag_connect (created_by_id);
CREATE INDEX idx_tag_connect_created_at ON tag_connect (created_at);
CREATE INDEX idx_tag_connect_deleted_at ON tag_connect (deleted_at);

create table projects
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    description   text,
    created_by_id uuid         not null,
    status        varchar(50)  not null default 'planning',
    start_date    timestamptz,
    end_date      timestamptz,
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_projects_created_by_id ON projects (created_by_id);
CREATE INDEX idx_projects_created_at ON projects (created_at);
CREATE INDEX idx_projects_updated_at ON projects (updated_at);
CREATE INDEX idx_projects_deleted_at ON projects (deleted_at);

CREATE TRIGGER update_updated_at_on_projects_table
    BEFORE UPDATE
    ON projects
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table worksheets
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    description   text,
    project_id    uuid         not null,
    created_by_id uuid         not null,
    status        varchar(50)  not null default 'draft',
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (project_id) references projects (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_worksheets_project_id ON worksheets (project_id);
CREATE INDEX idx_worksheets_created_by_id ON worksheets (created_by_id);
CREATE INDEX idx_worksheets_created_at ON worksheets (created_at);
CREATE INDEX idx_worksheets_updated_at ON worksheets (updated_at);
CREATE INDEX idx_worksheets_deleted_at ON worksheets (deleted_at);

create table taxes
(
    id                 uuid primary key      default uuid_generate_v4(),
    rate               numeric(5, 2),
    description        varchar(255) not null,
    country_code       varchar(2)   not null,
    tax_category       varchar(50)  not null,              -- 'standard', 'reduced', 'exempt', 'reverse_charge', 'small_business_exempt', etc.
    is_rate_applicable boolean      not null default true, -- false for Alanyi adómentes
    legal_text         text,                               -- Required legal disclaimer for invoices
    reporting_code     varchar(50),                        -- For tax authority reporting (e.g., NAV reporting codes)
    is_default         boolean      not null default false,
    status             varchar(50)  not null default 'active',
    created_by_id      uuid         not null,
    created_at         timestamptz  not null default now(),
    updated_at         timestamptz  not null default now(),
    deleted_at         timestamptz,
    foreign key (country_code) references countries (code),
    foreign key (created_by_id) references users (id),
    unique nulls not distinct (rate, country_code, tax_category, deleted_at),
    constraint check_rate_applicable check (
        (is_rate_applicable = true and rate is not null) or
        (is_rate_applicable = false and rate is null)
        )
);

CREATE INDEX idx_taxes_country_code ON taxes (country_code);
CREATE INDEX idx_taxes_tax_category ON taxes (tax_category);
CREATE INDEX idx_taxes_created_by_id ON taxes (created_by_id);
CREATE INDEX idx_taxes_created_at ON taxes (created_at);
CREATE INDEX idx_taxes_updated_at ON taxes (updated_at);
CREATE INDEX idx_taxes_deleted_at ON taxes (deleted_at);

CREATE TRIGGER update_updated_at_on_taxes_table
    BEFORE UPDATE
    ON taxes
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table currencies
(
    id            uuid primary key     default uuid_generate_v4(),
    currency      varchar(3)  not null,
    created_by_id uuid        not null,
    created_at    timestamptz not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by_id) references users (id),
    unique nulls not distinct (currency, deleted_at)
);

CREATE INDEX idx_currencies_created_by_id ON currencies (created_by_id);
CREATE INDEX idx_currencies_created_at ON currencies (created_at);

create table services
(
    id             uuid primary key      default uuid_generate_v4(),
    name           varchar(255) not null,
    description    text,
    default_price  numeric(15, 2),
    default_tax_id uuid,
    currency_id    uuid, -- default currency
    status         varchar(50)  not null default 'active',
    created_by_id  uuid         not null,
    created_at     timestamptz  not null default now(),
    updated_at     timestamptz  not null default now(),
    deleted_at     timestamptz,
    foreign key (currency_id) references currencies (id),
    foreign key (created_by_id) references users (id),
    foreign key (default_tax_id) references taxes (id)
);

CREATE INDEX idx_services_currency_id ON services (currency_id);
CREATE INDEX idx_services_default_tax_id ON services (default_tax_id);
CREATE INDEX idx_services_created_by_id ON services (created_by_id);
CREATE INDEX idx_services_created_at ON services (created_at);
CREATE INDEX idx_services_updated_at ON services (updated_at);
CREATE INDEX idx_services_deleted_at ON services (deleted_at);

CREATE TRIGGER update_updated_at_on_services_table
    BEFORE UPDATE
    ON services
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER update_updated_at_on_worksheets_table
    BEFORE UPDATE
    ON worksheets
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table tasks
(
    id            uuid primary key     default uuid_generate_v4(),
    worksheet_id  uuid        not null,
    service_id    uuid        not null,
    currency_id   uuid        not null,
    price         numeric(15, 2),
    tax_id        uuid        not null,
    created_by_id uuid        not null,
    status        varchar(50) not null default 'pending',
    priority      varchar(50),
    due_date      timestamptz,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    deleted_at    timestamptz,
    foreign key (worksheet_id) references worksheets (id),
    foreign key (service_id) references services (id),
    foreign key (created_by_id) references users (id),
    foreign key (tax_id) references taxes (id),
    foreign key (currency_id) references currencies (id)
);

CREATE INDEX idx_tasks_worksheet_id ON tasks (worksheet_id);
CREATE INDEX idx_tasks_service_id ON tasks (service_id);
CREATE INDEX idx_tasks_currency_id ON tasks (currency_id);
CREATE INDEX idx_tasks_tax_id ON tasks (tax_id);
CREATE INDEX idx_tasks_created_by_id ON tasks (created_by_id);
CREATE INDEX idx_tasks_created_at ON tasks (created_at);
CREATE INDEX idx_tasks_updated_at ON tasks (updated_at);
CREATE INDEX idx_tasks_deleted_at ON tasks (deleted_at);

CREATE TRIGGER update_updated_at_on_tasks_table
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table units_of_measure
(
    id              uuid primary key     default uuid_generate_v4(),
    unit_of_measure varchar(50) not null,
    created_by_id   uuid        not null,
    created_at      timestamptz not null default now(),
    deleted_at      timestamptz,
    foreign key (created_by_id) references users (id),
    unique nulls not distinct (unit_of_measure, deleted_at)
);

CREATE INDEX idx_units_of_measure_created_by_id ON units_of_measure (created_by_id);
CREATE INDEX idx_units_of_measure_created_at ON units_of_measure (created_at);

create table products
(
    id                 uuid primary key      default uuid_generate_v4(),
    name               varchar(255) not null,
    description        text,
    unit_of_measure_id uuid         not null,
    status             varchar(50)  not null default 'active',
    created_by_id      uuid         not null,
    created_at         timestamptz           default now(),
    updated_at         timestamptz           default now(),
    deleted_at         timestamptz,
    foreign key (unit_of_measure_id) references units_of_measure (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_products_unit_of_measure_id ON products (unit_of_measure_id);
CREATE INDEX idx_products_created_by_id ON products (created_by_id);
CREATE INDEX idx_products_created_at ON products (created_at);
CREATE INDEX idx_products_updated_at ON products (updated_at);
CREATE INDEX idx_products_deleted_at ON products (deleted_at);

CREATE TRIGGER update_updated_at_on_products_table
    BEFORE UPDATE
    ON products
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table product_category
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    description   text,
    parent_id     uuid,
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    foreign key (parent_id) references product_category (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_product_category_parent_id ON product_category (parent_id);
CREATE INDEX idx_product_category_created_by_id ON product_category (created_by_id);
CREATE INDEX idx_product_category_created_at ON product_category (created_at);

create table product_category_connect
(
    id                  uuid primary key     default uuid_generate_v4(),
    product_id          uuid        not null,
    product_category_id uuid        not null,
    created_by_id       uuid        not null,
    created_at          timestamptz not null default now(),
    deleted_at          timestamptz,
    foreign key (product_id) references products (id),
    foreign key (product_category_id) references product_category (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_product_category_connect_product_id ON product_category_connect (product_id);
CREATE INDEX idx_product_category_connect_product_category_id ON product_category_connect (product_category_id);
CREATE INDEX idx_product_category_connect_created_by_id ON product_category_connect (created_by_id);
CREATE INDEX idx_product_category_connect_created_at ON product_category_connect (created_at);
CREATE INDEX idx_product_category_connect_deleted_at ON product_category_connect (deleted_at);

create table warehouses
(
    id            uuid primary key      default uuid_generate_v4(),
    name          varchar(255) not null,
    contact_name  varchar(255),
    contact_phone varchar(50),
    status        varchar(50)  not null default 'active',
    created_by_id uuid         not null,
    created_at    timestamptz  not null default now(),
    updated_at    timestamptz  not null default now(),
    deleted_at    timestamptz,
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_warehouses_created_by_id ON warehouses (created_by_id);
CREATE INDEX idx_warehouses_created_at ON warehouses (created_at);
CREATE INDEX idx_warehouses_updated_at ON warehouses (updated_at);
CREATE INDEX idx_warehouses_deleted_at ON warehouses (deleted_at);

CREATE TRIGGER update_updated_at_on_warehouses_table
    BEFORE UPDATE
    ON warehouses
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table inventory
(
    id            uuid primary key     default uuid_generate_v4(),
    product_id    uuid        not null,
    warehouse_id  uuid        not null,
    quantity      integer     not null default 0,
    price         numeric(15, 2),
    tax_id        uuid        not null,
    currency_id   uuid        not null,
    created_by_id uuid        not null,
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    deleted_at    timestamptz,
    foreign key (currency_id) references currencies (id),
    foreign key (product_id) references products (id),
    foreign key (warehouse_id) references warehouses (id),
    foreign key (created_by_id) references users (id),
    foreign key (tax_id) references taxes (id)
);

CREATE INDEX idx_inventory_price ON inventory (price);
CREATE INDEX idx_inventory_tax_id ON inventory (tax_id);
CREATE INDEX idx_inventory_currency_id ON inventory (currency_id);
CREATE INDEX idx_inventory_product_id ON inventory (product_id);
CREATE INDEX idx_inventory_warehouse_id ON inventory (warehouse_id);
CREATE INDEX idx_inventory_created_by_id ON inventory (created_by_id);
CREATE INDEX idx_inventory_created_at ON inventory (created_at);
CREATE INDEX idx_inventory_updated_at ON inventory (updated_at);
CREATE INDEX idx_inventory_deleted_at ON inventory (deleted_at);

CREATE TRIGGER update_updated_at_on_inventory_table
    BEFORE UPDATE
    ON inventory
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

create table task_assignments
(
    id            uuid primary key default uuid_generate_v4(),
    user_id       uuid                           not null,
    task_id       uuid                           not null,
    created_by_id uuid                           not null,
    created_at    timestamptz      default now() not null,
    deleted_at    timestamptz,
    unique nulls not distinct (user_id, task_id, deleted_at),
    foreign key (user_id) references users (id),
    foreign key (task_id) references tasks (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_task_assignments_user_id ON task_assignments (user_id);
CREATE INDEX idx_task_assignments_task_idd ON task_assignments (task_id);
CREATE INDEX idx_task_assignments_created_by_id ON task_assignments (created_by_id);
CREATE INDEX idx_task_assignments_created_at ON task_assignments (created_at);
CREATE INDEX idx_task_assignments_deleted_at ON task_assignments (deleted_at);

create table project_assignments
(
    id            uuid primary key default uuid_generate_v4(),
    user_id       uuid                           not null,
    project_id    uuid                           not null,
    created_by_id uuid                           not null,
    created_at    timestamptz      default now() not null,
    deleted_at    timestamptz,
    unique nulls not distinct (user_id, project_id, deleted_at),
    foreign key (user_id) references users (id),
    foreign key (project_id) references tasks (id),
    foreign key (created_by_id) references users (id)
);

CREATE INDEX idx_project_assignments_user_id ON project_assignments (user_id);
CREATE INDEX idx_project_assignments_project_id ON project_assignments (project_id);
CREATE INDEX idx_project_assignments_created_by_id ON project_assignments (created_by_id);
CREATE INDEX idx_project_assignments_created_at ON project_assignments (created_at);
CREATE INDEX idx_project_assignments_deleted_at ON project_assignments (deleted_at);
