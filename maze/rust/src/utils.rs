#![deny(missing_docs)]
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
/// Parameters for undistorting an image.
/// - fx: focal length x
/// - fy: focal length y
/// - cx: principal point x
/// - cy: principal point y
/// - skew: skew of distortion
/// - k1: radial distortion coefficient
/// - k2: radial distortion coefficient
/// - k3: radial distortion coefficient
/// - t1: tangential distortion coefficient
/// - t2: tangential distortion coefficient
pub struct DistortionOptions {
    /// focal length x
    pub fx: f64,
    /// focal length y
    pub fy: f64,
    /// principal point x
    pub cx: f64,
    /// principal point y
    pub cy: f64,
    /// skew of distortion
    pub skew: f64,
    /// radial distortion coefficient k1
    pub k1: f64,
    /// radial distortion coefficient k2
    pub k2: f64,
    /// radial distortion coefficient k3
    pub k3: f64,
    /// tangential distortion coefficient t1
    pub t1: f64,
    /// tangential distortion coefficient t2
    pub t2: f64,
}

impl Default for DistortionOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl DistortionOptions {
    #[wasm_bindgen(constructor)]
    /// New distortion options.
    pub fn new() -> DistortionOptions {
        DistortionOptions {
            fx: 0.0,
            fy: 0.0,
            cx: 0.0,
            cy: 0.0,
            skew: 0.0,
            k1: 0.0,
            k2: 0.0,
            k3: 0.0,
            t1: 0.0,
            t2: 0.0,
        }
    }

    /// # Description
    /// Inverts the distortion coefficients as given by \
    /// https://www.ncbi.nlm.nih.gov/pmc/articles/PMC4934233/#app3-sensors-16-00807 \
    /// Having 9 coefficients after inversion only matters for repeated stability, so returning only the first three reverse coefficients is not an issue.
    pub fn invert(&self) -> DistortionOptions {
        DistortionOptions {
            cx: self.cx,
            cy: self.cy,
            fx: self.fx,
            fy: self.fy,
            skew: self.skew,
            t1: self.t1,
            t2: self.t2,
            k1: -self.k1,
            k2: 3. * self.k1 * self.k1 - self.k2,
            k3: -12. * f64::powi(self.k1, 3) + 8. * self.k1 * self.k2 - self.k3,
        }
    }
}

#[wasm_bindgen]
/// Parameters for altering distortion results.
/// x: the x offset
/// y: the y offset
/// x_scale: how much to scale x axis
/// y_scale: how much to scale y axis
pub struct DistortionOffsetOptions {
    /// the x offset
    pub x: f64,
    /// the y offset
    pub y: f64,
    /// how much to scale x axis
    pub x_scale: f64,
    /// how much to scale y axis
    pub y_scale: f64,
}

impl Default for DistortionOffsetOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl DistortionOffsetOptions {
    #[wasm_bindgen(constructor)]
    /// New distortion offset options.
    pub fn new() -> DistortionOffsetOptions {
        DistortionOffsetOptions {
            x: 0.0,
            y: 0.0,
            x_scale: 1.0,
            y_scale: 1.0,
        }
    }
}

/// Structure containing points
#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize)]
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
    fn from_ints(t: &(i32, i32)) -> Point {
        Point {
            x: t.0.into(),
            y: t.1.into(),
        }
    }
    /// Finds the distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        f64::sqrt(x * x + y * y)
    }

    /// Transform: "reverse"
    /// positive x offset will result in the image seeming to be shifted left
    /// greater than 1. x scale will result in the image appearing to be scaled down
    pub fn transform(&self, options: &DistortionOffsetOptions, center: &Point) -> Point {
        Point {
            x: options.x_scale * (self.x - center.x) + center.x + options.x,
            y: options.y_scale * (self.y - center.y) + center.y + options.y,
        }
    }
}

/// Structure containing normalized position
#[wasm_bindgen]
#[derive(Debug)]
pub struct NormPoint {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
}

