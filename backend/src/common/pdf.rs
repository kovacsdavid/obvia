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

use serde::Serialize;
use tempfile::NamedTempFile;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum PdfGenError {
    #[error("Payload could not be converted to JSON: {0}")]
    PayloadJson(String),

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
    WarehouseView,
    TaxView,
    ProductView,
    InventoryView,
    InventoryMovementView,
}

impl Display for PdfTemplates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let template = match &self {
            Self::Test => "test",
            Self::CustomerView => "customer_view",
            Self::WarehouseView => "warehouse_view",
            Self::TaxView => "tax_view",
            Self::ProductView => "product_view",
            Self::InventoryView => "inventory_view",
            Self::InventoryMovementView => "inventory_movement_view",
        };
        write!(f, "templates/{template}.typ")
    }
}

#[derive(Debug)]
struct PdfGen<'a, T>
where
    T: Serialize,
{
    path: &'a Path,
    template: &'a PdfTemplates,
    payload: &'a T,
}

impl<'a, T> PdfGen<'a, T>
where
    T: Serialize,
{
    pub fn new(path: &'a Path, template: &'a PdfTemplates, payload: &'a T) -> PdfGenResult<Self> {
        Ok(PdfGen {
            path,
            template,
            payload,
        })
    }
    pub fn path(&self) -> &Path {
        self.path
    }
    pub fn template(&self) -> &PdfTemplates {
        self.template
    }
    pub fn payload(&self) -> &T {
        self.payload
    }
    pub fn typst_compile_args(&self) -> PdfGenResult<Vec<String>> {
        let args = vec![
            "compile".to_owned(),
            "-f".to_owned(),
            "pdf".to_owned(),
            self.template().to_string(),
            self.path().to_string_lossy().into_owned(),
            "--input".to_owned(),
            format!(
                "payload={}",
                serde_json::to_string(self.payload())
                    .map_err(|e| PdfGenError::PayloadJson(e.to_string()))?
            ),
        ];
        Ok(args)
    }
}

pub fn gen_pdf_temporary<T>(template: &PdfTemplates, payload: &T) -> PdfGenResult<Vec<u8>>
where
    T: Serialize,
{
    let tmp_file = NamedTempFile::new().map_err(|e| PdfGenError::IOError(e.to_string()))?;
    let pdf_gen = PdfGen::new(tmp_file.path(), template, payload)?;
    let mut output = Command::new("typst");

    for arg in pdf_gen.typst_compile_args()? {
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

pub fn gen_pdf_persistent<'a, T>(
    path: &'a Path,
    template: &PdfTemplates,
    params: &T,
) -> PdfGenResult<&'a Path>
where
    T: Serialize,
{
    let pdf_gen = PdfGen::new(path, template, params)?;

    let mut output = Command::new("typst");

    for arg in pdf_gen.typst_compile_args()? {
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

#[cfg(test)]
mod tests {}
