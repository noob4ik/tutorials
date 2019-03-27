import {log} from "./logger";
export declare function allocate(size: i32): i32;
export declare function deallocate(ptr: i32, size: i32): void;
export declare function invoke(ptr: i32, size: i32): i32;
export declare function store_db(ptr: i32, byte: u8): void;
export declare function load_db(ptr: i32): u8;

function invokeStr(request: string): i32 {
    let strLen: i32 = request.length;
    let addr = allocate(strLen);

    for (let i = 0; i < strLen; i++) {
        let b: u8 = request.charCodeAt(i) as u8;
        store_db(addr + i, b);
    }

    return invoke(addr, strLen);
}

function getResultString(addr: i32): string {
    let lenBytes: u8[] = new Array(4);
    for (let i = 0; i < 4; i++) {
        lenBytes[i] = load_db(addr + i);
    }

    let sizeLen: i32 = 0;

    for (let i = 0; i < 4; i++) {
        sizeLen = sizeLen | (lenBytes[i] << (8*(i - 4) as u8))
    }

    let strBytes = new Uint8Array(sizeLen);

    for (let i = 0; i < sizeLen; i++) {
        strBytes[i] = load_db(addr + i + 4);
    }


    let result = String.fromUTF8(strBytes.buffer.data, strBytes.length);

    deallocate(addr, sizeLen + 4);

    return result;
}

function doRequest(request: string): string {

    log("[doRequest] Request: " + request);

    let resultAddr = invokeStr(request);

    let result = getResultString(resultAddr);

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
