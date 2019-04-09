import {ErrorResponse, GetBalanceResponse, JoinResponse, RollResponse} from "./response";
import {
    countPlayers,
    createPlayer,
    deletePlayer,
    getPlayersBalance, initTables,
    maximumId,
    minimumId,
    updateBalance
} from "./database";
import {log} from "../node_modules/assemblyscript-sdk/assembly/logger";

const PLAYERS_MAX_COUNT: i32 = 1024;
const SEED: u64 = 123456;
// the account balance of new players
const INIT_ACCOUNT_BALANCE: u64 = 100;
// if win, player receives bet_amount * PAYOUT_RATE money
const PAYOUT_RATE: u64 = 5;
const DICE_LINE_COUNT: u8 = 6;

export class GameManager {

    constructor() {
        NativeMath.seedRandom(SEED);
        initTables();
    }

    join(): string {
        // delete the oldest player, if maximum players reach

        let playerNumber = countPlayers();

        if (playerNumber >= PLAYERS_MAX_COUNT) {
            let lastPlayer = minimumId();
            deletePlayer(lastPlayer);
        }

        let newPlayerId: u64;
        if (playerNumber == 0) {
            newPlayerId = 0;
        } else {
            newPlayerId = maximumId() + 1
        }

        createPlayer(newPlayerId, INIT_ACCOUNT_BALANCE);

        let response = new JoinResponse(newPlayerId);

        let resultStr = response.serialize();
        return resultStr;
    }

    roll(playerId: u64, betPlacement: u8, betSize: u64): string {

        if (betPlacement > DICE_LINE_COUNT) {
            let error = new ErrorResponse("Incorrect placement, please choose number from 1 to 6");
            return error.serialize();
        }

        let balance = getPlayersBalance(playerId);

        if (!balance) {
            let error = new ErrorResponse("There is no player with such id: " + playerId.toString());
            return error.serialize();
        }

        if (betSize > balance) {
            let error = new ErrorResponse("Player hasn't enough money: player's current balance is " + balance.toString()  + " while the bet is " + betSize.toString());
            return error.serialize();
        }

        let outcome = ((Math.random() * 1000000) % DICE_LINE_COUNT + 1) as u8;

        let newBalance: u64 = 0;

        if (betPlacement == outcome) {
            newBalance = balance + (betSize * PAYOUT_RATE);
        } else {
            newBalance = balance - betSize;
        }

        updateBalance(playerId, newBalance);

        let response = new RollResponse(outcome, newBalance);
        let resultStr = response.serialize();
        memory.free(changetype<usize>(response));
        return resultStr;
    }

    getBalance(playerId: u64): string {
        let balance = getPlayersBalance(playerId);
        if (!balance) {
            let error = new ErrorResponse("There is no player with id: " + playerId.toString());
            return error.serialize();
        }
        let response = new GetBalanceResponse(balance);
        return response.serialize();
    }
}
