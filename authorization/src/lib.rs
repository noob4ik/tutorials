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
use log::info;

use crate::signature::*;

#[macro_use]
extern crate lazy_static;

mod signature;

/// Result for all possible Error types.
type GenResult<T> = ::std::result::Result<T, Box<Error>>;

const RESULT_SIZE_LEN: usize = 4;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

#[invocation_handler(init_fn = init)]
fn main(input: String) -> String {
    let result =
        if false {
            check_input(&input).and_then(forward)
        } else {
            forward(input.as_str())
        };


    match result {
        Ok(response) => response,
        Err(err_msg) => format!("[Error] {}", err_msg),
    }
}

fn forward(input: &str) -> GenResult<String> {
    unsafe {

        let bytes = input.as_bytes();
        let len = bytes.len();

        let ptr = allocateBackend(len);

        for i in 0..len {

            storeBackend(ptr + i, bytes[i]);
        }

        let result_ptr = invokeBackend(ptr, len);

        let mut len_bytes: [u8; RESULT_SIZE_LEN] = [0; RESULT_SIZE_LEN];

        for i in 0..RESULT_SIZE_LEN {
            len_bytes[i] = loadBackend(result_ptr + i);
        }

        let result_len: usize = std::mem::transmute(len_bytes);

        let mut result_bytes = vec![0; result_len];

        for i in RESULT_SIZE_LEN..(result_len + RESULT_SIZE_LEN) {
            result_bytes[i - RESULT_SIZE_LEN] = loadBackend(result_ptr + i);
        }

        let result_str = std::str::from_utf8(result_bytes.as_ref())?;

        deallocateBackend(result_ptr, result_len + RESULT_SIZE_LEN);

        return Ok(result_str.to_owned())
    }
}

#[link(wasm_import_module = "dice_game")]
extern {
    fn allocateBackend(size: usize) -> usize;
    fn deallocateBackend(ptr: usize, size: usize);
    fn invokeBackend(ptr: usize, size: usize) -> usize;
    fn loadBackend(ptr: usize) -> u8;
    fn storeBackend(ptr: usize, byte: u8);
}