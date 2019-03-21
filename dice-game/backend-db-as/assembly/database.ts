export declare function allocate(size: i32): i32;
export declare function deallocate(ptr: i32, size: i32): void;
export declare function invoke(ptr: i32, size: i32): i32;
export declare function storeLlama(ptr: i32, byte: u8): void;
export declare function loadLlama(ptr: i32): u8;

function doRequest(request: string): string {

    let strLen: i32 = request.length;
    let addr = allocate(strLen);

    for (let i = 0; i < strLen; i++) {
        let b: u8 = request.charCodeAt(i) as u8;
        storeLlama(addr + i, b);
    }

    let resultAddr = invoke(addr, strLen);


    let lenBytes: u8[] = new Array(4);
    for (let i = 0; i < 4; i++) {
        lenBytes[i] = loadLlama(resultAddr + i);
    }

    let sizeLen: i32 = 0;

    for (let i = 0; i < 4; i++) {
        strLen = strLen | (lenBytes[i] << (8*i as u8))
    }

    let strBytes: u8[] = new Array(sizeLen);

    for (let i = 0; i < sizeLen; i++) {
        strBytes[i] = loadLlama(resultAddr + i + 4);
    }

    deallocate(resultAddr, sizeLen + 4);
    let result = String.fromUTF8(changetype<usize>(strBytes), strBytes.length);

    return result;
}

export function initTables(): void {
    let request = "CREATE TABLE dice_players(id u64, balance u64)";

    doRequest(request);
}

export function createPlayer(id: u64, balance: u64): string {
    let request = "INSERT INTO dice_players VALUES(" + id.toString() + ", " + balance.toString() + ")";

    let result = doRequest(request);
    return result;
}

export function getPlayersBalance(id: u64): u64 {
    let request = "SELECT balance FROM dice_game WHERE id = " + id.toString();
    let result = doRequest(request);
    return U64.parseInt(result);
}

export function updateBalance(id: u64, balance: u64): string {
    let request = "UPDATE dice_players as u set u.balance = " + balance.toString() + " where u.id = " + id.toString() + ";";
    let result = doRequest(request);
    return result;
}

export function countPlayers(): i32 {
    let request = "SELECT COUNT(*) FROM dice_players";
    let result = doRequest(request);
    return I32.parseInt(result);
}

export function deletePlayer(id: u64): string {
    let request = "DELETE FROM dice_players where id = " + id.toString();
    let result = doRequest(request);
    return result;
}

export function maximumId(): u64 {
    let request = "SELECT MAX(id) FROM dice_players";
    let result = doRequest(request);
    return U64.parseInt(result);
}

export function minimumId(): u64 {
    let request = "SELECT MIN(id) FROM dice_players";
    let result = doRequest(request);
    return U64.parseInt(result);
}
