# Game of Dice
TODO: game description

## Game of Dice backend

### Setting up Rust

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

### Creating a dice-game backend part

The game logic of dice-game is already implemented in [`GameManager`](backend/src/game_manager.rs), your task is to implement functions routing and entry function for interaction with Fluence node in [`lib.rs`](backend/src/lib.rs) file. 

So, open it in your favorite text editor, you can see the following code

```Rust
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
```

This snippet imports all modules and crates and also define settings that manages the game. All dice game logic is implemented in `GameManager` struct. It maintains linked hash map with users and their balances. This hash map contains maximum `PLAYERS_MAX_COUNT` players and deletes the oldest one if limit is exceeded.

It has three export functions: 
- `join` - creates new player, returns it's player_id
- `bet` - makes a bet with player_id, placement, bet_amount, returns outcome and new player balance
- `get_player_balance` - returns player balance for a given player_id

At first, we need to define create `GameManager` instance. It should be global since the game state needs to be persisted between module calls. There are some possibilities to define global variable that does heap allocations in Rust, but in case of single-threaded Wasm environment the most suitable one is using thread local storage by `thread_local!`:

```Rust
thread_local! {
    static GAME_MANAGER: RefCell<GameManager> = RefCell::new(GameManager::new());
}
```

`RefCell` here is needed to provide interior mutability since `thread_local!` assume that its content is immutable.

Then let's write a functions that parses and performs requests. There are Request and Response in [`request_response.rs`](backend/src/request_response.rs) enums that could be serialized and deserialized by `serde`. They are supposed to be used for deserialize requests and make responses. With great power of `serde_json` this function can be implemented like this:

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

We have one step left on fully working example. All we need is to define invocation handler function that manages requests from a `client-side` to `do_request`, let's call it main.

To make main function called from the Fluence external code `#[invocation_handler]` macro can be used. Function marked with `#[invocation_handler]` is called a gateway function. It is an entrypoint to your application, all transactions sent by users will be passed to that function, and it's result will be available to users. Gateway function can receive and return either String or Vec<u8>.

It is possible here to return either String or Vec<u8> from main but it is more easy to return String:

```Rust
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

Finally, we have `main` function that receives json as string, process it by `do_request` and returns back also a json as string.

Also there is [`lib.rs.full`](backend/src/lib.rs.full) file that contains full working code.

### Compiling to WebAssembly

Run the following code to build a `.wasm` file from your Rust code in the project root directory (NOTE: Downloading and compiling dependencies might take a few minutes).

```bash
~/hello-world $ cargo +nightly build --target wasm32-unknown-unknown --release
    Updating crates.io index
    ...
    Finished release [optimized] target(s) in 1m 16s
```

If everything goes well, you should have a `.wasm` file deep in `target`. Let's check it:
```bash
~/hello-world $ ls -lh target/wasm32-unknown-unknown/release/dice_game.wasm
-rwxr-xr-x  2 user  user  1.4M Feb 11 11:59 target/wasm32-unknown-unknown/release/dice_game.wasm
```

## Publishing
TODO

## Game of Dice frontend
_For this part, you will need installed `npm`. Please refer to [npm docs](https://www.npmjs.com/get-npm) for installation instructions._

Having a [Rust backend for the Game of Dice (TODO: link)](../../../vm/examples/dice), the next logical step is to provide potential users with a web interface for the game. 

## Getting the code
There is already an existing web app for the Game of Dice, we can start from here. You can get it by cloning the repository:
```bash
~ $ git clone https://github.com/fluencelabs/dice-web-example
~ $ cd dice-web-example
~/dice-web-example $ 
```

There are just three files (except for README, LICENSE and .gitignore):
- `package.json` that declares needed dependencies
- `webpack.config.js` needed for the webpack to work
- `index.js` that imports `fluence` js library and shows how to connect to a cluster

## Reviewing the code
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