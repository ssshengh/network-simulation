use libm::sqrt;

#[derive(Default, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn init_2d(x: f64, y: f64) -> Self {
        Self { x, y, z: 0.0 }
    }

    pub fn init_3d(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn norm_2(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
