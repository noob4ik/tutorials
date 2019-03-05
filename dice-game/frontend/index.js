import "bootstrap/dist/css/bootstrap.min.css";
import * as fluence from "fluence";

let globalInfo = {losses_number: 0};
let history = [];

window.info = globalInfo;

// save fluence to global variable, so it can be accessed from Developer Console
window.fluence = fluence;

// convert result to a string
window.getResultString = function (result) {
	return result.result().then((r) => r.asString())
};

window.logResultAsString = function(result) {
	return getResultString(result).then((r) => console.log(r))
};

window.onload = function () {
	// locate html elements
	const statusDiv = document.getElementById('status');

	const gameDiv = document.getElementById('game');
	const resultDiv = document.getElementById('result');
	const balanceDiv = document.getElementById('balance');
	const betInput = document.getElementById('bet');
	const guessInput = document.getElementById('guess');
	const rollButton = document.getElementById('roll');
	const historyTable = document.getElementById('history');
	
	// address to Fluence contract in Ethereum blockchain. Interaction with blockchain created by MetaMask or with local Ethereum node
	let contractAddress = "0x074a79f29c613f4f7035cec582d0f7e4d3cda2e7";

	// set ethUrl to `undefined` to use MetaMask instead of Ethereum node
	let ethUrl = "http://data.fluence.one:8545/";

	// application to interact with that stored in Fluence contract
	let appId = "10";

	// create a session between client and backend application, and then join the game
	fluence.connect(contractAddress, appId, ethUrl).then((s) => {
		console.log("Session created");
		window.session = s;
	}).then(() => join());

	// send request to join the game
	function join() {
		let result = session.invoke(`{ "action": "Join" }`);
		getResultString(result).then(function (str) {
			let response = JSON.parse(str);
			if (response.player_id || response.player_id === 0) {
				statusDiv.innerText = "You joined to game. Your id is: " + response.player_id;
				// 100 is hardcoded, because we always register a new player
				updateBalance(100);
				startGame(response.player_id);
			} else {
				showError("Unable to register: " + str);
			}
		});
	}

	// hide registration, show game controls and balance
	function startGame(id) {
		globalInfo.player_id = id;
		gameDiv.hidden = false;
		betInput.focus();
	}

	// call roll() on button click
	rollButton.addEventListener("click", roll);

	// roll the dice by sending a request to backend, show the outcome and balance
	function roll() {
		if (checkInput()) {
			resultDiv.innerHTML = "";
			let request = betRequest();
			let result = session.invoke(request);
			getResultString(result).then(str => {
				let response = JSON.parse(str);
				if (response.outcome) {
					showResult(parseInt(response.outcome), guess);
					saveGame(bet, response);
				} else {
					showError("Unable to roll: " + str);
				}
			});
		}
	}

	// build a bet JSON request from inputs
	function betRequest() {
		let bet = parseInt(betInput.value.trim());
		let guess = parseInt(guessInput.value.trim());

		let request = {
			player_id: globalInfo.player_id,
			action: "Bet",
			placement: guess,
			bet_amount: parseInt(bet)
		};
		
		return JSON.stringify(request);
	}

	// check inputs are valid, and report if they're not
	function checkInput() {
		if (!(betInput.checkValidity() && guessInput.checkValidity())) {
			betInput.reportValidity();
			guessInput.reportValidity();
			return false;
		}

		return true;
	}

	// display results in UI
	function showResult(fact, guess) {
		let resultStr = `<h2>Outcome is ${fact}.   `;
		if (fact !== guess) {
			globalInfo.losses_number = globalInfo.losses_number + 1;
			let time = globalInfo.losses_number === 1 ? "time" : "times in a row";
			resultDiv.innerHTML =  `${resultStr}<b style='color:red'>You've lost ${globalInfo.losses_number} ${time}!</b></h2>`
		} else {
			globalInfo.losses_number = 0;
			resultDiv.innerHTML = resultStr + "<b style='color:green'>You won!</b></h2>"
		}
	}

	// prepend game results to the game history table
	function saveGame(bet, response) {
		updateBalance(response.player_balance);
		history.unshift(`<tr><td>${bet}</td><td>${response.outcome}</td><td>${response.player_balance}</td></tr>`);
		historyTable.innerHTML = history.join("");
	}

	// update balance in UI
	function updateBalance(balance) {
		globalInfo.balance = balance;
		balanceDiv.innerHTML = globalInfo.balance;
	}

	// show error to the user
	function showError(error) {
		console.error(error)
		// TODO: show error visually
	}

	// hackity-hack! we could get balance for any player
	function getBalance(id) {
		let result = session.invoke(`{ "player_id": ${id}, "action": "GetBalance"}`);
		return getResultString(result).then(function (str) {
			let response = JSON.parse(str);
			if (response.player_balance) {
				updateBalance(response.player_balance)
			} else {
				showError("Unable to get balance: " + str);
			}
		});
	}
};
