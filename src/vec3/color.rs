use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_string(&self) -> String {
        // Translate the [0,1] component values to the byte range [0,255].
        let r_byte = (255.99f64 * self.x()) as i32;
        let b_byte = (255.99f64 * self.y()) as i32;
        let g_byte = (255.99f64 * self.z()) as i32;

        format!("{} {} {}\n", r_byte, b_byte, g_byte)
    }

    pub fn new_zero() -> Color {
        Self {
            e: [0f64, 0f64, 0f64],
        }
    }
}
