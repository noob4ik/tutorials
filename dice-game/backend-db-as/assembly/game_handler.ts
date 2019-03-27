import {Action, decode, GetBalanceRequest, Request, RollRequest, UnknownRequest} from "./request";
import {ErrorResponse} from "./response";
import {GameManager} from "./dice";
import {log} from "./logger";
import {checkSignature} from "../node_modules/signature-connector/assembly/index"

let gameManager = new GameManager();

// returns string, because serialization to a byte array is not compatible with our invoke handlers
export function handler(requestBytes: Uint8Array): string {

    //TODO add admin commands and check signature for this commands
    if (false) {
        let requestStr = String.fromUTF8(requestBytes.buffer.data, requestBytes.length);

        log("requestStr: " + requestStr);

        let checkResult = checkSignature(requestStr);

        if (!checkResult.checkPassed) {
            log("Error check signature: " + checkResult.errorMessage)
        }
    }



    let request: Request = decode(requestBytes);

    let response: string;

    if (request.action == Action.Join) {
        log("Handling Join request\n");
        response = gameManager.join();
    } else if (request.action == Action.Roll) {
        log("Handling Roll request\n");
        let r = request as RollRequest;
        response = gameManager.roll(r.playerId, r.betPlacement, r.betSize);
    } else if (request.action == Action.GetBalance) {
        log("Handling GetBalance request\n");
        let r = request as GetBalanceRequest;
        response = gameManager.getBalance(r.playerId);
    } else if (request.action == Action.Unknown) {
        log("Unknown request\n");
        let r = request as UnknownRequest;
        let error = new ErrorResponse(r.message);
        let response = error.serialize();
        memory.free(changetype<usize>(error));
    } else {
        let error = new ErrorResponse("Unreachable.");
        response = error.serialize();
    }

    log("Response: " + response);

    return response;
}
