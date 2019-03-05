import * as fluence from "fluence";

let globalInfo = {losses_number: 0};
let history = [];

window.info = globalInfo;

// save fluence to global variable, so it can be accessed from Developer Console
window.fluence = fluence;

// convert result to a string
window.getResultString = function (result) {
	return result.result().then((r) => JSON.parse(r.asString()))
};

window.logResultAsString = function(result) {
	return getResultString(result).then((r) => console.log(r))
};

window.onload = function () {

	// address to Fluence contract in Ethereum blockchain. Interaction with blockchain created by MetaMask or with local Ethereum node
	let contractAddress = "0x4c9c53c3005a52d3468bb8a20fcb4c3ea76098ac";

	// set ethUrl to `undefined` to use MetaMask instead of Ethereum node
	let ethUrl = "http://data.fluence.one:8545/";

	// application to interact with that stored in Fluence contract
	let appId = "35";

	// create a session between client and backend application
	fluence.connect(contractAddress, appId, ethUrl).then((s) => {
		console.log("Session created");
		window.session = s;
	});

	let BOARD_SIZE = 3;
	let boxes = [];
	let EMPTY = "&nbsp;";
	let current_tile = "X";

	let gameBoard = document.getElementById("tictactoe");

	function createPlayer(name) {
		let request = JSON.stringify({
			action: "create_player",
			player_name: name
		});

		console.log("request: " + request);

		getResultString(session.invoke(request)).then((r) => {
			console.log("response: " + JSON.stringify(r))
		});
	}

	// created, pending, finished
	let gameState = "";

	/*
	{"board":["_","_","_","_","_","_","_","_","_"],"player_tile":"X","winner":"None"}
	 */

	function initState(state) {
		boxes.forEach((v, idx) => {
			let st = state.board[idx];
			if (st === "_") v.innerHTML = EMPTY;
			else v.innerHTML = st;
		});

	}

	function createGame() {

		let name = getName();

		if (name) {
			let request = JSON.stringify({
				action: "Login",
				player_name: name
			});

			console.log("request: " + request);

			getResultString(session.invoke(request)).then((r) => {
				initState(r);
				gameBoard.hidden = false;
				console.log("response: " + JSON.stringify(r))
			});
		} else {
			console.log("create game no name")
		}
	}

	function clearTable() {
		boxes.forEach((b) => {
			b.innerHTML = EMPTY;
		});
	}

	function playerMove(name, x, y) {
		let request = JSON.stringify({
			action: "PlayerMove",
			player_name: name,
			coords: [x, y]
		});

		console.log("request: " + request);

		getResultString(session.invoke(request)).then((r) => {
			console.log("response: " + JSON.stringify(r));
			if (r.winner === "None") {
				let appMove = r.coords;
				let cell = boxes.find((el) => {
					return el.game_col === appMove[1] && el.game_row === appMove[0]
				});
				setInCell(cell, "O", false);
			} else {
				console.log("winner: " + r.winner + "!");
				clearTable()
			}

		});
	}

	window.playerMove = playerMove;
	window.createPlayer = createPlayer;
	window.createGame = createGame;

	/*
	 * Initializes the Tic Tac Toe board and starts the game.
	 */
	function init() {
		var board = document.createElement('table');
		board.setAttribute("border", 1);
		board.setAttribute("cellspacing", 0);

		var identifier = 1;
		for (var i = 0; i < BOARD_SIZE; i++) {
			var row = document.createElement('tr');
			board.appendChild(row);
			for (var j = 0; j < BOARD_SIZE; j++) {
				var cell = document.createElement('td');
				cell.setAttribute('height', 80);
				cell.setAttribute('width', 80);
				cell.setAttribute('align', 'center');
				cell.setAttribute('valign', 'center');
				cell.classList.add('col' + j,'row' + i);
				cell.identifier = identifier;
				cell.game_row = i;
				cell.game_col = j;
				cell.addEventListener("click", set);
				row.appendChild(cell);
				boxes.push(cell);
				identifier += 1;
			}
		}

		document.getElementById("tictactoe").appendChild(board);
		startNewGame();
	}

	function getName() {
		return document.getElementById("login").value
	}

	let createBtn = document.getElementById("create-game");
	createBtn.addEventListener("click", createGame);

	function set() {
		setInCell(this, current_tile, true)
	}

	function setInCell(el, tile, from_player) {
		let name = getName();
		console.log(name);
		console.log(el);
		if (name) {
			if (el.innerHTML === EMPTY) {
				el.innerHTML = tile;
				if (from_player) playerMove(name, el.game_row, el.game_col)
			}
		} else {
			console.log("no name")
		}
	}

	/*
	 * New game
	 */
	function startNewGame() {
		boxes.forEach(function (square) {
			square.innerHTML = EMPTY;
		});
	}

	init();
};
