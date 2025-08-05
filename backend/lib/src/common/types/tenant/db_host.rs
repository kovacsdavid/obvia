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

use crate::common::types::value_object::{ValueObject, ValueObjectable};
use regex::Regex;
use serde::Deserialize;
use std::fmt::Display;
use std::net::IpAddr;
use std::str::FromStr;

/// Represents the hostname or address of a database.
///
/// The `DbHost` struct is a simple wrapper around a `String` that is used
/// to encapsulate the hostname or address of a database server. This can
/// help ensure type safety and improve code readability by explicitly
/// conveying the purpose of the contained value.
///
/// # Fields
///
/// * `0`: The inner `String` containing the hostname or address of the database.
#[derive(Debug, Clone)]
pub struct DbHost(pub String);

impl Display for DbHost {
    /// Implements the `fmt` method from the `std::fmt::Display` or `std::fmt::Debug` trait,
    /// enabling a custom display of the struct or type.
    ///
    /// # Parameters
    /// - `&self`: A reference to the instance of the type implementing this method.
    /// - `f`: A mutable reference to a `std::fmt::Formatter` used for formatting output.
    ///
    /// # Returns
    /// - `std::fmt::Result`: Indicates whether the formatting operation was successful
    ///   (`Ok(())`) or an error occurred (`Err`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValueObjectable for DbHost {
    type DataType = String;

    /// Validates the input string stored in the `self.0` field. The function checks whether the string
    /// represents a valid global IP address or a valid domain name (subject to specific constraints).
    ///
    /// # Validation Process
    /// - If the string can be parsed into an IP address, it ensures that the IP address is global
    ///   (not a private or reserved address).
    /// - If the string is not a valid IP address, it checks whether the string matches the structure
    ///   of a valid domain name using a regular expression. Additionally, it ensures that the string
    ///   does not include ".local" or "localhost", which are not considered valid.
    ///
    /// # Returns
    /// - `Ok(())`: If the string passes all validation checks.
    /// - `Err(String)`: Returns an error with the message "Hibás adatbázis kiszolgáló" (Hungarian for
    ///   "Invalid database server") if the string is determined to be invalid.
    ///
    /// # Errors
    /// - The function relies on `IpAddr::from_str` and `Regex::new`. If either fails (invalid input
    ///   or regex compilation error), the function will handle the failure within the matching logic.
    fn validate(&self) -> Result<(), String> {
        let res = match IpAddr::from_str(&self.0) {
            Ok(ip) => is_global(&ip),
            Err(_) => match Regex::new(
                r##"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"##,
            ) {
                Ok(re) => {
                    re.is_match(&self.0)
                        && !self.0.contains(".local")
                        && !self.0.contains("localhost")
                }
                Err(_) => false,
            },
        };
        match res {
            true => Ok(()),
            false => Err(String::from("Hibás adatbázis kiszolgáló")),
        }
    }

