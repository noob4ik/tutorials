#[link(wasm_import_module = "proxy-as")]
extern {
    pub fn invoke() -> i32;
}