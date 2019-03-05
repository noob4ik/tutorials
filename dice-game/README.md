# Game of Dice
TODO: game description

## Game of Dice backend
### Rust setup
TODO
### lib.rs
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
