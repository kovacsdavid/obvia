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

use crate::common::value_object::*;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct TaxCategory(String);

impl TaxCategory {
    pub const VALIDATION_ERROR: &'static str = "Hibás adó kategória";
}

impl ValueObjectData for TaxCategory {
    type DataType = String;

    fn new(data: &str) -> ValueObjectResult<Option<Self>> {
        let data_trim = data.trim();
        if !data_trim.is_empty() {
            Ok(Some(Self(data_trim.to_owned())))
        } else {
            Ok(None)
        }
    }
    fn validate(&self) -> Result<(), ValueObjectError> {
        match self.0.as_str() {
            // Taxable supplies (Adóköteles ügyletek)
            "AK" // Általános adókulcs
            // Exempt supplies (Adómentes ügyletek)
            |"AAM"            // Alanyi adómentes – XIII. fejezet
            | "TAM"          // Tárgyi adómentes – 85. §, 86. §
            | "KBAET"        // Közösségen belüli adómentes termékértékesítés – 89. §
            | "KBAUK"        // Új közlekedési eszköz Közösségen belüli értékesítése – 89. § (2)
            | "EAM"          // Termékexport – 98–109. §
            | "NAM"          // Nemzetközi ügyletekhez kapcsolódó mentesség – 93–95. §, 110–118. §
            // Outside the scope of Hungarian VAT (Áfa tárgyi hatályán kívül / külföldön teljesített)
            | "ATK"          // Tárgyi hatályon kívül – 2–3. §
            | "EUFAD37"      // Másik tagállamban teljesített, 37. § (1) szerinti szolgáltatás
            | "EUFADE"       // Másik tagállamban teljesített, egyéb fordítottan adózó ügylet
            | "EUE"          // Másik tagállamban teljesített, nem fordítottan adózó ügylet
            | "HO"           // Harmadik országban teljesített ügylet
            // Free supplies treated as paid (Ingyenes, de ellenérték fejében végzettnek minősülő ügylet) – 11. §, 14. §
            | "REFUNDABLE_VAT"     // Az áfát a kedvezményezett megtéríti
            | "NONREFUNDABLE_VAT"  // Az áfát a kedvezményezett nem téríti meg
            => Ok(()),
            _ => Err(ValueObjectError::InvalidInput(Self::VALIDATION_ERROR)),
        }
    }

    fn get_data(&self) -> &Self::DataType {
        &self.0
    }
}

impl Display for TaxCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let category = "standard"
            .parse::<ValueObjectRequired<TaxCategory>>()
            .unwrap();
        assert_eq!(category.as_str().unwrap(), "standard");
    }
    #[test]
    fn test_invalid() {
        let category = "invalid".parse::<ValueObjectRequired<TaxCategory>>();
        assert!(category.is_err());
    }
}
