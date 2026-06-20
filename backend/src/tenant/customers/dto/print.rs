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

use crate::tenant::customers::model::CustomerResolved;
use chrono_tz::Tz;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, PartialEq, Debug)]
pub struct CustomerResolvedPrint {
    id: Uuid,
    name: String,
    contact_name: Option<String>,
    email: String,
    phone_number: Option<String>,
    status: String,
    customer_type: String,
    created_by_id: Uuid,
    created_by: String,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl CustomerResolvedPrint {
    pub fn from_customer_revolved(customer_resolved: CustomerResolved, tz: Tz) -> Self {
        let date_format_string = format!("%Y. %m. %d. %H:%M:%S ({tz})");
        Self {
            id: customer_resolved.id,
            name: customer_resolved.name,
            contact_name: customer_resolved.contact_name,
            email: customer_resolved.email,
            phone_number: customer_resolved.phone_number,
            status: Self::map_status(&customer_resolved.status),
            customer_type: Self::map_customer_type(&customer_resolved.customer_type),
            created_by_id: customer_resolved.created_by_id,
            created_by: customer_resolved.created_by,
            created_at: customer_resolved
                .created_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            updated_at: customer_resolved
                .updated_at
                .with_timezone(&tz)
                .format(&date_format_string)
                .to_string(),
            deleted_at: customer_resolved
                .deleted_at
                .map(|v| v.with_timezone(&tz).format(&date_format_string).to_string()),
        }
    }
    fn map_status(status: &str) -> String {
        match status {
            "active" => "Aktív",
            "lead" => "Érdeklődő",
            "prospect" => "Lehetséges vevő",
            "inactive" => "Inaktív",
            _ => "Ismeretlen státusz",
        }
        .to_string()
    }
    fn map_customer_type(customer_type: &str) -> String {
        match customer_type {
            "natural" => "Természetes személy",
            "legal" => "Jogi személy",
            _ => "Ismeretlen típus",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_from_customer_resolved() {
        let customer_id = Uuid::new_v4();
        let created_by_id = Uuid::new_v4();
        let input_date: DateTime<Utc> = "2026-01-01T01:00:00Z".parse().unwrap();
        let tz: Tz = "Europe/Budapest".parse().unwrap();
        let output_date = "2026. 01. 01. 02:00:00 (Europe/Budapest)".to_string();
        let customer_resolved = CustomerResolved {
            id: customer_id,
            name: "Teszt Elek".to_string(),
            contact_name: None,
            email: "teszt.elek@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "active".to_string(),
            customer_type: "natural".to_string(),
            created_by_id,
            created_by: "Kovács Dávid".to_string(),
            created_at: input_date,
            updated_at: input_date,
            deleted_at: None,
        };
        let customer_resolved_print =
            CustomerResolvedPrint::from_customer_revolved(customer_resolved, tz);
        let customer_resolved_print_expected = CustomerResolvedPrint {
            id: customer_id,
            name: "Teszt Elek".to_string(),
            contact_name: None,
            email: "teszt.elek@example.com".to_string(),
            phone_number: Some("+36301234567".to_string()),
            status: "Aktív".to_string(),
            customer_type: "Természetes személy".to_string(),
            created_by_id,
            created_by: "Kovács Dávid".to_string(),
            created_at: output_date.clone(),
            updated_at: output_date,
            deleted_at: None,
        };

        assert_eq!(customer_resolved_print, customer_resolved_print_expected);
    }
}
