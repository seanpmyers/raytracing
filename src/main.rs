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

pub struct PPM(Vec<Vec<RGB>>);

pub fn generate_ppm_pixels() -> PPM {
    let mut pixels: Vec<Vec<RGB>> = Vec::with_capacity(IMAGE_HEIGHT as usize);

    for row in 0..IMAGE_HEIGHT {
        let mut row_pixels = Vec::with_capacity(IMAGE_WIDTH as usize);
        for column in 0..IMAGE_WIDTH {
            let r: f32 = column as f32 / (IMAGE_WIDTH as f32 - 1f32);
            let g: f32 = row as f32 / (IMAGE_HEIGHT as f32 - 1f32);
            let b: f32 = 0f32;

            row_pixels.push(RGB {
                red: (255.99f32 * r) as u32,
                green: (255.99f32 * g) as u32,
                blue: (255.99f32 * b) as u32,
            });
        }
        pixels.push(row_pixels);
    }
    PPM(pixels)
}

pub struct RGB {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub fn to_ppm(height: u32, width: u32, pixels: PPM) -> String {
    let mut result: String = String::new();

    result.push_str(&format!("P3\n{} {}\n255\n", width, height));

    for (i, row) in pixels.0.iter().enumerate() {
        eprintln!("Lines remaining: {}", i);
        for column in row.iter() {
            result.push_str(&format!(
                "{} {} {}\n",
                column.red, column.green, column.blue
            ));
        }
    }

    eprintln!("Done.");

    result
}
