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

#![allow(dead_code)]
use std::{fmt::Display, fs, path::Path, process::Command};

use indexmap::IndexMap;
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PdfGenError {
    #[error("Missing key required by template: {0}")]
    MissingParam(String),

    #[error("IOError: {0}")]
    IOError(String),

    #[error("Typst failed: {0}")]
    SubProcess(String),
}

pub type PdfGenResult<T> = Result<T, PdfGenError>;

#[derive(Debug)]
pub enum PdfTemplates {
    Test,
    CustomerView,
}

impl PdfTemplates {
    pub fn input_keys(&self) -> Vec<&'static str> {
        match &self {
            Self::Test => vec!["test", "name"],
            Self::CustomerView => vec![
                "customer_resolved_id",
                "customer_resolved_name",
                "customer_resolved_contact_name",
                "customer_resolved_email",
                "customer_resolved_phone_number",
                "customer_resolved_status",
                "customer_resolved_customer_type",
                "customer_resolved_created_by",
                "customer_resolved_created_at",
                "customer_resolved_updated_at",
            ],
        }
    }
}

impl Display for PdfTemplates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let template = match &self {
            Self::Test => "test",
            Self::CustomerView => "customer_view",
        };
        write!(f, "templates/{template}.typ")
    }
}

#[derive(Debug)]
struct PdfGen<'a> {
    path: &'a Path,
    template: &'a PdfTemplates,
    params: IndexMap<String, String>,
}

impl<'a> PdfGen<'a> {
    pub fn new(
        path: &'a Path,
        template: &'a PdfTemplates,
        params: &IndexMap<String, String>,
    ) -> PdfGenResult<Self> {
        let mut params_filtered: IndexMap<String, String> = IndexMap::new();
        for key in template.input_keys() {
            if !params.contains_key(key) {
                return Err(PdfGenError::MissingParam(key.to_owned()));
            }
            params_filtered.insert(key.to_owned(), params[key].to_owned());
        }
        Ok(PdfGen {
            path,
            template,
            params: params_filtered,
        })
    }
    pub fn path(&self) -> &Path {
        self.path
    }
    pub fn template(&self) -> &PdfTemplates {
        self.template
    }
    pub fn params(&self) -> &IndexMap<String, String> {
        &self.params
    }
    pub fn typst_compile_args(&self) -> Vec<String> {
        let mut args = vec![
            "compile".to_owned(),
            "-f".to_owned(),
            "pdf".to_owned(),
            self.template().to_string(),
            self.path().to_string_lossy().into_owned(),
        ];
        for (k, v) in self.params() {
            args.push("--input".to_owned());
            args.push(format!("{k}={v}"));
        }
        args
    }
}

