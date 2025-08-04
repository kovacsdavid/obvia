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
pub struct DbHost(String);

impl DbHost {
    /// Returns a string slice (`&str`) referencing the inner string data.
    ///
    /// # Notes
    /// - This function borrows the inner string (`self.0`) as a shared reference.
    ///
    /// # Allowance
    /// The `#[allow(dead_code)]` attribute indicates that the function may not always be used and avoids warnings during compilation.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.0
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

/// Validates whether a given string is a valid database host.
///
/// The function checks the input string against the following criteria:
/// 1. If the input is an IP address, it uses the `is_global` function to determine
///    if the IP address is globally routable.
/// 2. If the input is not an IP address, it validates the string as a hostname
///    using a regex pattern.
/// 3. The hostname must not include ".local" or "localhost".
///
/// If the input fails either IP address validation or hostname validation, the function returns `false`.
///
/// # Arguments
///
/// * `s` - A string slice that holds the potential database host to validate.
///
/// # Returns
///
/// * `true` if the provided string is a valid globally routable IP address or a valid hostname that does not contain ".local" or "localhost".
/// * `false` otherwise.
///
/// # Errors
///
/// * If a regular expression fails to compile, the function will return `false`.
fn is_valid_db_host(s: &str) -> bool {
    match IpAddr::from_str(s) {
        Ok(ip) => is_global(&ip),
        Err(_) => match Regex::new(
            r##"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"##,
        ) {
            Ok(re) => re.is_match(s) && !s.contains(".local") && !s.contains("localhost"),
            Err(_) => false,
        },
    }
}

impl TryFrom<String> for DbHost {
    type Error = String;

    /// Attempts to create an instance of the type implementing this method from the given `String`.
    ///
    /// This function takes a `String` as input and tries to parse it into the desired type. If
    /// parsing is successful, it returns `Ok(Self)` containing the created instance.
    /// If parsing fails, it returns a `Result::Err` containing the appropriate error.
    ///
    /// # Arguments
    ///
    /// * `value` - A `String` that represents the source value to be parsed into the target type.
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - If the parsing is successful.
    /// * `Err(Self::Error)` - If the parsing fails, enclosing the error describing the failure.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided `String` cannot be parsed into the target type.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl FromStr for DbHost {
    type Err = String;

    /// Attempts to create an instance of `DbHost` from the given string slice.
    ///
    /// This function validates the provided string to ensure it meets the criteria
    /// for a valid database host. If the string is valid, it constructs a new
    /// `DbHost` instance and returns it wrapped in a `Result::Ok`. Otherwise,
    /// it returns a `Result::Err` containing an error message.
    ///
    /// # Parameters
    /// - `s`: A string slice representing the database string to be validated and used for creating a new `DbHost` instance.
    ///
    /// # Returns
    /// - `Ok(DbHost)`: If the string provided is a valid database host.
    /// - `Err(String)`: If the string is invalid, containing an error message.
    ///
    /// # Errors
    /// - Returns `"Hibás adatbázis kiszolgáló"` as the error message if validation fails.
    ///
    /// # Note
    /// The function `is_valid_db_host(s: &str)` is expected to perform the
    /// validation logic and must be defined elsewhere in the module.
    ///
    /// # Implements
    /// This function is a part of the `FromStr` trait implementation for the `DbHost` type,
    /// enabling string-to-`DbHost` conversions.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match is_valid_db_host(s) {
            true => Ok(DbHost(s.to_string())),
            false => Err(String::from("Hibás adatbázis kiszolgáló")),
        }
    }
}

impl<'de> Deserialize<'de> for DbHost {
    /// A custom implementation of the `deserialize` method for a type that can be deserialized
    /// from a string using the Serde library.
    ///
    /// # Type Parameters:
    /// - `D`: The deserializer type implementing the `serde::Deserializer` trait.
    ///
    /// # Parameters:
    /// - `deserializer`: A deserializer instance to read and interpret the input data
    ///   and convert it into the appropriate type.
    ///
    /// # Returns:
    /// - `Result<Self, D::Error>`: Returns either:
    ///   - The successfully deserialized instance of the type (`Self`).
    ///   - An error of type `D::Error` if deserialization fails.
    ///
    /// # Behavior:
    /// 1. The function first attempts to deserialize the input data into a `String`.
    /// 2. Then, it tries to parse the deserialized string into the target type (`Self`)
    ///    using the `parse` method.
    /// 3. If parsing fails, an error is returned using `serde::de::Error::custom` to
    ///    generate a descriptive error message.
    ///
    /// # Errors:
    /// - Returns an error if:
    ///   - The input data cannot be deserialized into a `String`.
    ///   - The parsed string cannot be converted into the type being deserialized.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Display for DbHost {
    /// Implements the `fmt` method for formatting the current type using the `Display` trait.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
            let db_host: DbHost = serde_json::from_str(format!("\"{}\"", &host).as_str()).unwrap();
            assert_eq!(db_host.as_str(), host);
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
            let db_host: Result<DbHost, _> =
                serde_json::from_str(format!("\"{}\"", &host).as_str());
            assert!(db_host.is_err());
        }
    }
}
