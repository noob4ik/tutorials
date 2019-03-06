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

#[cfg(test)]
mod tests;

mod error_type;
mod game;
mod game_manager;
mod player;
mod request_response;

use crate::error_type::AppResult;
use crate::game_manager::GameManager;
use crate::request_response::{Request, Response};

use fluence::sdk::*;
use serde_json::Value;
use std::cell::RefCell;

mod settings {
    pub const GAMES_MAX_COUNT: usize = 1024;
    pub const PLAYERS_MAX_COUNT: usize = 1024;
    pub const SEED: u64 = 12345678;
    // to prevent DoS attack with large strings
    pub const USER_NAME_MAX_LEN: usize = 1024;
}

thread_local! {
    static GAME_MANAGER: RefCell<GameManager> = RefCell::new(GameManager::new());
}

fn do_request(req: String) -> AppResult<Value> {
    let request: Request = serde_json::from_str(req.as_str())?;

    match request {
        Request::PlayerMove {
            player_name,
            coords,
        } => GAME_MANAGER.with(|gm| gm.borrow().make_move(player_name, coords)),

        Request::Login { player_name } => {
            GAME_MANAGER.with(|gm| gm.borrow_mut().login(player_name))
        }

        Request::CreateGame { player_name } => {
            GAME_MANAGER.with(|gm| gm.borrow_mut().create_game(player_name))
        }

        Request::GetGameState { player_name } => {
            GAME_MANAGER.with(|gm| gm.borrow().get_game_state(player_name))
        }

        Request::GetStatistics => GAME_MANAGER.with(|gm| gm.borrow().get_statistics()),
    }
}

#[invocation_handler]
fn main(req: String) -> String {
    match do_request(req) {
        Ok(res) => res.to_string(),
        Err(err) => {
            let response = Response::Error {
                error: err.to_string(),
            };
            serde_json::to_string(&response).unwrap()
        }
    }
}
