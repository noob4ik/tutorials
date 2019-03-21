import "allocator/buddy";
import {invoke as counterInvoke} from "./counter";

export function invoke(): i32 {
    return counterInvoke(0, 0);
}
