//! An example of using perlin noise

use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};

pub fn get_perlin_value(x: f64, y: f64, offset: f64, scale_y: f64, scale_hor: f64) -> f64 {
    let hasher = PermutationTable::new(0);
    let r_x = (x + 0.1) / scale_hor;
    let r_y = (y + 0.1) / scale_hor;
    perlin_2d([r_x, r_y], &hasher) * scale_y + offset
}
