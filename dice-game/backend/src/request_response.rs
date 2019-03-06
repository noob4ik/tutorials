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

use serde::{Deserialize, Serialize};

/*
  join: {} -> {id}
  bet: {id, bet_placement, bet_size} -> {outcome, player_balance}
  get_balance: {id} -> {player_balance}
*/

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Join,
    Roll {
        player_id: u64,
        bet_placement: u8,
        bet_size: u32,
    },
    GetBalance {
        player_id: u64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Join { player_id: u64 },
    Roll { outcome: u8, player_balance: u64 },
    GetBalance { player_balance: u64 },
    Error { message: String },
}
