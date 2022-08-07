use libm::sqrt;
use std::f64::consts::PI;

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
    /// 求到点 other 的到达角, 是一个锐角, 返回角度, 而不是弧度！
    pub fn aov_2d(&self, other: &Point) -> f64 {
        let mut atan21 = libm::atan2(other.y - self.y, other.x - self.x) * 180. / PI;
        atan21 = atan21.abs();

        let count = 90.;
        while atan21 > 90. {
            atan21 -= count;
        }
        atan21
    }
}

#[cfg(test)]
mod test {
    use crate::utils::point::Point;

    #[test]
    fn test_norm() {
        let point = Point::init_2d(3., 4.);
        assert_eq!(point.norm_2(), 5.);
    }
    #[test]
    fn test_dot() {
        let p1 = Point::init_2d(1., 2.);
        let p2 = Point::init_2d(2., 3.);
        assert_eq!(p1.dot(&p2), 8.);
    }
    #[test]
    fn test_aov_2d() {
        // 在坐标轴的任意象限都应该是 45 度
        let p1 = Point::init_2d(0., 0.);
        let p2 = Point::init_2d(-1., 1.);
        assert_eq!(p1.aov_2d(&p2), 45.);

        let p3 = Point::init_2d(1., -1.);
        let p4 = Point::init_2d(-1., -1.);
        assert_eq!(p1.aov_2d(&p3), 45.);
        assert_eq!(p1.aov_2d(&p3), p1.aov_2d(&p4));

        // 测试非零点的情况
        let p = Point::init_2d(1., 1.);
        let p_o = Point::init_2d(2., 0.);
        assert_eq!(p.aov_2d(&p_o), 45.);
    }
}
