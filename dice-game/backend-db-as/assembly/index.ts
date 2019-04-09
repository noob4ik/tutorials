// only this allocator can reset it's state
import "allocator/arena";

import {handler} from "./game_handler";
import {loggedStringHandler} from "../node_modules/assemblyscript-sdk/assembly/index";
import {log} from "./logger";

export function allocate(size: usize) :i32 {
  return memory.allocate(size);
}

export function deallocate(ptr: i32, size: usize): void {
  memory.reset();
}

export function invoke(ptr: i32, size: i32): i32 {
    return loggedStringHandler(ptr, size, handler, log);
}
