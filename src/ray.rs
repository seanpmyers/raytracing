use crate::vec3::{Vec3, color::Color, point3::Point3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    Color::new(e0, e1, e2)
}
