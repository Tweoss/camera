/// utils module for processing
pub mod utils;

use js_sys::{Array, ArrayBuffer, Uint8Array};
use utils::{
    bilinear_interpolation, distort_point, get_corner_unlikelihood, set_color, Color, NormPoint,
    Point, ToleranceOptions, WeightageOptions,
};
use wasm_bindgen::prelude::*;
#[allow(unused_imports)] // for logging
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[allow(unused_macros)]
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
/// Takes an array buffer containing the image data (RGBA) as well as its width and height.
/// k1, k2, and k3 are the radial distortion coefficients.
/// Returns a new Uint8Array containing the transformed image data. (positive coeffs correspond to barrel, and negative to pincushion distortion.)
pub fn undistort_data(
    in_data: ArrayBuffer,
    width: u32,
    height: u32,
    k1: f64,
    k2: f64,
    k3: f64,
) -> Uint8Array {
    // copying the input data buffer prevents losing it due to malloc
    let temp_data = Uint8Array::new(&in_data);
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
            // safe to expect here, because we defined the array in js from same value that gave width and height
            // if set_color is err, then js logic is incorrect
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
/// Takes an array buffer containing the image data (RGBA) as well as its width and height.
/// Returns an array containing the four most likely chessboard corners.
pub fn detect_corners(
    in_data: ArrayBuffer,
    width: u32,
    height: u32,
    range: u32,
    tolerances: ToleranceOptions,
    weightings: WeightageOptions,
) -> Array {
    // according to the js sys docs, this (Uint8Array and then to_vec) only creates a single copy of the input data
    // copying the input data buffer prevents losing it due to malloc
    let temp_data = Uint8Array::new(&in_data);
    // using a vec for better panic messages
    let vec = temp_data.to_vec();

    // make sure this output vector is always correctly sorted by unlikelihood increasing
    // might make the checking slightly faster if the valid points are sparse (as they should be)
    let mut output = vec![(f64::MAX, Point { x: 0.0, y: 0.0 }); 8];
    for i in 0..width {
        for j in 0..height {
            let unlikelihood =
                get_corner_unlikelihood(&vec, i, j, width, height, range, &tolerances, &weightings)
                    .unwrap_or(f64::MAX);
            if unlikelihood < output[0].0 {
                output[0] = (
                    unlikelihood,
                    Point {
                        x: i.into(),
                        y: j.into(),
                    },
                );
                output.sort_unstable_by(|b, a| a.0.partial_cmp(&b.0).unwrap());
            }
        }
    }
    log!("output: {:?}", output);
    output.into_iter().map(|a| a.1).map(JsValue::from).collect()
}

/// For viewing the unlikelihoods in an image. useful for selecting weights
#[wasm_bindgen]
pub fn corner_map(
    in_data: ArrayBuffer,
    width: u32,
    height: u32,
    range: u32,
    opt: ToleranceOptions,
    weight: WeightageOptions,
) -> Uint8Array {
    let temp_data = Uint8Array::new(&in_data);
    let vec = temp_data.to_vec();

    let scale_factor = 1.0;
    let mut output = Uint8Array::new_with_length(vec.len() as u32);

    let mut max = 0.;
    for i in 0..width {
        for j in 0..height {
            let temp_i = i as i32;
            let temp_j = j as i32;
            let mut unlikelihood =
                get_corner_unlikelihood(&vec, i, j, width, height, range, &opt, &weight)
                    .unwrap_or_else(|err| {
                        0xFF.into()
                    });
            unlikelihood *= scale_factor;
            let c = if unlikelihood > 255. {
                255
            } else {
                unlikelihood as u8
            };
            let color = Color {
                r: c,
                g: c,
                b: c,
                a: 255,
            };
            let _ = set_color(color, &mut output, i, j, width, height);
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
