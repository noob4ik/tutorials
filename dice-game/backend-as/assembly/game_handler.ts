import {Action, decode, GetBalanceRequest, Request, RollRequest, UnknownRequest} from "./request";
import {ErrorResponse} from "./response";
import {GameManager} from "./dice";
import {log} from "./logger";

let gameManager = new GameManager();

// returns string, because serialization to a byte array is not compatible with our invoke handlers
export function handler(requestBytes: Uint8Array): string {

    let request: Request = decode(requestBytes);

    if (request.action == Action.Join) {
        log("Handling Join request\n");
        return gameManager.join();
    } else if (request.action == Action.Roll) {
        log("Handling Roll request\n");
        let r = request as RollRequest;
        return gameManager.roll(r.playerId, r.betPlacement, r.betSize);
    } else if (request.action == Action.GetBalance) {
        log("Handling GetBalance request\n");
        let r = request as GetBalanceRequest;
        return gameManager.getBalance(r.playerId);
    } else if (request.action == Action.Unknown) {
        log("Unknown request\n");
        let r = request as UnknownRequest;
        let error = new ErrorResponse(r.message);
        let returnStr = error.serialize();
        memory.free(changetype<usize>(error));
        return returnStr;
    }

    memory.free(changetype<usize>(request));

    let response = new ErrorResponse("Unreachable.");
    let returnStr = response.serialize();
    memory.free(changetype<usize>(response));
    return returnStr;
}