pub fn gen_pdf_temporary(
    template: &PdfTemplates,
    params: &IndexMap<String, String>,
) -> PdfGenResult<Vec<u8>> {
    let tmp_file = NamedTempFile::new().map_err(|e| PdfGenError::IOError(e.to_string()))?;
    let pdf_gen = PdfGen::new(tmp_file.path(), template, params)?;
    let mut output = Command::new("typst");

    for arg in pdf_gen.typst_compile_args() {
        output.arg(arg);
    }

    let output = output
        .output()
        .map_err(|e| PdfGenError::IOError(e.to_string()))?;

    if !output.status.success() {
        return Err(PdfGenError::SubProcess(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }

    fs::read(pdf_gen.path()).map_err(|e| PdfGenError::IOError(e.to_string()))
}

pub fn gen_pdf_persistent<'a>(
    path: &'a Path,
    template: &PdfTemplates,
    params: &IndexMap<String, String>,
) -> PdfGenResult<&'a Path> {
    let pdf_gen = PdfGen::new(path, template, params)?;

    let mut output = Command::new("typst");

    for arg in pdf_gen.typst_compile_args() {
        output.arg(arg);
    }

    let output = output
        .output()
        .map_err(|e| PdfGenError::IOError(e.to_string()))?;

    if !output.status.success() {
        return Err(PdfGenError::SubProcess(
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    Ok(path)
}

pub fn index_map_key_prefix<T>(
    prefix: &'static str,
    index_map: IndexMap<String, T>,
) -> IndexMap<String, T> {
    index_map
        .into_iter()
        .map(|(key, value)| (format!("{prefix}_{key}"), value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pdf_gen_struct_params() {
        let path = Path::new("/home/test");
        let template = PdfTemplates::Test;
        let mut params = IndexMap::new();
        params.insert("test".to_owned(), "value1".to_owned());
        params.insert("name".to_owned(), "value2".to_owned());
        let pdf_gen = PdfGen::new(path, &template, &params).unwrap();
        assert_eq!(pdf_gen.path(), path);
        assert_eq!(&pdf_gen.template().to_string(), "templates/test.typ");
        assert_eq!(pdf_gen.params(), &params);
    }
    #[test]
    fn pdf_gen_struct_too_many_params() {
        let path = Path::new("/home/test");
        let template = PdfTemplates::Test;
        let mut params = IndexMap::new();
        params.insert("test".to_owned(), "value1".to_owned());
        params.insert("name".to_owned(), "value2".to_owned());
        params.insert("extra1".to_owned(), "value3".to_owned());
        let mut expected_params = IndexMap::new();
        expected_params.insert("test".to_owned(), "value1".to_owned());
        expected_params.insert("name".to_owned(), "value2".to_owned());
        let pdf_gen = PdfGen::new(path, &template, &params).unwrap();
        assert_eq!(pdf_gen.path(), path);
        assert_eq!(&pdf_gen.template().to_string(), "templates/test.typ");
        assert_eq!(pdf_gen.params(), &expected_params);
    }
    #[test]
    fn pdf_gen_struct_missing_param() {
        let path = Path::new("/home/test");
        let template = PdfTemplates::Test;
        let mut params = IndexMap::new();
        params.insert("test".to_owned(), "value1".to_owned());
        let pdf_gen_error = PdfGen::new(path, &template, &params).unwrap_err();
        assert_eq!(pdf_gen_error, PdfGenError::MissingParam("name".to_owned()));
    }
    #[test]
    fn gen_pdf_typst_compile_args() {
        let expected_args = vec![
            "compile".to_owned(),
            "-f".to_owned(),
            "pdf".to_owned(),
            "templates/test.typ".to_owned(),
            "/var/obvia/docs/test.pdf".to_owned(),
            "--input".to_owned(),
            "test=value1".to_owned(),
            "--input".to_owned(),
            "name=value2".to_owned(),
        ];
        let path = Path::new("/var/obvia/docs/test.pdf");
        let template = PdfTemplates::Test;
        let mut params = IndexMap::new();
        params.insert("name".to_owned(), "value2".to_owned());
        params.insert("test".to_owned(), "value1".to_owned());
        let pdf_gen = PdfGen::new(path, &template, &params).unwrap();
        assert_eq!(pdf_gen.typst_compile_args(), expected_args);
    }
    #[test]
    fn gen_pdf_typst_compile_args_too_many_params() {
        let expected_args = vec![
            "compile".to_owned(),
            "-f".to_owned(),
            "pdf".to_owned(),
            "templates/test.typ".to_owned(),
            "/var/obvia/docs/test.pdf".to_owned(),
            "--input".to_owned(),
            "test=value1".to_owned(),
            "--input".to_owned(),
            "name=value2".to_owned(),
        ];
        let path = Path::new("/var/obvia/docs/test.pdf");
        let template = PdfTemplates::Test;
        let mut params = IndexMap::new();
        params.insert("test".to_owned(), "value1".to_owned());
        params.insert("name".to_owned(), "value2".to_owned());
        params.insert("extra1".to_owned(), "value3".to_owned());
        let pdf_gen = PdfGen::new(path, &template, &params).unwrap();
        assert_eq!(pdf_gen.typst_compile_args(), expected_args);
    }
}
