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
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the structure of the claims contained in a JSON Web Token (JWT).
/// These claims provide metadata and security information for the token.
///
/// # Fields:
///
/// * `sub` - The subject of the token, which represents the user's unique identifier
///   (e.g., a UUID as a string).
///
/// * `exp` - The expiration timestamp of the token in UNIX time. Indicates the time
///   after which the token is no longer valid.
///
/// * `iat` - The issued-at timestamp of the token in UNIX time. Specifies when the token
///   was created.
///
/// * `nbf` - The "not valid before" timestamp in UNIX time. Indicates the time before
///   which the token is not considered valid.
///
/// * `iss` - The issuer of the token, typically representing the domain or service name
///   that issued the token
///
/// * `aud` - The audience for the token, identifying the intended recipient(s) of the
///   token (e.g., frontend client ID or domain).
///
/// * `jti` - A unique identifier for the token, typically a UUID, which ensures that
///   each token is unique. This can help prevent token reuse or replay attacks.
///
/// # Usage:
///
/// This struct is typically used to validate, decode, or create JWTs in the application.
/// By accessing its fields, you can ensure the token's integrity, validate its audience,
/// and enforce time-based constraints.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Claims {
    sub: Uuid,
    exp: usize,
    iat: usize,
    nbf: usize,
    iss: String,
    aud: String,
    jti: Uuid,
    family_id: Option<Uuid>,
    active_tenant: Option<Uuid>,
}

