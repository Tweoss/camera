#![deny(missing_docs)]
use js_sys::Uint8Array;

/// Structure containing points
#[derive(Debug)]
pub struct Point {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
}

impl Point {
    /// Normalize point
    pub fn normalized(&self, center: &Point) -> NormPoint {
        NormPoint {
            x: (self.x - center.x) / center.x,
            y: (self.y - center.y) / center.y,
        }
    }
    /// Undo normalization
    pub fn unnormalized(&self, center: &Point) -> Point {
        Point {
            x: (self.x * center.x + center.x),
            y: (self.y * center.y + center.y),
        }
    }
    /// Unscale
    pub fn unscale(&self, scale: f64) -> Point {
        Point {
            x: self.x / scale,
            y: self.y / scale,
        }
    }
}

/// Structure containing normalized position
#[derive(Debug)]
pub struct NormPoint {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
}

/// Structure holding color information
#[derive(Debug, PartialEq)]
pub struct Color {
    /// red component
    pub r: u8,
    /// green component
    pub g: u8,
    /// blue component
    pub b: u8,
    /// alpha component
    pub a: u8,
}

impl Color {
    /// Interpolate two values
    /// Alpha ought to be 0xFF in order for interpolation to work as expected
    pub fn interpolate(&self, other: &Color, t: f64) -> Result<Color, String> {
        if self.a != 0xFF || other.a != 0xFF {
            Err("Alpha should be 0xFF for interpolation to work correctly".to_string())
        } else {
            Ok(Color {
                r: ((self.r as f64 * (1. - t)) + (other.r as f64 * t)) as u8,
                g: ((self.g as f64 * (1. - t)) + (other.g as f64 * t)) as u8,
                b: ((self.b as f64 * (1. - t)) + (other.b as f64 * t)) as u8,
                a: ((self.a as f64 * (1. - t)) + (other.a as f64 * t)) as u8,
            })
        }
    }
}

/// # Description
/// Returns a point after correction for radial distortion
/// p is the distorted point, c is the center of distortion, and k1 through k4 are the radial distortion coefficients.
/// # Usage
/// ```
/// use img_tools::utils::{undistort_point, Point};
/// let original_point = Point { x: 1.0, y: 2.0 };
/// let center_point = Point { x: 0.0, y: 0.0 };
/// let (k1, k2, k3) = (0.2, 0.0, 0.0);
/// // assert_eq!(Point { x: 0.5, y: 1.0 }, undistort_point(original_point, center_point, k1, k2, k3));
/// // assert_eq!(Point { x: 0.0, y: 0.0 }, undistort_point(original_point, center_point, k1, k2, k3));
/// panic!();
/// ```
pub fn undistort_point(p: NormPoint, c: NormPoint, k1: f64, k2: f64, k3: f64) -> Point {
    let d = f64::sqrt(f64::powi(p.x - c.x, 2) + f64::powi(p.y - c.y, 2));
    let expr = 1. + k1 * f64::powi(d, 2) + k2 * f64::powi(d, 4) + k3 * f64::powi(d, 6);
    Point {
        x: c.x + (p.x - c.x) / expr,
        y: c.y + (p.y - c.y) / expr,
    }
}

/// # Description
/// Returns a point after distorting it by a radial distortion
/// p is the undistorted point, c is the center of distortion, and k1 through k4 are the radial distortion coefficients.
/// Uses the formula at
/// https://docs.opencv.org/4.5.2/dc/dbb/tutorial_py_calibration.html
/// # Usage
/// ```
/// use img_tools::utils::{distort_point, Point};
/// panic!();
/// ```
pub fn distort_point(p: NormPoint, c: NormPoint, k1: f64, k2: f64, k3: f64) -> Point {
    let d = f64::sqrt(f64::powi(p.x - c.x, 2) + f64::powi(p.y - c.y, 2));
    let expr = 1. + k1 * f64::powi(d, 2) + k2 * f64::powi(d, 4) + k3 * f64::powi(d, 6);
    Point {
        x: c.x + (p.x - c.x) * expr,
        y: c.y + (p.y - c.y) * expr,
    }
}
// pub fn bilinear_interpolation(p: Point, a: &[], b, c, d){

