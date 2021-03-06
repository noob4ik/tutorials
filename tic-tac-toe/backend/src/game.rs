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

use crate::settings::SEED;
use boolinator::Boolinator;
use rand::{Rng, SeedableRng};
use rand_isaac::IsaacRng;
use std::convert::From;
use std::{fmt, result::Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    X,
    O,
}

impl Tile {
    pub fn to_char(self) -> char {
        match self {
            Tile::X => 'X',
            Tile::O => 'O',
        }
    }

    // returns tile type of opposite player
    pub fn other(self) -> Self {
        match self {
            Tile::X => Tile::O,
            Tile::O => Tile::X,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Winner {
    X,
    O,
    Draw,
}

impl fmt::Display for Winner {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let winner_as_str = match self {
            Winner::X => "X",
            Winner::O => "O",
            Winner::Draw => "Draw",
        };
        fmt.write_str(winner_as_str)?;
        Ok(())
    }
}

impl From<Tile> for Winner {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::X => Winner::X,
            Tile::O => Winner::O,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GameMove {
    pub x: usize,
    pub y: usize,
}

impl GameMove {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        fn is_valid(x: usize, y: usize) -> bool {
            x <= 2 && y <= 2
        }

        is_valid(x, y).as_some(GameMove { x, y })
    }
}

pub struct Game {
    board: [[Option<Tile>; 3]; 3],
    player_tile: Tile,
    rng: IsaacRng,
}

impl Game {
    pub fn new(player_tile: Tile) -> Self {
        Game {
            board: [[None; 3]; 3],
            player_tile,
            rng: SeedableRng::seed_from_u64(SEED),
        }
    }

    /// Returns Some(Winner) if there is some and None otherwise.
    pub fn get_winner(&self) -> Option<Winner> {
        fn same_row(game: &Game) -> Option<Winner> {
            for col in 0..3 {
                if game.board[0][col].is_some()
                    && (game.board[0][col] == game.board[1][col])
                    && (game.board[1][col] == game.board[2][col])
                {
                    return Some(game.board[0][col].unwrap().into());
                }
            }
            None
        }

        fn same_col(game: &Game) -> Option<Winner> {
            for row in 0..3 {
                if game.board[row][0].is_some()
                    && (game.board[row][0] == game.board[row][1])
                    && (game.board[row][1] == game.board[row][2])
                {
                    return Some(game.board[row][0].unwrap().into());
                }
            }
            None
        }

        // checks the left-right diagonal
        fn same_main_diag(game: &Game) -> Option<Winner> {
            if game.board[0][0].is_some()
                && (game.board[0][0] == game.board[1][1])
                && (game.board[1][1] == game.board[2][2])
            {
                return Some(game.board[0][0].unwrap().into());
            }
            None
        }

        // checks the right-left diagonal
        fn same_anti_diag(game: &Game) -> Option<Winner> {
            if game.board[0][2].is_some()
                && (game.board[0][2] == game.board[1][1])
                && (game.board[1][1] == game.board[2][0])
            {
                return Some(game.board[0][2].unwrap().into())
            }
            None
        }

        // checks that all tiles are empty (a draw condition)
        fn no_empty(game: &Game) -> Option<Winner> {
            game.board
                .iter()
                .all(|row| row.iter().all(|cell| cell.is_some()))
                .as_some(Winner::Draw)
        }

        same_row(self)
            .or_else(|| same_col(self))
            .or_else(|| same_main_diag(self))
            .or_else(|| same_anti_diag(self))
            .or_else(|| no_empty(self))
    }

    /// Makes player and application moves successively. Returns Some() of with coords of app move
    /// if it was successfull and None otherwise. None result means a draw or win of the player.
    pub fn player_move(
        &mut self,
        game_move: GameMove,
        entropy: u64,
    ) -> Result<Option<GameMove>, String> {
        if let Some(player) = self.get_winner() {
            return Err(format!("Player {} has already won this game", player));
        }

        self.board[game_move.x][game_move.y]
            .is_none()
            .ok_or_else(|| "Please choose a free position".to_owned())?;

        self.board[game_move.x][game_move.y].replace(self.player_tile);

        Ok(self.app_move((game_move.x + game_move.y) as u64 + entropy))
    }

    /// Returns current game state as a tuple with players tile and board.
    pub fn get_state(&self) -> (Tile, Vec<char>) {
        let mut board: Vec<char> = Vec::new();

        for tile in self.board.iter().flat_map(|r| r.iter()) {
            match tile {
                Some(tile) => board.push(tile.to_char()),
                None => board.push('_'),
            }
        }

        (self.player_tile, board)
    }

    /// Makes application move. Returns Some() of with coords of app move if it was successfull and
    /// None otherwise. None result means a draw or win of the app.
    pub fn app_move(&mut self, entropy: u64) -> Option<GameMove> {
        if self.get_winner().is_some() {
            return None;
        }

        // TODO: use more complicated strategy
        let mut empty_tiles = Vec::<(usize, usize)>::new();
        for (x, row) in self.board.iter_mut().enumerate() {
            for (y, tile) in row.iter_mut().enumerate() {
                if tile.is_some() {
                    continue;
                }
                empty_tiles.push((x, y));
            }
        }

        if empty_tiles.is_empty() {
            return None;
        }

        self.rng = IsaacRng::seed_from_u64(empty_tiles.len() as u64 + entropy);

        let app_move = self.rng.gen_range(0, empty_tiles.len());
        let app_move = empty_tiles[app_move];
        self.board[app_move.0][app_move.1] = Some(self.player_tile.other());

        Some(GameMove::new(app_move.0, app_move.1).unwrap())
    }
}
