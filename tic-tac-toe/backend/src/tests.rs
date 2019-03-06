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
use crate::main;
use crate::request_response::{Request, Response};

// TODO: add more tests

#[test]
fn x_tile_win() {
    let login_request = create_login_request("John".to_owned());
    assert_eq!(
        main(login_request), "{\"board\":[\"_\",\"_\",\"_\",\"_\",\"X\",\"_\",\"_\",\"_\",\"_\"],\"player_tile\":\"O\",\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_move_request("John".to_owned(), 2, 2);
    assert_eq!(
        main(login_request),
        "{\"coords\":[0,2],\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_move_request("John".to_owned(), 1, 2);
    assert_eq!(
        main(login_request),
        "{\"coords\":[2,1],\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_login_request("John".to_owned());
    assert_eq!(
        main(login_request), "{\"board\":[\"_\",\"_\",\"X\",\"_\",\"X\",\"O\",\"_\",\"X\",\"O\"],\"player_tile\":\"O\",\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_login_request("Peter".to_owned());
    assert_eq!(
        main(login_request), "{\"board\":[\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\",\"_\"],\"player_tile\":\"X\",\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_move_request("Peter".to_owned(), 2, 2);
    assert_eq!(
        main(login_request),
        "{\"coords\":[0,1],\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_move_request("Peter".to_owned(), 1, 2);
    assert_eq!(
        main(login_request),
        "{\"coords\":[0,2],\"winner\":\"None\"}".to_owned()
    );

    let login_request = create_move_request("Peter".to_owned(), 1, 1);
    assert_eq!(
        main(login_request),
        "{\"coords\":[2,0],\"winner\":\"None\"}".to_owned()
    );
}

fn create_move_request(player_name: String, x: usize, y: usize) -> String {
    let request = Request::PlayerMove {
        player_name,
        coords: (x, y),
    };

    serde_json::to_string(&request).unwrap()
}

fn create_login_request(player_name: String) -> String {
    let request = Request::Login { player_name };

    serde_json::to_string(&request).unwrap()
}

fn create_game_request(player_name: String) -> String {
    let request = Request::CreateGame { player_name };

    serde_json::to_string(&request).unwrap()
}

fn get_state_request(player_name: String) -> String {
    let request = Request::GetGameState { player_name };

    serde_json::to_string(&request).unwrap()
}

fn get_statistics_request(player_name: String) -> String {
    let request = Request::GetStatistics;

    serde_json::to_string(&request).unwrap()
}
