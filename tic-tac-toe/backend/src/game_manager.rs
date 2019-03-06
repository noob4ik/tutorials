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

use crate::error_type::AppResult;
use crate::game::{Game, GameMove, Tile};
use crate::player::Player;
use crate::request_response::Response;

use crate::settings::{GAMES_MAX_COUNT, PLAYERS_MAX_COUNT, USER_NAME_MAX_LEN};
use arraydeque::{ArrayDeque, Wrapping};
use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use serde_json::Value;
use std::{cell::RefCell, collections::HashMap, ops::AddAssign, rc::Rc, rc::Weak};

pub struct GameStatistics {
    // overall players count that has been registered
    pub players_created: u64,
    // overall players count that has been created
    pub games_created: u64,
    // overall move count that has been made
    pub moves_count: u64,
}

pub struct GameManager {
    players: ArrayDeque<[Rc<RefCell<Player>>; PLAYERS_MAX_COUNT], Wrapping>,
    games: ArrayDeque<[Rc<RefCell<Game>>; GAMES_MAX_COUNT], Wrapping>,
    // TODO: String key should be replaced with Cow<'a, str>. After that signatures of all public
    // functions also should be changed similar to https://jwilm.io/blog/from-str-to-cow/.
    players_by_name: HashMap<String, Weak<RefCell<Player>>>,
    game_statistics: RefCell<GameStatistics>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            games: ArrayDeque::new(),
            players: ArrayDeque::new(),
            players_by_name: HashMap::new(),
            game_statistics: RefCell::new(GameStatistics {
                players_created: 0,
                games_created: 0,
                moves_count: 0,
            }),
        }
    }

    /// Marks an empty position on the board by user's tile type. Returns MoveResponse structure
    /// as a serde_json Value.
    pub fn make_move(&self, player_name: String, coords: (usize, usize)) -> AppResult<Value> {
        let game = self.get_player_game(&player_name)?;
        let mut game = game.borrow_mut();
        let game_move = GameMove::new(coords.0, coords.1)
            .ok_or_else(|| format!("Invalid coordinates: x = {} y = {}", coords.0, coords.1))?;

        let response =
            match game.player_move(game_move, self.game_statistics.borrow().games_created)? {
                Some(app_move) => {
                    // checks did the app win in this turn?
                    let winner = match game.get_winner() {
                        Some(winner) => winner.to_string(),
                        None => "None".to_owned(),
                    };
                    Response::PlayerMove {
                        winner,
                        coords: (app_move.x, app_move.y),
                    }
                }
                // none means a win of the player or a draw
                None => Response::PlayerMove {
                    winner: game.get_winner().unwrap().to_string(),
                    coords: (std::usize::MAX, std::usize::MAX),
                },
            };

        self.game_statistics.borrow_mut().moves_count.add_assign(1);

        serde_json::to_value(response).map_err(Into::into)
    }

    /// Creates a new player with given player name.
    pub fn login(&mut self, player_name: String) -> AppResult<Value> {
        if player_name.len() > USER_NAME_MAX_LEN {
            return Err(format!(
                "The user name is too long ({} bytes), the limit is {}",
                player_name.len(),
                USER_NAME_MAX_LEN
            ))
            .map_err(Into::into);
        }

        if let None = self.players_by_name.get(&player_name) {
            let new_player = Rc::new(RefCell::new(Player::new(player_name.clone())));

            self.players_by_name
                .insert(new_player.borrow().name.clone(), Rc::downgrade(&new_player));

            if let Some(prev) = self.players.push_back(new_player) {
                // if some elements poped from the deque, delete a corresponding weak link from
                // names_to_players
                self.players_by_name.remove(&prev.borrow().name);
            }
        }

        let player = self.get_player(&player_name).unwrap();
        if let Some(game) = player.borrow().game.upgrade() {
            return self.serialize_game_state(&game);
        }

        self.game_statistics
            .borrow_mut()
            .players_created
            .add_assign(1);

        self.create_game(player_name)
    }

    pub fn serialize_game_state(&self, game: &Rc<RefCell<Game>>) -> AppResult<Value> {
        let (chosen_tile, board) = game.borrow().get_state();
        let response = Response::GameState {
            board,
            player_tile: chosen_tile.to_char(),
            winner: match game.borrow().get_winner() {
                Some(winner) => winner.to_string(),
                None => "None".to_owned(),
            },
        };
        return serde_json::to_value(response).map_err(Into::into);
    }

    /// Returns current game state for provided user as a GetGameStateResponse serde_json Value.
    pub fn get_game_state(&self, player_name: String) -> AppResult<Value> {
        let game = self.get_player_game(&player_name)?;
        self.serialize_game_state(&game)
    }

    /// Returns statistics of application usage.
    pub fn get_statistics(&self) -> AppResult<Value> {
        let response = Response::Statistics {
            players_created: self.game_statistics.borrow().players_created,
            games_created: self.game_statistics.borrow().games_created,
            moves_count: self.game_statistics.borrow().moves_count,
        };
        serde_json::to_value(response).map_err(Into::into)
    }

    /// Creates a new game for provided player. Note that the previous one is deleted (if it
    /// present) and won't be accessed anymore. Returns CreateGameResponse as a serde_json Value if
    /// 'X' tile type has been chosen and MoveResponse otherwise.
    pub fn create_game(&mut self, player_name: String) -> AppResult<Value> {
        let player = self.get_player(&player_name)?;

        let player_tile = self.generate_tile();
        let game = Rc::new(RefCell::new(Game::new(player_tile)));
        player.borrow_mut().game = Rc::downgrade(&game);

        if player_tile == Tile::O {
            game.borrow_mut()
                .app_move(self.game_statistics.borrow().games_created);
        }
        let response = self.serialize_game_state(&game);

        self.game_statistics
            .borrow_mut()
            .games_created
            .add_assign(1);

        self.games.push_back(game);
        response
    }

    fn generate_tile(&self) -> Tile {
        let mut rng = IsaacRng::seed_from_u64(self.game_statistics.borrow().games_created);
        if rng.gen::<bool>() {
            Tile::X
        } else {
            Tile::O
        }
    }

    fn get_player(&self, player_name: &str) -> AppResult<Rc<RefCell<Player>>> {
        // try to find player by name in players_by_name and then convert Weak<Player> to Rc<Player>
        match self.players_by_name.get(&player_name.to_owned()) {
            Some(player) => player.upgrade().ok_or_else(|| {
                "Internal error occurred - player has been already removed".to_owned()
            }),
            None => Err(format!("Player with name {} wasn't found", player_name)),
        }
        .map_err(Into::into)
    }

    fn get_player_game(&self, player_name: &str) -> AppResult<Rc<RefCell<Game>>> {
        self
            // returns Rc<RefCell<Player>> if success
            .get_player(player_name)?
            // borrows a mutable link to Player from RefCell
            .borrow_mut()
            // gets Weak<RefCell<Game>> from Player
            .game
            // tries to upgrade Weak<RefCell<Game>> to Rc<RefCell<Game>>
            .upgrade()
            .ok_or_else(|| "Sorry! Your game was deleted, but you can start a new one".to_owned())
            .map_err(Into::into)
    }
}
