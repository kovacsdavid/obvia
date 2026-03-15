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

use std::str::FromStr;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    common::types::{ValueObject, value_object::ValueObjectError},
    tenant::activity_feed::types::ResourceType,
};

#[derive(Deserialize)]
pub struct ActivityFeedRawQuery {
    resource_id: Uuid,
    resource_type: String,
    q: Option<String>,
}

impl ActivityFeedRawQuery {
    pub fn resource_id(&self) -> Uuid {
        self.resource_id
    }
    pub fn resource_type(&self) -> Result<ValueObject<ResourceType>, ValueObjectError> {
        ValueObject::new(ResourceType::from_str(&self.resource_type)?)
    }
    pub fn q(&self) -> &str {
        match &self.q {
            Some(v) => v,
            None => "",
        }
    }
}
