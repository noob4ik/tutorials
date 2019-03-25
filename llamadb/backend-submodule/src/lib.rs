/*
 * Copyright 2018 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Wrapper for Llamadb (a test for Fluence network).
//!
//! Provides the FFI (`main`) for interact with Llamadb.

use fluence::sdk::*;
use log::info;

#[cfg(test)]
mod tests;

#[macro_use]
extern crate lazy_static;

use llamadb::tempdb::{ExecuteStatementResponse, TempDb};
use std::error::Error;
use std::sync::Mutex;
use std::ptr::NonNull;

/// Result for all possible Error types.
type GenResult<T> = ::std::result::Result<T, Box<Error>>;

lazy_static! {
    static ref DATABASE: Mutex<TempDb> = Mutex::new(TempDb::new());
}

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

/// Executes SQL and converts llamadb error to string.
#[invocation_handler(init_fn = init)]
fn main(sql_str: String) -> String {
    info!("request: {}", sql_str);
    match run_query(&sql_str) {
        Ok(response) => {
            info!("response {}", response);
            response
        },

        Err(err_msg) => {
            let err = format!("[Error] {}", err_msg);
            info!("err {}", err);
            err
        },
    }
}

/// Acquires lock, does query, releases lock, returns query result.
fn run_query(sql_query: &str) -> GenResult<String> {
    let mut db = DATABASE.lock()?;
    db.do_query(sql_query)
        .map(statement_to_string)
        .map_err(Into::into)
}

/// Converts query result to CSV String.
fn statement_to_string(statement: ExecuteStatementResponse) -> String {
    match statement {
        ExecuteStatementResponse::Created => "table created".to_string(),
        ExecuteStatementResponse::Dropped => "table was dropped".to_string(),
        ExecuteStatementResponse::Inserted(number) => format!("{}", number),
        ExecuteStatementResponse::Select { column_names, rows } => {
            let col_names = column_names.to_vec().join(", ") + "\n";
            let rows_as_str = rows
                .map(|row| {
                    row.iter()
                        .map(|elem| elem.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                })
                .collect::<Vec<String>>()
                .join("\n");

            col_names + &rows_as_str
        }
        ExecuteStatementResponse::Deleted(number) => format!("rows deleted: {}", number),
        ExecuteStatementResponse::Explain(result) => result,
        ExecuteStatementResponse::Updated(number) => format!("rows updated: {}", number),
    }
}

#[no_mangle]
pub unsafe fn store_db(mut ptr: NonNull<u8>, byte: u8) {
    *ptr.as_mut() = byte;
}

#[no_mangle]
pub unsafe fn load_db(ptr: NonNull<u8>) -> u8 {
    return *ptr.as_ref();
}