import {log} from "../node_modules/assemblyscript-sdk/assembly/logger";
import {query} from "../node_modules/db-connector/assembly/index"

function doRequest(request: string): string {

    log("[doRequest] Request: " + request);

    let result = query(request);

    log("[doRequest] Result: ");
    log("[doRequest] " + result);

    return result;
}

export function initTables(): void {
    let request = "CREATE TABLE dice_players(id u64, balance u64)";
    let resp = doRequest(request);
}

export function createPlayer(id: u64, balance: u64): void {
    let request = "INSERT INTO dice_players VALUES(" + id.toString() + ", " + balance.toString() + ")";
    doRequest(request);
}

export function getPlayersBalance(id: u64): u64 {
    let request = "SELECT balance FROM dice_players WHERE id = " + id.toString();
    let result = doRequest(request);
    let balance = result.split("\n")[1];
    return U64.parseInt(balance);
}

export function updateBalance(id: u64, balance: u64): void {
    let request = "UPDATE dice_players as u set u.balance = " + balance.toString() + " where u.id = " + id.toString() + ";";
    doRequest(request);
}

export function countPlayers(): i32 {
    let request = "SELECT COUNT(*) FROM dice_players";
    let result = doRequest(request);
    let count = result.split("\n")[1].trim();
    if (!count || count.length == 0) {
        return 0;
    } else {
        return I32.parseInt(count);
    }
}

export function deletePlayer(id: u64): void {
    let request = "DELETE FROM dice_players where id = " + id.toString();
    doRequest(request);
}

export function maximumId(): u64 {
    let request = "SELECT MAX(id) FROM dice_players";
    let result = doRequest(request);
    let maximum = result.split("\n")[1];
    return U64.parseInt(maximum);
}

export function minimumId(): u64 {
    let request = "SELECT MIN(id) FROM dice_players";
    let result = doRequest(request);
    let minimum = result.split("\n")[1];
    return U64.parseInt(minimum);
}
