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

#[cfg(test)]
use mockall::automock;
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

#[derive(Debug, PartialEq)]
pub enum PdfTemplates {
    Test,
    CustomerView,
    WarehouseView,
    TaxView,
    ProductView,
    InventoryView,
    InventoryMovementView,
    InventoryReservationView,
    ServiceView,
    TaskView,
    WorksheetView,
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
            Self::InventoryReservationView => "inventory_reservation_view",
            Self::ServiceView => "service_view",
            Self::TaskView => "task_view",
            Self::WorksheetView => "worksheet_view",
        };
        write!(f, "templates/{template}.typ")
    }
}

#[derive(Debug)]
pub struct PdfGenerator {}

#[cfg_attr(test, automock)]
impl PdfGenerator {
    fn typst_compile_args<T: Serialize + 'static>(
        template: &PdfTemplates,
        path: &Path,
        payload: T,
    ) -> PdfGenResult<Vec<String>> {
        let args = vec![
            "compile".to_owned(),
            "-f".to_owned(),
            "pdf".to_owned(),
            template.to_string(),
            path.to_string_lossy().into_owned(),
            "--input".to_owned(),
            format!(
                "payload={}",
                serde_json::to_string(&payload)
                    .map_err(|e| PdfGenError::PayloadJson(e.to_string()))?
            ),
        ];
        Ok(args)
    }
    pub fn gen_pdf_temporary<T>(template: &PdfTemplates, payload: T) -> PdfGenResult<Vec<u8>>
    where
        T: Serialize + 'static,
    {
        let tmp_file = NamedTempFile::new().map_err(|e| PdfGenError::IOError(e.to_string()))?;
        let mut output = Command::new("typst");

        for arg in PdfGenerator::typst_compile_args(template, tmp_file.path(), payload)? {
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

        fs::read(tmp_file.path()).map_err(|e| PdfGenError::IOError(e.to_string()))
    }

    pub fn gen_pdf_persistent<T>(
        path: &'static Path,
        template: &PdfTemplates,
        payload: T,
    ) -> PdfGenResult<&'static Path>
    where
        T: Serialize + 'static,
    {
        let mut output = Command::new("typst");

        for arg in PdfGenerator::typst_compile_args(template, path, payload)? {
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
}

#[cfg(test)]
pub mod tests {
    use lopdf::Document;
    use std::sync::Mutex;

    pub static PDF_GENERATOR_TEST_SYNC: Mutex<()> = Mutex::new(());

    pub fn extract_pdf_text(bytes: &[u8]) -> Result<String, lopdf::Error> {
        let document = Document::load_mem(bytes)?;
        let pages = document.get_pages();
        let page_numbers: Vec<u32> = pages.keys().copied().collect();
        document.extract_text(&page_numbers)
    }
}