// };

/// # Description
/// Shorthand to access Uint8Array.
/// Requires width of the array in order to index 2d
/// # Errors
/// If i is greater than width or if i * j is greater than the length of the array
pub fn ij(data: &[u8], i: usize, j: usize, width: u32, height: u32) -> Result<Color, String> {
    if i >= width as usize {
        return Err("Index i is greater than width".to_string());
    } else if j >= height as usize {
        return Err("Index j is greater than height".to_string());
    }
    let base = ((j * width as usize) + i) * 4;
    Ok(Color {
        r: data[base as usize],
        g: data[(base + 1) as usize],
        b: data[(base + 2) as usize],
        a: data[(base + 3) as usize],
    })
}

/// # Description
/// How much the corners will scale.
/// Divide by the scale factor to keep the edges at the original distance.
/// (assumes that the distortion is perfectly barrel with reverse being pincushion and the edges will therefore be the furthest out)
/// Note: should use the same formula as distort_point
/// # Usage
/// ```
/// use img_tools::utils::scale_factor;
/// assert_eq!(1.1, scale_factor(0.05, 0.0, 0.0));
/// ```
pub fn scale_factor(k1: f64, k2: f64, k3: f64) -> f64 {
    // distance of the edge in normalized coords
    let d = f64::sqrt(2.);
    // how much the edge scales
    1. + k1 * f64::powi(d, 2) + k2 * f64::powi(d, 4) + k3 * f64::powi(d, 6)
}

/// # Description
/// Performs bilinear interpolation at the given position (in floats)
/// # Usage
/// ```
/// use img_tools::utils::{bilinear_interpolation, Point, Color};
/// let vec = vec![1, 0, 0, 255, 2, 0, 0, 255, 3, 0, 0, 255, 4, 0, 0, 255];
/// let (width, height) = (2, 2);
/// let point = Point { x: 0.5, y: 0.5 };
/// let expected_output = Color { 
///     r: ((((1. + 2.) / 2.) as u8) + ((3. + 4.) / 2.) as u8) / 2, 
///     g: 0, 
///     b: 0, 
///     a: 255 
/// };
/// assert_eq!(Ok(expected_output), bilinear_interpolation(&point, &vec, width, height));
/// ```
pub fn bilinear_interpolation(
    distorted_point: &Point,
    array: &[u8],
    width: u32,
    height: u32,
) -> Result<Color, String> {
    let (x, y) = (distorted_point.x, distorted_point.y);
    if x < -0.0 || y < 0.0 {
        return Err("Point has negative coordinates.".to_string());
    }
    let x_floor = f64::floor(x) as usize;
    let x_ceil = f64::ceil(x) as usize;
    let y_floor = f64::floor(y) as usize;
    let y_ceil = f64::ceil(y) as usize;
    let x_frac = x - x_floor as f64;
    let y_frac = y - y_floor as f64;

    let left = ij(&array, x_floor, y_floor, width, height)?
        .interpolate(&ij(&array, x_floor, y_ceil, width, height)?, y_frac)?;
    let right = ij(&array, x_ceil, y_floor, width, height)?
        .interpolate(&ij(&array, x_ceil, y_ceil, width, height)?, y_frac)?;
    left.interpolate(&right, x_frac)
}

/// # Description
/// Sets a pixel in the Uint8Array to the given color.
/// Returns error if the indices are out of bounds.
/// # Usage
/// ```
/// panic!();
/// ```

pub fn set_color(
    color: Color,
    data: &mut Uint8Array,
    i: u32,
    j: u32,
    width: u32,
    height: u32,
) -> Result<(), String> {
    if i > width {
        return Err("Index i is greater than width".to_string());
    } else if j > height {
        return Err("Index j is greater than height".to_string());
    }
    let base = ((j * width) + i) * 4;
    data.set_index(base, color.r);
    data.set_index(base + 1, color.g);
    data.set_index(base + 2, color.b);
    data.set_index(base + 3, color.a);
    Ok(())
}