impl NormPoint {
    /// Unnormalize point
    pub fn unnormalized(&self, center: &Point) -> Point {
        Point {
            x: center.x + self.x * center.x,
            y: center.y + self.y * center.y,
        }
    }
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

#[derive(PartialEq)]
/// Simple enum representing either Black or White
pub enum BlackWhite {
    /// Black
    Black,
    /// White
    White,
}

#[wasm_bindgen]
#[derive(Debug)]
/// Tolerance options that can be specified. If a single condition is not met, point is rejected.
pub struct ToleranceOptions {
    /// how far the top left and bottom right can be from black
    pub black_dist: f64,
    ///  how far the bottom left and top right can be from white
    pub white_dist: f64,
    /// how far from the average of the top left, bottom left, top right, and bottom right the color can be \
    /// use because we don't want pixels that are pure white or pure black: center should be somewhat grey
    pub center_dist: f64,
    /// how unlikely the sum of the blacks minus sum of the whites averaged can be and still allow for a corner
    pub avg: f64,
    /// how far away from the intersection the perceived edges the corner can be
    pub intersect_dist: f64,
}

impl Default for ToleranceOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl ToleranceOptions {
    /// The constructor to be used by js
    #[wasm_bindgen(constructor)]
    pub fn new() -> ToleranceOptions {
        ToleranceOptions {
            black_dist: f64::MAX,
            white_dist: f64::MAX,
            center_dist: f64::MAX,
            avg: f64::MAX,
            intersect_dist: f64::MAX,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug)]
/// Weightage options that can be specified. Use for fine tuning the corner detection.
/// All fields should be from 0.0 to 1.0
pub struct WeightageOptions {
    /// weightage for the center point distance to the average of the top left, bottom left, top right, and bottom right
    pub center_dist: f64,
    /// weightage for the distance to black (how important the black corners are)
    pub black_dist: f64,
    /// weightage for the distance to white (how important the white corners are)
    pub white_dist: f64,
    /// weightage for the distance to the intersection
    pub intersect_dist: f64,
    /// weightage for the average of the sum of distances across the viewbox
    pub avg: f64,
}

impl Default for WeightageOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl WeightageOptions {
    /// The constructor to be used by js
    #[wasm_bindgen(constructor)]
    pub fn new() -> WeightageOptions {
        WeightageOptions {
            center_dist: 0.0,
            black_dist: 0.0,
            white_dist: 0.0,
            intersect_dist: 0.0,
            avg: 1.0,
        }
    }
    /// Set the weights such that they sum to 1.0 \
    /// Should **NOT** modify the structure after calling lock \
    /// Prevent the unlikelihood from increasing simply from adding fields
    pub fn lock(&mut self) {
        let sum =
            self.center_dist + self.black_dist + self.white_dist + self.intersect_dist + self.avg;
        self.center_dist /= sum;
        self.black_dist /= sum;
        self.white_dist /= sum;
        self.intersect_dist /= sum;
        self.avg /= sum;
    }
}

#[wasm_bindgen]
/// Essential overall information for the corner detection algorithm
/// - The viewbox range (the distance from center to a corner along one axis)
/// - The pre-condensed number of corners
/// - The valid proximity for condensing
/// - The number of post-condensed corners
pub struct OverallOptions {
    /// The range
    pub view_range: u32,
    /// The number of pre-condensed corners
    pub pre_corners: u32,
    /// The valid proximity for condensing
    pub valid_proximity: f64,
    /// The number of post-condensed corners
    pub post_corners: u32,
}

impl Default for OverallOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl OverallOptions {
    /// The constructor to be used by js\
    /// **Not usable defaults**
    #[wasm_bindgen(constructor)]
    pub fn new() -> OverallOptions {
        OverallOptions {
            view_range: 0,
            pre_corners: 0,
            valid_proximity: 0.0,
            post_corners: 0,
        }
    }
}

/// # Description
/// Returns a point after distorting it by a radial distortion
/// p is the undistorted point in normalized coordinates
/// Uses the formula at
/// https://docs.opencv.org/4.5.2/dc/dbb/tutorial_py_calibration.html
/// # Usage
/// ```
/// use img_tools::utils::{distort_point, Point};
/// panic!();
/// ```
pub fn distort_point(p: NormPoint, distortion_options: &DistortionOptions) -> Point {
    let d = distortion_options;
    let r_2 = f64::powi(p.x, 2) + f64::powi(p.y, 2);
    let expr = 1. + d.k1 * r_2 + d.k2 * f64::powi(r_2, 2) + d.k3 * f64::powi(r_2, 3);
    let x = p.x * expr + d.t2 * (r_2 + 2. * p.x * p.x) + 2. * d.t1 * p.x * p.y;
    let y = p.y * expr + d.t1 * (r_2 + 2. * p.y * p.y) + 2. * d.t2 * p.x * p.y;
    Point {
        x: d.cx + x * d.fx + y * d.skew,
        y: d.cy + y * d.fy,
    }
}

/// # Description
/// Shorthand to access Uint8Array.
/// Requires width of the array in order to index 2d
/// # Errors
/// If i is greater than width or if j is greater than the height
pub fn ij(data: &[u8], i: i32, j: i32, width: u32, height: u32) -> Result<Color, String> {
    if i >= width as i32 || j >= height as i32 {
        return Err("Index i or j is greater than the width or height".to_string());
    } else if j < 0 || i < 0 {
        return Err("Index i or j is less than 0".to_string());
    }

    let base = ((j * width as i32) + i) * 4;
    Ok(Color {
        r: data[base as usize],
        g: data[(base + 1) as usize],
        b: data[(base + 2) as usize],
        a: data[(base + 3) as usize],
    })
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
/// # Errors
/// If the point's correspondings numbers would be outside the image
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
    let x_floor = f64::floor(x) as i32;
    let x_ceil = f64::ceil(x) as i32;
    let y_floor = f64::floor(y) as i32;
    let y_ceil = f64::ceil(y) as i32;
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
/// # Errors
/// If i is greater than width or if j is greater than the height
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

/// # Description
/// Gets how strongly a color is not black or white.
/// Finds square root of (variance between r, g, b + square distance to black or white).
/// # Usage
/// ```
/// panic!();
/// ```
fn get_black_white_unlikelihood(c: &Color, target: BlackWhite) -> f64 {
    // square root of the variance within the color plus the square of the distance to the target
    let mean = (c.r as f64 + c.g as f64 + c.b as f64) / 3.0;
    let var = f64::powi(c.r as f64 - mean, 2)
        + f64::powi(c.g as f64 - mean, 2)
        + f64::powi(c.b as f64 - mean, 2);
    // distance from the target
    let rgb_target = match target {
        BlackWhite::Black => 0.0,
        BlackWhite::White => 255.0,
    };
    let distance = f64::powi(c.r as f64 - rgb_target, 2)
        + f64::powi(c.g as f64 - rgb_target, 2)
        + f64::powi(c.b as f64 - rgb_target, 2);
    f64::sqrt(var + distance)
}

/// # Description
/// Gets the color the pixel is closer to, white or black.
/// # Usage
/// ```
/// panic!();
/// ```
fn get_black_white_closer(c: Color) -> BlackWhite {
    let black_distance = f64::powi(c.r as f64 - 0.0, 2)
        + f64::powi(c.g as f64 - 0.0, 2)
        + f64::powi(c.b as f64 - 0.0, 2);
    let white_distance = f64::powi(c.r as f64 - 255.0, 2)
        + f64::powi(c.g as f64 - 255.0, 2)
        + f64::powi(c.b as f64 - 255.0, 2);
    if black_distance < white_distance {
        BlackWhite::Black
    } else {
        BlackWhite::White
    }
}

/// # Description
/// Finds the pixel along an edge that is the transition between white and black.
/// Walks along the edge until a pixel is found that flips from white to black or vice versa.
/// down_not_right determines if the walking direction is down or right.
/// # Usage
/// ```
/// panic!();
/// ```
#[allow(clippy::too_many_arguments)]
fn get_black_white_transition(
    data: &[u8],
    start_color: BlackWhite,
    i: i32,
    j: i32,
    length: i32,
    width: u32,
    height: u32,
    down_not_right: bool,
) -> Result<(i32, i32), String> {
    if down_not_right {
        // length + 1 because we want to include that last length
        for y in j..(j + length + 1) {
            if get_black_white_closer(ij(&data, i, y, width, height)?) != start_color {
                return Ok((i, y));
            }
        }
    } else {
        // length + 1 because we want to include that last length
        for x in i..(i + length + 1) {
            if get_black_white_closer(ij(&data, x, j, width, height)?) != start_color {
                return Ok((x, j));
            }
        }
    }
    Err("Could not find a transition".to_string())
}

/// # Description
/// Gets the intersection of two lines given by four points.
/// Points specified clockwise.
/// https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
/// # Usage
/// ```
/// panic!();
/// ```
fn get_intersection(p1: Point, p3: Point, p2: Point, p4: Point) -> Result<Point, String> {
    // The points are specified clockwise: first one is p1, second is p3, third, p2, fourth, p4
    let a = (p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x);
    let b = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    let t = a / b;
    if (0.0..=1.0).contains(&t) {
        Ok(Point {
            x: p1.x + t * (p2.x - p1.x),
            y: p1.y + t * (p2.y - p1.y),
        })
    } else {
        Err("Intersection not found".to_string())
    }
}

/// # Description
/// Gets how strongly a pixel is not a corner.
/// Data is the image data, i and j are the pixel coordinates, width and height are the image dimensions.
/// 2 * Range + 1 is the side length of the square, centered around i and j, which is used to check for cornerness.
/// Options contains the tolerances.
/// # Usage
/// ```
/// panic!();
/// ```
/// # Errors
/// If range and i,j is ever out of bounds.
#[allow(clippy::too_many_arguments)]
pub fn get_corner_unlikelihood(
    data: &[u8],
    i: u32,
    j: u32,
    width: u32,
    height: u32,
    range: u32,
    options: &ToleranceOptions,
    weightings: &WeightageOptions,
) -> Result<f64, String> {
    let i_i32 = i as i32;
    let j_i32 = j as i32;
    let r_i32 = range as i32;
    let ip_i32 = i_i32 + r_i32;
    let jp_i32 = j_i32 + r_i32;
    let im_i32 = i_i32 as i32 - r_i32 as i32;
    let jm_i32 = j_i32 as i32 - r_i32 as i32;

    let mut unlikelihood = 0.0;

    let corner_colors = vec![
        ij(data, im_i32, jm_i32, width, height)?,
        ij(data, im_i32, jp_i32, width, height)?,
        ij(data, ip_i32, jm_i32, width, height)?,
        ij(data, ip_i32, jp_i32, width, height)?,
    ];

    let center_color = ij(data, i_i32, j_i32, width, height)?;
    let avg = corner_colors.iter().fold(
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
        |sum, c| {
            sum.interpolate(c, 0.5).unwrap_or(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            })
        },
    );
    let err = f64::powi(avg.r as f64 - center_color.r as f64, 2)
        + f64::powi(avg.g as f64 - center_color.g as f64, 2)
        + f64::powi(avg.b as f64 - center_color.b as f64, 2);
    if err > options.center_dist {
        return Err("Center color is too different form average of corner colors".to_string());
    }
    unlikelihood += err * weightings.center_dist;

    // check corners of the square
    let corner_dists = vec![
        get_black_white_unlikelihood(&corner_colors[0], BlackWhite::Black),
        get_black_white_unlikelihood(&corner_colors[1], BlackWhite::White),
        get_black_white_unlikelihood(&corner_colors[2], BlackWhite::White),
        get_black_white_unlikelihood(&corner_colors[3], BlackWhite::Black),
    ];
    if corner_dists[0] > options.black_dist
        || corner_dists[1] > options.white_dist
        || corner_dists[2] > options.white_dist
        || corner_dists[3] > options.black_dist
    {
        return Err("Corners exceed unlikelihood tolerance".to_string());
    }
    unlikelihood += ((corner_dists[0] + corner_dists[3]) * weightings.black_dist
        + (corner_dists[1] + corner_dists[2]) * weightings.white_dist)
        / 4.;

    let length = 2 * r_i32 + 1;
    // walk right along the top edge from the top left corner
    let top = get_black_white_transition(
        data,
        BlackWhite::Black,
        im_i32,
        jm_i32,
        length,
        width,
        height,
        false,
    )?;
    // walk down along the left edge from the top left corner
    let left = get_black_white_transition(
        data,
        BlackWhite::Black,
        im_i32,
        jm_i32,
        length,
        width,
        height,
        true,
    )?;
    // walk down along the right edge from the top right corner
    let right = get_black_white_transition(
        data,
        BlackWhite::White,
        ip_i32,
        jm_i32,
        length,
        width,
        height,
        true,
    )?;
    // walk right along the bottom edge from the bottom left corner
    let bottom = get_black_white_transition(
        data,
        BlackWhite::White,
        im_i32,
        jp_i32,
        length,
        width,
        height,
        false,
    )?;
    // check that the point is close to the intersection of the lines formed by the four transitions
    let intersection = get_intersection(
        Point::from_ints(&top),
        Point::from_ints(&right),
        Point::from_ints(&bottom),
        Point::from_ints(&left),
    )?;

    let distance = (Point::distance(&intersection, &Point::from_ints(&top))
        + Point::distance(&intersection, &Point::from_ints(&right))
        + Point::distance(&intersection, &Point::from_ints(&left))
        + Point::distance(&intersection, &Point::from_ints(&bottom)))
        / 4.;
    if distance > options.intersect_dist {
        return Err("Intersection is too far away".to_string());
    }
    unlikelihood += distance * weightings.intersect_dist;

    let mut sum_unlikelihood = 0.0;
    for x in (im_i32)..=(ip_i32) {
        for y in (jm_i32)..=(jp_i32) {
            // four line segments: top to center, right to center, bottom to center, left to center
            // left / above = true
            let center_to_top = (top.0 - i_i32) * (y - j_i32) > (top.1 - j_i32) * (x - i_i32);
            let center_to_right = (right.0 - i_i32) * (y - j_i32) > (right.1 - j_i32) * (x - i_i32);
            let bottom_to_center =
                (i_i32 - bottom.0) * (y - bottom.1) > (j_i32 - bottom.1) * (x - bottom.0);
            let left_to_center = (i_i32 - left.0) * (y - left.1) > (j_i32 - left.1) * (x - left.0);

            if center_to_top && left_to_center || !center_to_right && !bottom_to_center {
                sum_unlikelihood += get_black_white_unlikelihood(
                    &ij(data, x, y, width, height)?,
                    BlackWhite::Black,
                );
            } else {
                sum_unlikelihood += get_black_white_unlikelihood(
                    &ij(data, x, y, width, height)?,
                    BlackWhite::White,
                );
            }
        }
    }
    sum_unlikelihood /= (ip_i32 - im_i32) as f64 * (jp_i32 - jm_i32) as f64;
    unlikelihood += sum_unlikelihood * weightings.avg;
    if sum_unlikelihood > options.avg {
        Err("Sum error is too high".to_string())
    } else {
        Ok(unlikelihood)
    }
}

/// # Description
/// For condensing output of `detect_corners`.
/// Mashes neigboring pixels (as determined by `proximity`) together.
/// Returns the specified number of corners based off the number of pixels corresponding to each grouping.
/// # Usage
/// ```
/// panic!();
/// ```
pub fn condense_corners(points: &[Point], proximity: f64, num_corners: u32) -> Vec<Point> {
    let mut output: Vec<(u32, Point)> = Vec::new();
    points.iter().for_each(|p| {
        if p.x != 0.0 && p.y != 0.0 {
            if let Some(index) = output
                .iter()
                .position(|(_, group)| group.distance(p) < proximity)
            {
                let count = output[index].0;
                let mut group = &mut output[index].1;
                group.x = ((group.x) * count as f64 + p.x) / (count + 1) as f64;
                group.y = ((group.y) * count as f64 + p.y) / (count + 1) as f64;
                output[index].0 += 1;
            } else {
                output.push((1, p.clone()));
            }
        }
    });
    output.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    output.truncate(num_corners as usize);
    output.into_iter().map(|a| a.1).collect::<Vec<Point>>()
}