impl Claims {
    /// Creates a new instance of Claims.
    ///
    /// # Parameters
    ///
    /// - `sub` (`String`): The subject of the token, which represents the user's unique identifier (e.g., a UUID as a string).
    /// - `exp` (`usize`): The expiration timestamp of the token in UNIX time. Indicates the time after which the token is no longer valid.
    /// - `iat` (`usize`): The issued-at timestamp of the token in UNIX time. Specifies when the token was created.
    /// - `nbf` (`usize`): The "not valid before" timestamp in UNIX time. Indicates the time before which the token is not considered valid.
    /// - `iss` (`String`): The issuer of the token, typically representing the domain or service name that issued the token
    /// - `aud` (`String`): The audience for the token, identifying the intended recipient(s) of the token (e.g., frontend client ID or domain).
    /// - `jti` (`String`): A unique identifier for the token, typically a UUID, which ensures that each token is unique. This can help prevent token reuse or replay attacks.
    ///
    /// # Returns
    ///
    /// Returns a new instance populated with the provided values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sub: Uuid,
        exp: usize,
        iat: usize,
        nbf: usize,
        iss: String,
        aud: String,
        jti: Uuid,
        family_id: Option<Uuid>,
        active_tenant: Option<Uuid>,
    ) -> Self {
        Self {
            sub,
            exp,
            iat,
            nbf,
            iss,
            aud,
            jti,
            family_id,
            active_tenant,
        }
    }

    /// Attempts to create an instance of the struct by decoding and validating a JWT.
    ///
    /// This function validates the provided JWT string (`s`) using the supplied decoding key (`decoding_key`)
    /// and enforces specific claims (`"sub"`, `"exp"`, `"iat"`, `"nbf"`, `"iss"`, `"aud"`, `"jti"`).
    ///
    /// # Parameters
    ///
    /// - `s`: A reference to the JWT string token to decode and validate.
    /// - `decoding_key`: A byte slice containing the secret key used to decode the token.
    /// - `iss`: The expected issuer (claims field `"iss"`) of the token for validation.
    /// - `aud`: The expected audience (claims field `"aud"`) of the token for validation.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)`: Returns an instance of the struct if the token is successfully decoded and all validations pass.
    /// - `Err(String)`: Returns an error string if the token is invalid or any validation (e.g., claim checks) fails.
    ///
    /// # Errors
    ///
    /// Returns an error with the message `"Invalid token"` if decoding fails or the token is invalid.
    pub fn from_token(s: &str, decoding_key: &[u8], iss: &str, aud: &str) -> Result<Self, String> {
        let mut validator = Validation::new(Algorithm::HS256);
        validator.validate_nbf = true;
        validator.set_issuer(&[iss]);
        validator.set_audience(&[aud]);
        validator.set_required_spec_claims(&["sub", "exp", "iat", "nbf", "iss", "aud", "jti"]);
        Ok(
            decode::<Claims>(s, &DecodingKey::from_secret(decoding_key), &validator)
                .map_err(|_| String::from("Invalid token"))?
                .claims,
        )
    }

    /// Converts the current instance of the object into a valid token string using a provided encoding key.
    ///
    /// # Arguments
    ///
    /// * `encoding_key` - A byte slice representing the secret key used for encoding the token.
    ///
    /// # Returns
    ///
    /// This function returns a `Result`:
    /// * `Ok(String)` - Contains the encoded token as a `String` if the operation is successful.
    /// * `Err(String)` - Contains an error message as a `String` if the encoding process fails.
    ///
    /// # Errors
    ///
    /// Returns an `Err` with the message `"Could not encode token"` if the token encoding fails.
    pub fn to_token(&self, encoding_key: &[u8]) -> Result<String, String> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(encoding_key),
        )
        .map_err(|_| String::from("Could not encode token"))
    }

    /// Returns a reference to the `sub` field of the object.
    /// `sub` The subject of the token, which represents the user's unique identifier (e.g., a UUID as a string).
    pub fn sub(&self) -> Uuid {
        self.sub
    }
    /// Returns a reference to the `exp` field of the object.
    /// `exp` The expiration timestamp of the token in UNIX time. Indicates the time after which the token is no longer valid.
    #[allow(dead_code)]
    pub fn exp(&self) -> usize {
        self.exp
    }
    /// Returns a reference to the `iat` field of the object.
    /// `iat` The issued-at timestamp of the token in UNIX time. Specifies when the token was created.
    #[allow(dead_code)]
    pub fn iat(&self) -> usize {
        self.iat
    }
    /// Returns a reference to the `nbf` field of the object.
    /// `nbf` The "not valid before" timestamp in UNIX time. Indicates the time before which the token is not considered valid.
    #[allow(dead_code)]
    pub fn nbf(&self) -> usize {
        self.nbf
    }
    /// Returns a reference to the `iss` field of the object.
    /// `iss` The issuer of the token, typically representing the domain or service name that issued the token
    #[allow(dead_code)]
    pub fn iss(&self) -> &String {
        &self.iss
    }
    /// Returns a reference to the `aud` field of the object.
    /// `aud` The audience for the token, identifying the intended recipient(s) of the token (e.g., frontend client ID or domain).
    #[allow(dead_code)]
    pub fn aud(&self) -> &String {
        &self.aud
    }
    /// Returns a reference to the `jti` field of the object.
    /// `jti`A unique identifier for the token, typically a UUID, which ensures that each token is unique. This can help prevent token reuse or replay attacks.
    #[allow(dead_code)]
    pub fn jti(&self) -> Uuid {
        self.jti
    }

    /// Retrieves the family_id associated with the current context.
    pub fn family_id(&self) -> Option<Uuid> {
        self.family_id
    }

    /// Retrieves the UUID of the active tenant associated with the current context.
    pub fn active_tenant(&self) -> Option<Uuid> {
        self.active_tenant
    }

    /// Sets the active tenant for the current instance.
    ///
    /// This method allows you to set the `active_tenant` property of the instance
    /// by providing an `Option<Uuid>`. The `active_tenant` can either be `Some(Uuid)`
    /// to specify the active tenant or `None` to indicate no active tenant.
    pub fn set_active_tenant(mut self, active_tenant: Option<Uuid>) -> Self {
        self.active_tenant = active_tenant;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manager::app::config::AppConfigBuilder;
    use chrono::Local;
    use std::ops::{Add, Sub};
    use std::time::Duration;
    use uuid::Uuid;

    #[test]
    fn test_valid_claims() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token1 = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();
        let token2 = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();
        assert_eq!(token1, token2);
        assert_eq!(
            Claims::from_token(
                &token1,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                config.auth().jwt_audience()
            )
            .unwrap(),
            claims
        );
    }
    #[test]
    fn test_expired_claims() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().sub(Duration::from_secs(61)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();
        assert!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                config.auth().jwt_audience()
            )
            .is_err()
        );
    }
    #[test]
    fn test_invalid_not_before_claims() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().add(Duration::from_secs(61)).timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();
        assert!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                config.auth().jwt_audience()
            )
            .is_err()
        );
    }
    #[test]
    fn test_invalid_issuer_claims() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();

        assert!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                "invalid_issuer",
                config.auth().jwt_audience()
            )
            .is_err()
        );
    }
    #[test]
    fn test_invalid_audience_claims() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();

        assert!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                "invalid_audience"
            )
            .is_err()
        );
    }
    #[test]
    fn test_empty_active_tenant() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();

        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            None,
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();

        assert_eq!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                config.auth().jwt_audience()
            )
            .unwrap()
            .active_tenant(),
            None
        );
    }
    #[test]
    fn test_valid_active_tenant() {
        let config = AppConfigBuilder::default().build().unwrap();

        let exp = Local::now().add(Duration::from_secs(100)).timestamp();
        let iat = Local::now().timestamp();
        let nbf = Local::now().timestamp();
        let active_tenant_uuid = Uuid::new_v4();
        let claims = Claims::new(
            Uuid::new_v4(),
            usize::try_from(exp).unwrap(),
            usize::try_from(iat).unwrap(),
            usize::try_from(nbf).unwrap(),
            config.auth().jwt_issuer().to_string(),
            config.auth().jwt_audience().to_string(),
            Uuid::new_v4(),
            None,
            Some(active_tenant_uuid),
        );
        let token = claims
            .to_token(config.auth().jwt_secret().as_bytes())
            .unwrap();
        assert_eq!(
            Claims::from_token(
                &token,
                config.auth().jwt_secret().as_bytes(),
                config.auth().jwt_issuer(),
                config.auth().jwt_audience()
            )
            .unwrap()
            .active_tenant(),
            Some(active_tenant_uuid)
        );
    }
}