    /// Retrieves a reference to the value contained within the struct.
    ///
    /// # Returns
    /// A reference to the internal value of type `Self::DataType`.
    fn get_value(&self) -> &Self::DataType {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValueObject<DbHost> {
    /// Custom deserialization function for a type that implements deserialization using Serde.
    ///
    /// This function takes a Serde deserializer and attempts to parse the input into a `String`.
    /// It then wraps the string in a `DbHost` and validates it by calling `ValueObject::new`.
    /// If the validation fails, a custom deserialization error is returned.
    ///
    /// # Type Parameters
    /// - `D`: The type of the deserializer, which must implement `serde::Deserializer<'de>`.
    ///
    /// # Parameters
    /// - `deserializer`: The deserializer used to deserialize the input.
    ///
    /// # Returns
    /// - `Result<Self, D::Error>`:
    ///   - On success, returns the constructed and validated object wrapped in `Ok`.
    ///   - On failure, returns a custom error wrapped in `Err`.
    ///
    /// # Errors
    /// - Returns a deserialization error if:
    ///   - The input cannot be deserialized into a `String`.
    ///   - Validation using `ValueObject::new` fails, causing the `map_err` call to propagate an error.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ValueObject::new(DbHost(s)).map_err(serde::de::Error::custom)
    }
}

/// Determines whether a given IP address is globally reachable.
///
/// This function checks if the supplied `IpAddr` (IPv4 or IPv6) is globally
/// routable, meaning it is not reserved, private, loopback, or restricted
/// for specific protocols or purposes. It considers various constraints and
/// exclusions outlined by official standards (IANA/IETF).
///
/// # Parameters
/// - `ip`: A reference to an `IpAddr` (IPv4 or IPv6) to be evaluated.
///
/// # Returns
/// - `true`: If the IP address is globally reachable.
/// - `false`: If the IP address falls within one or more restricted categories
///   (e.g., loopback, private range, link-local, etc.).
///
/// # IPv4 Checks:
/// - Excludes the `0.0.0.0/8` block used for "This Network."
/// - Excludes private address ranges (e.g., `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`).
/// - Excludes loopback addresses (`127.0.0.0/8`).
/// - Excludes link-local addresses (`169.254.0.0/16`).
/// - Excludes most of the `192.0.0.0/24` block reserved for future use, except for
///   `.9` and `.10`, which are considered globally reachable.
/// - Excludes documentation addresses (`192.0.2.0/24`, `198.51.100.0/24`, `203.0.113.0/24`).
/// - Excludes broadcast addresses (e.g., `255.255.255.255`).
///
/// # IPv6 Checks:
/// - Excludes the unspecified address (`::/128`).
/// - Excludes the loopback address (`::1/128`).
/// - Excludes IPv4-mapped IPv6 addresses (`::ffff:0:0/96`).
/// - Excludes IPv4-IPv6 translation (`64:ff9b:1::/48`).
/// - Excludes the discard-only address block (`100::/64`).
/// - Excludes IETF protocol assignment address ranges (`2001::/23`), except for specific
///   globally routable anycast addresses (e.g., `2001:1::1`, `2001:1::2`, etc.).
/// - Excludes the 6to4 address block (`2002::/16`).
/// - Excludes certain address ranges reserved or used for specific purposes,
///   including the following:
///     - Segment Routing (`5f00::/16`)
///     - Unique local addresses (`fc00::/7`)
///     - Link-local addresses (`fe80::/10`)
///
/// # Notes
/// - Some unstable features (e.g., `is_benchmarking`, `is_reserved`) are commented
///   out, as they are not currently supported in the Rust standard library.
/// - You can replace this function after rust stabilizes it in standard library: https://github.com/rust-lang/rust/issues/27709
#[allow(clippy::manual_range_contains)]
fn is_global(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            !(ipv4.octets()[0] == 0 // "This network"
            || ipv4.is_private()
            // || ipv4.is_shared() // currently unstable
            || ipv4.is_loopback()
            || ipv4.is_link_local()
            // addresses reserved for future protocols (`192.0.0.0/24`)
            // .9 and .10 are documented as globally reachable so they're excluded
            || (
            ipv4.octets()[0] == 192 && ipv4.octets()[1] == 0 && ipv4.octets()[2] == 0
                && ipv4.octets()[3] != 9 && ipv4.octets()[3] != 10
        )
            || ipv4.is_documentation()
            // || ipv4.is_benchmarking() // currently unstable
            // || ipv4.is_reserved() // currently unstable
            || ipv4.is_broadcast())
        }
        IpAddr::V6(ipv6) => {
            !(ipv6.is_unspecified()
            || ipv6.is_loopback()
            // IPv4-mapped Address (`::ffff:0:0/96`)
            || matches!(ipv6.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
            // IPv4-IPv6 Translat. (`64:ff9b:1::/48`)
            || matches!(ipv6.segments(), [0x64, 0xff9b, 1, _, _, _, _, _])
            // Discard-Only Address Block (`100::/64`)
            || matches!(ipv6.segments(), [0x100, 0, 0, 0, _, _, _, _])
            // IETF Protocol Assignments (`2001::/23`)
            || (matches!(ipv6.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)
            && !(
            // Port Control Protocol Anycast (`2001:1::1`)
            u128::from_be_bytes(ipv6.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0001
                // Traversal Using Relays around NAT Anycast (`2001:1::2`)
                || u128::from_be_bytes(ipv6.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0002
                // AMT (`2001:3::/32`)
                || matches!(ipv6.segments(), [0x2001, 3, _, _, _, _, _, _])
                // AS112-v6 (`2001:4:112::/48`)
                || matches!(ipv6.segments(), [0x2001, 4, 0x112, _, _, _, _, _])
                // ORCHIDv2 (`2001:20::/28`)
                // Drone Remote ID Protocol Entity Tags (DETs) Prefix (`2001:30::/28`)`
                || matches!(ipv6.segments(), [0x2001, b, _, _, _, _, _, _] if b >= 0x20 && b <= 0x3F)
        ))
            // 6to4 (`2002::/16`) – it's not explicitly documented as globally reachable,
            // IANA says N/A.
            || matches!(ipv6.segments(), [0x2002, _, _, _, _, _, _, _])
            // || ipv6.is_documentation() // currently unstable
            // Segment Routing (SRv6) SIDs (`5f00::/16`)
            || matches!(ipv6.segments(), [0x5f00, ..])
            || ipv6.is_unique_local()
            || ipv6.is_unicast_link_local())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_db_host() {
        let valid_hosts = vec![
            r#"example.com"#,
            r#"postgres.example.com"#,
            r#"example.hu"#,
            r#"postgres.example.hu"#,
            r#"8.8.8.8"#,
            r#"8.8.4.4"#,
            r#"224c:ac51:1517:9700:fcec:3e03:0bb6:9e2e"#,
            r#"1e94:7af0:339b:2402:e58a:89c2:db5b:75a5"#,
        ];
        for host in valid_hosts {
            //panic!("{}", host);
            let db_host: ValueObject<DbHost> =
                serde_json::from_str(format!("\"{}\"", &host).as_str()).unwrap();
            assert_eq!(db_host.extract().get_value(), host);
        }
    }
    #[test]
    fn test_invalid_db_host() {
        let invalid_hosts = vec![
            r#"192.168.1.1"#,
            r#"192.168.1.255"#,
            r#"192.168.1.0/24"#,
            r#"255.255.255.255"#,
            r#"0.0.0.0"#,
            r#"0.0.0.0/0"#,
            r#"10.10.10.10"#,
            r#"10.10.10.255"#,
            r#"10.0.0.0"#,
            r#"10.0.0.255"#,
            r#"10.0.0.0/24"#,
            r#"10.0.0.1"#,
            r#"172.16.0.0"#,
            r#"172.16.0.255"#,
            r#"172.16.0.0/24"#,
            r#"172.16.0.1"#,
            r#"172.18.0.0"#,
            r#"172.18.0.255"#,
            r#"172.18.0.0/24"#,
            r#"172.18.0.1"#,
            r#"localhost"#,
            r#"anything.localhost"#,
            r#"anything.local"#,
            r#"example..com"#,
            r#"'example.com"#,
            r#"--example.com"#,
            r#"127.0.0.1"#,
            r#"::"#,
            r#"::/128"#,
            r#"::1"#,
            r#"::1/128"#,
            r#":"#,
            r#""#,
            r#" "#,
        ];
        for host in invalid_hosts {
            let db_host: Result<ValueObject<DbHost>, _> =
                serde_json::from_str(format!("\"{}\"", &host).as_str());
            assert!(db_host.is_err());
        }
    }
}
