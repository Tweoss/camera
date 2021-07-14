/// utils module for processing
pub mod utils;

use wasm_bindgen::prelude::*;
use js_sys::{ArrayBuffer, Uint8Array};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(input: &mut [u8]) -> Vec<u8> {
    input[0] = 10; // just changing some value here
    Vec::from(input)
}

#[wasm_bindgen]
pub fn args(buffer: Uint8Array) -> String {
    buffer.to_vec()[0].to_string()
    // let a = buffer[0];
    // a.to_string()
    // "success".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::eq_op)]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
