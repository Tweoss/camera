#![deny(missing_docs)]

/// Structure containing points
pub struct Point {
	/// x coord
	pub x: f64,
	/// y coord
	pub y: f64,
}

/// Structure holding color information
pub struct Color {
	/// red component
	pub r: u8,
	/// green component
	pub g: u8,
	/// blue component
	pub b: u8,
}


/// # Description
/// Returns a point after correction for radial distortion
/// p is the distorted point, c is the center of distortion, and k1 through k4 are the radial distortion coefficients.
/// # Usage
/// ```
/// use img_tools::utils::{undistort_point, Point};
/// let original_point = Point { x: 1.0, y: 2.0 };
/// let center_point = Point { x: 0.0, y: 0.0 };
/// let (k1, k2, k3) = (-4.941468905314635451e-01, 2.943355950899811391e-01,  -4.134461235736135165e-02);
/// panic!();
/// ```
pub fn undistort_point(p: Point, c: Point, k1: f64, k2: f64, k3: f64) -> Point {
	let d = distance(&p, &c);
	let expr = (k1 * d) + (k2 * d * d) + (k3 * d * d * d);
	Point {
		x: p.x + (p.x - c.x)*expr,
		y: p.y + (p.y - c.y)*expr,
	}
}

// pub fn bilinear_interpolation(p: Point, a: &[], b, c, d){

// };

fn distance(p1: &Point, p2: &Point) -> f64 {
	f64::sqrt(f64::powi(p1.x - p2.x, 2) + f64::powi(p1.y - p2.y, 2))
}
