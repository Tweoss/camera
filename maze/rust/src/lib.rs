/// utils module for processing
pub mod utils;

use js_sys::Uint8Array;
use utils::{
    bilinear_interpolation, distort_point, set_color, Color, NormPoint, Point,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn undistort_data(
    in_data: Uint8Array,
    width: u32,
    height: u32,
    k1: f64,
    k2: f64,
    k3: f64,
) -> Uint8Array {
    // copying the input data buffer prevents losing it due to malloc
    let temp_data = Uint8Array::new(&in_data.slice(0, in_data.byte_length()));
    // using a vec for better panic messages
    let vec = temp_data.to_vec();
    
    let mut output = Uint8Array::new_with_length(vec.len() as u32);
    let scale = utils::scale_factor(k1, k2, k3);
    let center = Point {
        x: width as f64 / 2.,
        y: height as f64 / 2.,
    };
    // iterate over "output," transform each pixel to distorted, interpolate to determine color, then place in output array
    for i in 0..width {
        for j in 0..height {
            let transformed = distort_point(
                Point {
                    x: i.into(),
                    y: j.into(),
                }
                .normalized(&center),
                NormPoint { x: 0., y: 0. },
                k1,
                k2,
                k3,
            )
            .unscale(scale)
            .unnormalized(&center);
            
            let color = bilinear_interpolation(&transformed, &vec, width, height);
            // if the canvas has any transparent pixels, the corresponding output will be null (0, 0, 0, 0xFF)
            // safe to unwrap here, because we defined the array in js from same value that gave width and height
            set_color(
                color.unwrap_or(Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 255,
                }),
                &mut output,
                i,
                j,
                width,
                height,
            )
            .expect("failed to set color: js passed invalid array or width and height");
        }
    }
    output
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(clippy::eq_op)]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
