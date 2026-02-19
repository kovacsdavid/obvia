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

use std::str::FromStr;
use thiserror::Error;

use crate::common::types::{ValueObject, ValueObjectable, value_object::ValueObjectError};

#[derive(Error, Debug, PartialEq)]
enum QueryError {
    #[error("{0}")]
    InvalidInput(String),

    #[error("Unexpected QueryError: {0}")]
    Custom(String),
}

impl From<ValueObjectError> for QueryError {
    fn from(value: ValueObjectError) -> Self {
        match value {
            ValueObjectError::InvalidInput(e) => QueryError::InvalidInput(e.to_string()),
            _ => QueryError::Custom(value.to_string()),
        }
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum Order {
    Ascending,
    Descending,
}

impl FromStr for Order {
    type Err = QueryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "asc" => Order::Ascending,
            "desc" => Order::Descending,
            _ => return Err(QueryError::InvalidInput("ordering".to_string())),
        })
    }
}

#[derive(PartialEq, Debug)]
struct Ordering<O>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    field: ValueObject<O>,
    value: Order,
}

impl<T> FromStr for Ordering<T>
where
    T: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = QueryError;
    fn from_str(s: &str) -> Result<Ordering<T>, Self::Err> {
        let collection: Vec<String> = s
            .replace("ordering:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            Ok(Ordering {
                field: ValueObject::new(T::from_str(&collection[0])?)?,
                value: Order::from_str(&collection[1])?,
            })
        } else {
            Err(QueryError::InvalidInput("ordering".to_string()))
        }
    }
}

#[derive(PartialEq, Debug)]
struct Paging {
    page: u64,
    limit: u64,
}

impl Paging {
    #[allow(dead_code)]
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.limit
    }
}

impl FromStr for Paging {
    type Err = QueryError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let collection: Vec<String> = s
            .replace("paging:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            let page = collection[0]
                .parse::<u64>()
                .map_err(|_| QueryError::InvalidInput("paging".to_string()))?;
            let limit = collection[1]
                .parse::<u64>()
                .map_err(|_| QueryError::InvalidInput("paging".to_string()))?;
            Ok(Self { page, limit })
        } else {
            Err(QueryError::InvalidInput("paging".to_string()))
        }
    }
}

#[derive(PartialEq, Debug)]
struct Filtering<F>
where
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    field: ValueObject<F>, // Secruity: ValueObject
    value: String,         // Secruity: You can use this only in bind queries!
}

impl<F> FromStr for Filtering<F>
where
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = QueryError;
    fn from_str(s: &str) -> Result<Filtering<F>, Self::Err> {
        let collection: Vec<String> = s
            .replace("filtering:", "")
            .split("-")
            .map(|v| v.to_string())
            .collect();
        if collection.len() == 2 {
            let value = collection[1].replace("|", "");
            Ok(Filtering {
                field: ValueObject::new(F::from_str(&collection[0])?)?,
                value,
            })
        } else {
            Err(QueryError::InvalidInput("filtering".to_string()))
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

fn preprocess_query_str(s: &str) -> (&str, &str, &str) {
    (
        extract_field(s, "ordering:"),
        extract_field(s, "paging:"),
        extract_field(s, "filtering:"),
    )
}

#[derive(PartialEq, Debug)]
struct Query<O, F>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    ordering: Ordering<O>,
    paging: Paging,
    filtering: Filtering<F>,
}

impl<O, F> FromStr for Query<O, F>
where
    O: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    F: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
{
    type Err = QueryError;
    fn from_str(s: &str) -> Result<Query<O, F>, Self::Err> {
        let (ordering_s, paging_s, filtering_s) = preprocess_query_str(s);

        Ok(Query {
            ordering: Ordering::from_str(ordering_s)?,
            paging: Paging::from_str(paging_s)?,
            filtering: Filtering::from_str(filtering_s)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::ValueObjectable;
    use crate::common::types::value_object::ValueObjectError;
    use serde::Serialize;
    use std::fmt::Display;

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

    impl<O, F> Query<O, F>
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
        pub fn new(field: ValueObject<O>, value: Order) -> Self {
            Self { field, value }
        }
    }

    impl Paging {
        pub fn new(page: u64, limit: u64) -> Self {
            Self { page, limit }
        }
    }

    impl<T> Filtering<T>
    where
        T: ValueObjectable<DataType = String> + FromStr<Err = ValueObjectError>,
    {
        pub fn new(
            field: ValueObject<T>, // Secruity: ValueObject
            value: String,         // Secruity: You can use this only in bind queries!
        ) -> Self {
            Self { field, value }
        }
    }

    #[test]
    fn test_query_from_str() {
        let test_str = "ordering:test-asc paging:1-25 filtering:test-|warehouse 1|";
        let query_from_str = Query::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = Query::new(
            Ordering::new(
                ValueObject::new(TestOrderBy("test".to_string())).unwrap(),
                Order::Ascending,
            ),
            Paging::new(1, 25),
            Filtering::new(
                ValueObject::new(TestFilterBy("test".to_string())).unwrap(),
                "warehouse 1".to_string(),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }

    #[test]
    fn test_query_from_str_different_data() {
        let test_str = "ordering:name-desc paging:3-30 filtering:type-|some type|";
        let query_from_str = Query::<TestOrderBy, TestFilterBy>::from_str(test_str).unwrap();
        let query_constructed = Query::new(
            Ordering::new(
                ValueObject::new(TestOrderBy("name".to_string())).unwrap(),
                Order::Descending,
            ),
            Paging::new(3, 30),
            Filtering::new(
                ValueObject::new(TestFilterBy("type".to_string())).unwrap(),
                "some type".to_string(),
            ),
        );
        assert_eq!(query_from_str, query_constructed);
    }
}
