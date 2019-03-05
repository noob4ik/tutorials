# Game of Dice
- [Set up Rust](#set-up-rust)
- [Use existing code](#use-existing-code)
- [Implement requests handling](#implement-requests-handling)
- [Compiling Rust to WebAssembly](#compiling-rust-to-webassembly)
- [Publishing](#publishing)
- [Game of Dice frontend](#game-of-dice-frontend)
- [Reviewing the frontend code](#reviewing-the-frontend-code)
- [Running the app](#running-the-app)
- [Hacking!](#hacking)

In this simple dice game, you can bet your points against dice rolled by the backend. 

Backend handles user registration, balances and dice generation, and frontend gives the end-user an interface to play the game.

Backend will be developed in Rust, because of it's tremendous WebAssembly support. And for the frontend, JavaScript and some HTML will do. (With some TypeScript under the hood 😉)

## Set up Rust

Install rust compiler and it's tools:
```bash
~ $ curl https://sh.rustup.rs -sSf | sh -s -- -y
info: downloading installer
...
Rust is installed now. Great!
To configure your current shell run source $HOME/.cargo/env
```

Let's listen to the installer and configure your current shell:
```bash
~ $ source $HOME/.cargo/env
<no output>
```

Then you need to install nightly toolchain, run:

```bash
~ $ rustup toolchain install nightly
info: syncing channel updates ...
...
  nightly-<arch> installed - rustc 1.34.0-nightly (57d7cfc3c 2019-02-11)
```

To check nightly toolchain was installed succesfully:
```bash
~ $ rustup toolchain list | grep nightly
# output should contain nighly toolchain
...
nightly-<arch>
```

Also, to be able to compile Rust to WebAssembly, we need to add wasm32 compilation target. Just run the following:
```bash
# install target for WebAssembly
~ $ rustup target add wasm32-unknown-unknown --toolchain nightly
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
```

Now it's time to create a Rust dice-game project!

## Use existing code

As most of the game is already implemented in the [`GameManager`](backend/src/game_manager.rs), your task will be to handle users' interaction: route their requests, handle errors, and bring the game to life! All this should be done inside [`lib.rs`](backend/src/lib.rs) file. 

Open `lib.rs` in your favorite text editor, you can see the following code

```Rust
// Describe modules used in the backend
mod error_type;
mod game_manager;
mod request_response;

// Import needed libraries
use crate::error_type::AppResult;
use crate::game_manager::GameManager;
use crate::request_response::{Request, Response};

use fluence::sdk::*;
use serde_json::Value;
use std::cell::RefCell;

// Define game settings
mod settings {
    pub const PLAYERS_MAX_COUNT: usize = 1024;
    pub const SEED: u64 = 12345678;
    // the account balance of new players
    pub const INIT_ACCOUNT_BALANCE: u64 = 100;
    // if win, player receives bet_amount * PAYOUT_RATE money
    pub const PAYOUT_RATE: u64 = 5;
}
```

This snippet imports needed modules and crates (libraries), and also defines the `settings` module with different game constants.

All game logic is implemented inside [`GameManager`](backend/src/game_manager.rs) struct. It maintains a linked hash map with users and their balances. This hash map contains up to `PLAYERS_MAX_COUNT` players, and deletes the oldest one if limit is exceeded.

`GameManager` has three public functions: 
- `join` - creates new player, returns it's `player_id`.
- `bet` - makes a bet with `player_id`, `guess`, `bet_amount`, returning an outcome and a player's balance.
- `get_player_balance` - returns the balances for the player specified by `player_id`.

We need to create a `GameManager` instance to store a game state. As the game state should be persisted between calls, `GameManager` should be a global variable. Since Wasm environment is single-threaded, `thread_local!` macro is used here for the global state storage.

```Rust
thread_local! {
    static GAME_MANAGER: RefCell<GameManager> = RefCell::new(GameManager::new());
}
```

`RefCell` here is needed to provide interior mutability since `thread_local!` assume that its content is immutable. It's a technical detail.

## Implement requests handling

_You can find full working example in the [`lib.rs.full`](backend/src/lib.rs.full) file._

There are `Request` and `Response` enums in the [`request_response.rs`](backend/src/request_response.rs) that could be serialized and deserialized by JSON framework named `serde`. 

These enums are to be used to parse requests and send back reponses. With the great power of `serde_json` routing can be easily implement via pattern matching:

```Rust
fn do_request(req: String) -> AppResult<Value> {
    let request: Request = serde_json::from_str(req.as_str())?;

    match request {
        Request::Join => GAME_MANAGER.with(|gm| gm.borrow_mut().join()),

        Request::Bet {
            player_id,
            placement,
            bet_amount,
        } => GAME_MANAGER.with(|gm| gm.borrow_mut().bet(player_id, placement, bet_amount)),

        Request::GetBalance { player_id } => {
            GAME_MANAGER.with(|gm| gm.borrow_mut().get_player_balance(player_id))
        }
    }
}
```

So, we have requests parsing and routing implemented! Great, now we need to tell Fluence how to call our code. For that, we need to mark some function with `#[invokation_handler]` macro. Let's call it `main`:
```rust
#[invocation_handler]
fn main(req: String) -> String
```

Such a function is called a _gateway function_. It is an entrypoint to your backend, and can take and return a `String` or a `Vec<u8>`, depending on your application needs. `String` seems like a better for for a JSON-based protocol, so we'll go with that. 

`do_request` returns a `Result`, possibly with errors, let's convert it to a `String`:

```rust
#[invocation_handler]
fn main(req: String) -> String {
    match do_request(req) {
        Ok(req) => req.to_string(),
        Err(err) => {
            let response = Response::Error {
                message: err.to_string(),
            };
            serde_json::to_string(&response).unwrap()
        }
    }
}
```

Finally, we have a `main` function that can receive JSON as a `String`, process it by `do_request`, and return a JSON result.

## Compiling Rust to WebAssembly

Run the following code to build a `.wasm` file from your Rust code in the project root directory.

NOTE: downloading and compiling dependencies might take a few minutes.

```bash
$ cd dice-game/backend
backend $ cargo +nightly build --target wasm32-unknown-unknown --release
    Updating crates.io index
    ...
    Finished release [optimized] target(s) in 1m 16s
```

If everything goes well, you should have a `.wasm` file deep in `target`. Let's check it:
```bash
backend $ ls -lh target/wasm32-unknown-unknown/release/dice_game.wasm
-rwxr-xr-x  2 user  user  1.4M Mar 5 00:00 target/wasm32-unknown-unknown/release/dice_game.wasm
```

## Publishing
Let's refer to the [Fluence Book](https://fluence.network/docs/book/quickstart/publish.html) to guide us through the publishing process.

## Game of Dice frontend
_For this part, you will need installed `npm`. Please refer to [npm docs](https://www.npmjs.com/get-npm) for installation instructions._

Having a [Rust backend](#implement-requests-handling) for the Game of Dice, the next logical step is to provide potential users with a web interface for the game. 

## Reviewing the frontend code
There are just three files (except for README, LICENSE and .gitignore) in `dice-game/frontend`:
- `package.json` that declares needed dependencies
- `webpack.config.js` needed for the webpack to work
- `index.js` that imports `fluence` js library and shows how to connect to a cluster

Let's take a look at `index.js`.

First, we locate needed html elements to use them later:
```javascript
// locate html elements
const registerDiv = document.querySelector('#register');
const usernameInput = document.querySelector('#username');
const joinButton = document.querySelector('#join');

const gameDiv = document.querySelector('#game');
const usernameTitle = document.querySelector('#username-title');
const balanceDiv = document.querySelector('#balance');
const betInput = document.querySelector('#bet');
const rollButton = document.querySelector('#roll');
const historyTable = document.querySelector('#history');
```

Next, connect to the Fluence real-time cluster hosting the app:
```javascript

// address of the Fluence smart contract on Ethereum.
let contractAddress = "0x074a79f29c613f4f7035cec582d0f7e4d3cda2e7";

// Address of the Ethereum node. If set to `undefined`, MetaMask will be used to send transactions.
let ethUrl = "http://207.154.240.52:8545/";

// appId of the backend as seen in Fluence smart contract.
let appId = "6";
...
// create a session between client and backend application
fluence.connect(contractAddress, appId, ethUrl).then((s) => {
  console.log("Session created");
  window.session = s;
  helloBtn.disabled = false;
});
```

Set a callback on `joinButton` to send a registration request to the backend. Upon receiving a success response from the backend, hide registration controls, and show game controls. Also, save `info` to the global `window.info` variable.
```javascript
// call join() on button click
joinButton.addEventListener("click", join)

// join the game by registering a user
function join() {
    const username = usernameInput.value.trim();
    let result = session.invoke(`{ "username": "${username}", "action": "register" }`);
    getResultString(result).then(function (str) {
        let response = JSON.parse(str);
        if (response.success = "true") {
            startGame(response.info);
        } else {
            showError("Unable to register: " + str);
        }
    });
}
```

Set a callback on roll button to make a bet, and roll the dice by sending a request to the backend:
```javascript

// call roll() on button click
rollButton.addEventListener("click", roll);

// roll the dice by sending a request to backend, show the outcome and balance
function roll() {
    let bet = betInput.value.trim();
    let result = session.invoke(`{ "id": "${window.info.id}", "action": "bet", "value": "${bet}" }`);
    getResultString(result).then(function (str) {
        let response = JSON.parse(str);
        if (response.success = "true") {
            saveGame(response.info);
        } else {
            showError("Unable to roll: " + str);
        }
    });
}
```

And a few utility functions to maintain games history, update balance, handling errors and working with Fluence responses:
```javascript
// prepend game results to the game history table
function saveGame(info) {
    updateBalance(info.balance);
    historyTable.prepend(`<tr><td>${info.bet}</td><td>${info.dice}</td<td>${info.balance}</td>></tr>`);
}

// update balance html
function updateBalance(balance) {
    window.info.balance = balance;
    balanceDiv.innerHTML = info.balance;
}

// show error to the user
function showError(error) {
    console.error(error)
    // TODO: show error visually
}

// convert result to a string
window.getResultString = function (result) {
    return result.result().then((r) => r.asString())
};

window.logResultAsString = function(result) {
    return getResultString(result).then((r) => console.log(r))
}
```

## Running the app
After putting it all together, let's run it:

```bash
$ cd dice-game/frontend
frontend $ npm install
frontend $ npm run start
> frontend-template@1.0.0 start /private/tmp/frontend-template
> webpack-dev-server

ℹ ｢wds｣: Project is running at http://localhost:8080/
...
```

Open [http://localhost:8080/](http://localhost:8080/), and you will see a simple registration form:

TODO: REGISTRATION SCREENSHOT

## Hacking!
Ideas to implement:
- Add names support on backend
- Add game history support on backend
- Make web interface fancier