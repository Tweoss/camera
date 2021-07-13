mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet() -> String {
    String::from("Hello World!")
}


#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::eq_op)]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
