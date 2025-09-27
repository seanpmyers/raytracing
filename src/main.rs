use crate::vec3::Vec3;

pub const IMAGE_WIDTH: u32 = 256u32;
pub const IMAGE_HEIGHT: u32 = 256u32;

pub mod vec3;
pub mod filesystem {}

fn main() {
    println!(
        "{}",
        to_ppm(IMAGE_HEIGHT, IMAGE_WIDTH, generate_ppm_pixels())
    );
}

pub struct PortablePixMap(Vec<Vec<Vec3>>);

pub fn generate_ppm_pixels() -> PortablePixMap {
    let mut pixels: Vec<Vec<Vec3>> = Vec::with_capacity(IMAGE_HEIGHT as usize);

    for row in 0..IMAGE_HEIGHT {
        let mut row_pixels = Vec::with_capacity(IMAGE_WIDTH as usize);
        for column in 0..IMAGE_WIDTH {
            let r: f64 = column as f64 / (IMAGE_WIDTH as f64 - 1f64);
            let g: f64 = row as f64 / (IMAGE_HEIGHT as f64 - 1f64);
            let b: f64 = 0f64;

            row_pixels.push(Vec3::new(255.99f64 * r, 255.99f64 * g, 255.99f64 * b));
        }
        pixels.push(row_pixels);
    }
    PortablePixMap(pixels)
}

pub fn to_ppm(height: u32, width: u32, pixels: PortablePixMap) -> String {
    let mut result: String = String::new();

    result.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for (i, row) in pixels.0.iter().enumerate() {
        eprintln!("Lines remaining: {}", i);
        for column in row.iter() {
            result.push_str(&column.to_ppm_line());
        }
    }

    eprintln!("Done.");

    result
}
