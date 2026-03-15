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

#![allow(dead_code)]
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;
use thiserror::Error;

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};

#[derive(Error, Debug, PartialEq)]
pub enum GetQueryError {
    #[error("{0}")]
    InvalidInput(String),

    #[error("Unexpected QueryError: {0}")]
    Custom(String),
}

impl From<ValueObjectError> for GetQueryError {
    fn from(value: ValueObjectError) -> Self {
        match value {
            ValueObjectError::InvalidInput(e) => GetQueryError::InvalidInput(e.to_string()),
            _ => GetQueryError::Custom(value.to_string()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Order {
    Ascending,
    Descending,
}

impl FromStr for Order {
    type Err = GetQueryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "asc" => Order::Ascending,
            "desc" => Order::Descending,
            _ => return Err(GetQueryError::InvalidInput("ordering".to_string())),
        })
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        };

        write!(f, "{str}")
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Ordering<O>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    order_by: Option<ValueObject<O>>,
    order: Option<Order>,
}

impl<O> Ordering<O>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    pub fn order_by(&self) -> Option<String> {
        Some(self.order_by.as_ref()?.to_string())
    }
    pub fn order(&self) -> &Option<Order> {
        &self.order
    }
}

impl<T> FromStr for Ordering<T>
where
    T: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = GetQueryError;
    fn from_str(s: &str) -> Result<Ordering<T>, Self::Err> {
        let collection: Vec<String> = s
            .replace("ordering:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            Ok(Ordering {
                order_by: Some(ValueObject::new(T::from_str(&collection[0])?)?),
                order: Some(Order::from_str(&collection[1])?),
            })
        } else {
            Ok(Ordering {
                order_by: None,
                order: None,
            })
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Paging {
    page: Option<u64>,
    limit: Option<u64>,
}

impl Paging {
    pub fn page(&self) -> Option<u64> {
        self.page
    }
    pub fn limit(&self) -> Option<u64> {
        self.limit
    }
    pub fn offset(&self) -> Option<u64> {
        Some((self.page? - 1) * self.limit?)
    }
}

impl FromStr for Paging {
    type Err = GetQueryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collection: Vec<String> = s
            .replace("paging:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            let page = collection[0]
                .parse::<u64>()
                .map_err(|_| GetQueryError::InvalidInput("paging".to_string()))?;
            let limit = collection[1]
                .parse::<u64>()
                .map_err(|_| GetQueryError::InvalidInput("paging".to_string()))?;
            Ok(Self {
                page: Some(page),
                limit: Some(limit),
            })
        } else {
            Ok(Self {
                page: None,
                limit: None,
            })
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Filtering<F>
where
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    filter_by: Option<ValueObject<F>>,
    value: Option<String>,
}

impl<F> Filtering<F>
where
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    // Secruity: ValueObject
    pub fn filter_by(&self) -> Option<&str> {
        self.filter_by.as_ref().map(|v| v.as_str())
    }
    // Secruity: Unchecked user input! You can only use this in bind queries!
    pub fn value_unchecked(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

impl<F> FromStr for Filtering<F>
where
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = GetQueryError;
    fn from_str(s: &str) -> Result<Filtering<F>, Self::Err> {
        let collection: Vec<String> = s
            .replace("filtering:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            let value = collection[1].replace("|", "");
            Ok(Filtering {
                filter_by: Some(ValueObject::new(F::from_str(&collection[0])?)?),
                value: Some(value),
            })
        } else {
            Ok(Filtering {
                filter_by: None,
                value: None,
            })
        }
    }
}

fn extract_field<'a>(s: &'a str, field: &str) -> &'a str {
    if let Some(start) = s.find(field) {
        let rest = &s[start..];

        let mut inside_pipes = false;
        let mut end = rest.len();

        for (i, c) in rest.char_indices() {
            match c {
                '|' => inside_pipes = !inside_pipes,
                ' ' if !inside_pipes && i > 0 => {
                    end = i;
                    break;
                }
                _ => {}
            }
        }
        &rest[..end]
    } else {
        ""
    }
}

#[derive(PartialEq, Debug)]
pub struct GetQuery<O, F>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    ordering: Ordering<O>,
    paging: Paging,
    filtering: Filtering<F>,
}

impl<O, F> GetQuery<O, F>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    pub fn ordering(&self) -> &Ordering<O> {
        &self.ordering
    }
    pub fn paging(&self) -> &Paging {
        &self.paging
    }
    pub fn filtering(&self) -> &Filtering<F> {
        &self.filtering
    }
}

