use crate::{ray::Ray, vec3::Vec3};

// pub const IMAGE_WIDTH: u32 = 256u32;
// pub const IMAGE_HEIGHT: u32 = 256u32;
pub const ASPECT_RATIO: f32 = 16.0f32 / 9.0f32;
pub const IMAGE_WIDTH: u16 = 400u16;
pub const VIEWPORT_HEIGHT: f32 = 1.0f32;

pub mod filesystem;
pub mod ray;
pub mod vec3;

fn main() {
    // println!(
    //     "{}",
    //     to_ppm(IMAGE_HEIGHT, IMAGE_WIDTH, generate_ppm_pixels())
    // );
}

pub struct PortablePixMap(Vec<Vec<Vec3>>);

pub fn generate_ppm_pixels(width: u32, height: u32) -> PortablePixMap {
    let mut pixels: Vec<Vec<Vec3>> = Vec::with_capacity(height as usize);

    for row in 0..height {
        let mut row_pixels = Vec::with_capacity(width as usize);
        for column in 0..width {
            let r: f64 = column as f64 / (width as f64 - 1f64);
            let g: f64 = row as f64 / (height as f64 - 1f64);
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

pub fn calcualte_height_from_aspect_ratio(aspect_ratio: f32, width: f32) -> u32 {
    let height = (width / aspect_ratio) as u32;

    if height < 1u32 {
        return 1u32;
    }

    height
}

pub fn calculate_viewport_width(viewport_height: f32, image_width: f32, image_height: f32) -> f64 {
    viewport_height as f64 * (image_width as f64 / image_height as f64)
}

#[cfg(test)]
pub mod test {
    use crate::calcualte_height_from_aspect_ratio;

    const TEST_ASPECT_RATIO: f32 = 16.0 / 9.0;
    const TEST_WIDTH: u16 = 400u16;
    const TEST_WIDTH_ONE: u16 = 1u16;

    #[test]
    pub fn test_calculate_height_from_aspect_ratio() {
        let height = calcualte_height_from_aspect_ratio(TEST_ASPECT_RATIO, TEST_WIDTH as f32);
        assert_ne!(height, 1)
    }

    #[test]
    pub fn test_calculate_height_from_aspect_ratio_less_than_one() {
        let height = calcualte_height_from_aspect_ratio(TEST_ASPECT_RATIO, TEST_WIDTH_ONE as f32);
        assert_eq!(height, 1)
    }
}
