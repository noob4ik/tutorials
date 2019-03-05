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
 CreatePlayer: {player_name} -> {result}
 PlayerMove: {player_name, coords} -> { winner, coords}
 CreateGame: {player_name} -> {result}
 GetGameState: {player_name} -> {player_tile, board}
 GetStatistics: {} -> {players_created, games_created, moves_count}
*/

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    PlayerMove {
        player_name: String,
        coords: (usize, usize),
    },
    Login {
        player_name: String,
    },
    CreateGame {
        player_name: String,
    },
    GetGameState {
        player_name: String,
    },
    GetStatistics,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    PlayerMove {
        winner: String,
        coords: (usize, usize),
    },
    GetGameState {
        board: Vec<char>,
        player_tile: char,
        winner: String,
    },
    GetStatistics {
        players_created: u64,
        games_created: u64,
        moves_count: u64,
    },
    Error {
        error: String,
    },
}
