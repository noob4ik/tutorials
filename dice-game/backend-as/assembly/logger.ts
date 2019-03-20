export declare function write(b: i32): void;
export declare function flush(): void;

export function log(message: string): void {
    let strLen: i32 = message.length;

    for (let i = 0; i < strLen; i++) {
        let b: u8 = message.charCodeAt(i) as u8;
        write(b);
    }
    flush();
}
