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

mod error_type;
mod game_manager;
mod request_response;

use crate::error_type::AppResult;
use crate::game_manager::GameManager;
use crate::request_response::{Request, Response};

use fluence::sdk::*;
use serde_json::Value;
use std::cell::RefCell;

mod settings {
    pub const PLAYERS_MAX_COUNT: usize = 1024;
    pub const SEED: u64 = 12345678;
    // the account balance of new players
    pub const INIT_ACCOUNT_BALANCE: u64 = 100;
    // if win, player receives bet_amount * PAYOUT_RATE money
    pub const PAYOUT_RATE: u64 = 5;
}
