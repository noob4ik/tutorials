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

use std::error::Error;
use std::sync::Mutex;

use fluence::sdk::*;
use llamadb::tempdb::{ExecuteStatementResponse, TempDb};

use crate::signature::*;

mod signature;

/// Result for all possible Error types.
type GenResult<T> = ::std::result::Result<T, Box<Error>>;


#[invocation_handler]
fn main(input: String) -> String {
    let result = check_input(&input).and_then(forward);

    match result {
        Ok(response) => response,
        Err(err_msg) => format!("[Error] {}", err_msg),
    }
}

fn forward(input: String) -> GenResult<String> {
    // forward request to next module
    "".to_owned()
}