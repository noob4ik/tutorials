import {log} from "./logger";

export declare function allocate(size: i32): i32;
export declare function deallocate(ptr: i32, size: i32): void;
export declare function invoke(ptr: i32, size: i32): i32;
export declare function store_db(ptr: i32, byte: u8): void;
export declare function load_db(ptr: i32): u8;

function doRequest(request: string): string {

    log("[doRequest] Request: " + request);

    let strLen: i32 = request.length;
    let addr = allocate(strLen);
    log("str len: " + strLen.toString());
    log("addr: " + addr.toString());

    for (let i = 0; i < strLen; i++) {
        let b: u8 = request.charCodeAt(i) as u8;
        store_db(addr + i, b);
    }

    let resultAddr = invoke(addr, strLen);

    log("[doRequest] Result addr: " + resultAddr.toString());


    let lenBytes: u8[] = new Array(4);
    for (let i = 0; i < 4; i++) {
        let b = load_db(resultAddr + i);
        log("[doRequest] len byte: " + b.toString());
        lenBytes[i] = b;
    }

    let sizeLen: i32 = 0;

    for (let i = 0; i < 4; i++) {
        sizeLen = sizeLen | (lenBytes[i] << (8*(i - 4) as u8))
        log("[doRequest] str len loop: " + sizeLen.toString());
    }

    log("[doRequest] Size len: " + sizeLen.toString());

    let strBytes = new Uint8Array(sizeLen);

    for (let i = 0; i < sizeLen; i++) {
        strBytes[i] = load_db(resultAddr + i + 4);
    }

    log("[doRequest] Get result");


    let result = String.fromUTF8(strBytes.buffer.data, strBytes.length);

    log("[doRequest] Result: ");
    log("[doRequest] " + result);

    log("[doRequest] Deallocate: " + resultAddr.toString());

    // deallocate(resultAddr, sizeLen + 4);

    return result;
}

export function initTables(): void {
    let request = "CREATE TABLE dice_players(id u64, balance u64)";
    log("Creating table dice_players");
    let resp = doRequest(request);
}

export function createPlayer(id: u64, balance: u64): void {
    let request = "INSERT INTO dice_players VALUES(" + id.toString() + ", " + balance.toString() + ")";
    log("Insert player " + id.toString());
    doRequest(request);
}

export function getPlayersBalance(id: u64): u64 {
    let request = "SELECT balance FROM dice_game WHERE id = " + id.toString();
    log("Select player: " + id.toString());
    let result = doRequest(request);
    let balance = result.split("\n")[1];
    return U64.parseInt(balance);
}

export function updateBalance(id: u64, balance: u64): void {
    let request = "UPDATE dice_players as u set u.balance = " + balance.toString() + " where u.id = " + id.toString() + ";";
    log("Update balance. id: " + id.toString() + ", balance: " + balance.toString());
    doRequest(request);
}

export function countPlayers(): i32 {
    let request = "SELECT COUNT(*) FROM dice_players";
    let result = doRequest(request);
    log("Count players result: " + result);
    let count = result.split("\n")[1];
    if (count) {
        return 0;
    } else {
        return I32.parseInt(count);
    }
}

export function deletePlayer(id: u64): void {
    let request = "DELETE FROM dice_players where id = " + id.toString();
    log("Delete player " + id.toString());
    doRequest(request);
}

export function maximumId(): u64 {
    let request = "SELECT MAX(id) FROM dice_players";
    let result = doRequest(request);
    log("Maximum ID result: " + result);
    let maximum = result.split("\n")[1];
    return U64.parseInt(maximum);
}

export function minimumId(): u64 {
    let request = "SELECT MIN(id) FROM dice_players";
    let result = doRequest(request);
    log("Minimum ID result: " + result);
    let minimum = result.split("\n")[1];
    return U64.parseInt(minimum);
}
