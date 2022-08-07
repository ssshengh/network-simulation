use crate::utils::point::Point;
use libm::sin;
use log::trace;
use ndarray::{s, Array, ArrayBase, Ix1, OwnedRepr};
use num::complex::Complex;
use std::f64::consts::PI;

/// 求达到角为 theta(角度) 的 steering vector, 其中到达角在 aov_2d 中做了简单的假设,
/// 没有考虑仰角, 并且假设天线之间的距离 d = lambda / 2
/// 因此二维情况下关键的参数只取决于角度和天线数量, 相比 matlab 简单了许多
fn steering_vector(theta: f64, num_antenna: usize) -> ArrayBase<OwnedRepr<Complex<f64>>, Ix1> {
    // 需要转换为弧度
    let radian = theta * PI / 180.;
    let norm_angle = 0.5 * sin(radian);

    let mut sv = Array::<Complex<f64>, Ix1>::default((num_antenna));
    sv.slice_mut(s![0]).fill(Complex::new(1., 0.));
    for n in 1..num_antenna {
        // exp(-2*pi*theta * (n-1) * i) = i * sin(2*pi*theta * (n-1))
        sv.slice_mut(s![n])
            .fill(Complex::new(0., sin(2.0 * PI * norm_angle * (n as f64))));
    }
    trace!(
        "The steering vector of theta: [{}] with n = {}, is: {}",
        theta,
        num_antenna,
        sv
    );
    sv
}

#[cfg(test)]
mod test {
    use crate::communication::mimo::steering_vector;
    use libm::{exp, sin};
    use ndarray::prelude::*;
    use num::pow::Pow;
    use num::{Complex, ToPrimitive};
    use std::f64::consts::PI;

    #[test]
    fn test_steering_vector() -> anyhow::Result<()> {
        std::env::set_var("NS_LOG", "trace");
        ns_log::init(None)?;
        let sv1 = steering_vector(90., 4);
        assert_eq!(
            sv1,
            array![
                Complex::new(1., 0.),
                Complex::new(0., 0.00000000000000012246467991473532),
                Complex::new(0., -0.00000000000000024492935982947064),
                Complex::new(0., 0.00000000000000036739403974420594),
            ]
        );

        let sv2 = steering_vector(30., 4);
        assert_eq!(
            sv2,
            array![
                Complex::new(1., 0.),
                Complex::new(0., 1.),
                Complex::new(0., 0.000000000000000566553889764798),
                Complex::new(0., -1.),
            ]
        );
        Ok(())
    }
}
