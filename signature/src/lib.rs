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

use fluence::sdk::*;

use crate::signature::*;
use std::ptr::NonNull;

#[macro_use]
extern crate lazy_static;

mod signature;

/// Result for all possible Error types.
type GenResult<T> = ::std::result::Result<T, Box<Error>>;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(input: String) -> String {
    let result = check_input(&input);

    match result {
        Ok(response) => response.to_owned(),
        Err(err_msg) => format!("[Error] {}", err_msg),
    }
}

#[no_mangle]
pub unsafe fn store(mut ptr: NonNull<u8>, byte: u8) {
    *ptr.as_mut() = byte;
}

#[no_mangle]
pub unsafe fn load(ptr: NonNull<u8>) -> u8 {
    return *ptr.as_ref();
}
