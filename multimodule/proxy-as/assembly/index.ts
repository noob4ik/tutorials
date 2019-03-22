import "allocator/buddy";
import {invoke} from "./counter";

export function invokeProxy(): i32 {
    return invoke(0, 0);
}
