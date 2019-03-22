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

mod imported;
use fluence::sdk::*;
extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;


#[invocation_handler]
pub fn main() -> Vec<u8> {
    unsafe {
        let i: i32 = imported::invokeProxy();
        let mut bs = [0u8; mem::size_of::<i32>()];
        bs.as_mut()
            .write_i32::<LittleEndian>(i)
            .expect("Unable to write");
        bs.to_vec()
    }
}