impl<O, F> FromStr for GetQuery<O, F>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = GetQueryError;
    fn from_str(s: &str) -> Result<GetQuery<O, F>, Self::Err> {
        Ok(GetQuery {
            ordering: Ordering::from_str(extract_field(s, "ordering:"))?,
            paging: Paging::from_str(extract_field(s, "paging:"))?,
            filtering: Filtering::from_str(extract_field(s, "filtering:"))?,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommonRawQuery {
    q: Option<String>,
}

impl CommonRawQuery {
    pub fn q(&self) -> &str {
        match &self.q {
            Some(v) => v,
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObjectable;
    use crate::common::types::value_object::ValueObjectError;
    use serde::Serialize;

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct TestOrderBy(pub String);

    impl ValueObjectable for TestOrderBy {
        type DataType = String;

        fn validate(&self) -> Result<(), ValueObjectError> {
            match self.0.trim() {
                "test" | "name" => Ok(()),
                _ => Err(ValueObjectError::InvalidInput("Hibás sorrend formátum")),
            }
        }

        fn get_value(&self) -> &Self::DataType {
            &self.0
        }
    }

    impl Display for TestOrderBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for TestOrderBy {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.to_string()))
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct TestFilterBy(pub String);

    impl ValueObjectable for TestFilterBy {
        type DataType = String;

        fn validate(&self) -> Result<(), ValueObjectError> {
            match self.0.trim() {
                "test" | "type" => Ok(()),
                _ => Err(ValueObjectError::InvalidInput("Hibás sorrend formátum")),
            }
        }

        fn get_value(&self) -> &Self::DataType {
            &self.0
        }
    }

    impl Display for TestFilterBy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for TestFilterBy {
        type Err = ValueObjectError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.to_string()))
        }
    }

    impl<O, F> GetQuery<O, F>
    where
        O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
        F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    {
        pub fn new(ordering: Ordering<O>, paging: Paging, filtering: Filtering<F>) -> Self {
            Self {
                ordering,
                paging,
                filtering,
            }
        }
    }

    impl<O> Ordering<O>
    where
        O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    {
        pub fn new(field: Option<ValueObject<O>>, value: Option<Order>) -> Self {
            Self {
                order_by: field,
                order: value,
            }
        }
    }

    impl Paging {
        pub fn new(page: Option<u64>, limit: Option<u64>) -> Self {
            Self { page, limit }
        }
    }

    impl<T> Filtering<T>
    where
        T: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    {
        pub fn new(
            field: Option<ValueObject<T>>, // Secruity: ValueObject
            value: Option<String>,         // Secruity: You can use this only in bind queries!
        ) -> Self {
            Self {
                filter_by: field,
                value,
            }
        }
    }

    #[test]
    fn test_query_from_str() {
        let test_str = "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = GetQuery::new(
            Ordering::new(
                Some(ValueObject::new(TestOrderBy("test".to_string())).unwrap()),
                Some(Order::Ascending),
            ),
            Paging::new(Some(1), Some(25)),
            Filtering::new(
                Some(ValueObject::new(TestFilterBy("test".to_string())).unwrap()),
                Some("warehouse 1".to_string()),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_different_data() {
        let test_str = "ordering:name-desc paging:3-30 filtering:type-|some type|";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = GetQuery::new(
            Ordering::new(
                Some(ValueObject::new(TestOrderBy("name".to_string())).unwrap()),
                Some(Order::Descending),
            ),
            Paging::new(Some(3), Some(30)),
            Filtering::new(
                Some(ValueObject::new(TestFilterBy("type".to_string())).unwrap()),
                Some("some type".to_string()),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_different_order() {
        let test_str = "filtering:type-|some type| paging:3-30 ordering:name-desc";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = GetQuery::new(
            Ordering::new(
                Some(ValueObject::new(TestOrderBy("name".to_string())).unwrap()),
                Some(Order::Descending),
            ),
            Paging::new(Some(3), Some(30)),
            Filtering::new(
                Some(ValueObject::new(TestFilterBy("type".to_string())).unwrap()),
                Some("some type".to_string()),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_trailing_spaces() {
        let test_str = "     filtering:type-|some type| paging:3-30 ordering:name-desc    ";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = GetQuery::new(
            Ordering::new(
                Some(ValueObject::new(TestOrderBy("name".to_string())).unwrap()),
                Some(Order::Descending),
            ),
            Paging::new(Some(3), Some(30)),
            Filtering::new(
                Some(ValueObject::new(TestFilterBy("type".to_string())).unwrap()),
                Some("some type".to_string()),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_defaults() {
        let test_str = "";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();

        let query_constructed = GetQuery::new(
            Ordering::new(None, None),
            Paging::new(None, None),
            Filtering::new(None, None),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_partial_1() {
        let test_str = "ordering:name-desc";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();

        let query_constructed = GetQuery::new(
            Ordering::new(
                Some(ValueObject::new(TestOrderBy("name".to_string())).unwrap()),
                Some(Order::Descending),
            ),
            Paging::new(None, None),
            Filtering::new(None, None),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_partial_2() {
        let test_str = "paging:3-30";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();

        let query_constructed = GetQuery::new(
            Ordering::new(None, None),
            Paging::new(Some(3), Some(30)),
            Filtering::new(None, None),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_partial_3() {
        let test_str = "filtering:type-|some type|";
        let query_from_str = GetQuery::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();

        let query_constructed = GetQuery::new(
            Ordering::new(None, None),
            Paging::new(None, None),
            Filtering::new(
                Some(ValueObject::new(TestFilterBy("type".to_string())).unwrap()),
                Some("some type".to_string()),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }
}
