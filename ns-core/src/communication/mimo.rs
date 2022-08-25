//! 这个文件描述了毫米波场景下基于 MIMO 的信道增益, 核心为天线转向矩阵以及衰落方程.

use crate::utils::point::Point;
use libm::sin;
use log::{info, trace};
use ndarray::{s, Array, ArrayBase, Ix1, Ix2, OwnedRepr};
use num::complex::Complex;
use std::f64::consts::PI;

pub type ChannelGain = ArrayBase<OwnedRepr<Complex<f64>>, Ix2>;

/// 只考虑 LOS 径的信道增益, 返回一个 r*t 的矩阵
///
/// # 参数
/// -`alpha`: 天线复增益
/// -`num_antenna_t`: 发送端天线数量
/// -`num_antenna_r`: 接收端天线数量
/// -`theta_t`: 发送端到达角
/// -`theta_r`: 接收端到达角
pub fn channel_gain_los(
    alpha: Complex<f64>,
    num_antenna_t: usize,
    num_antenna_r: usize,
    theta_t: f64,
    theta_r: f64,
) -> ChannelGain {
    trace!("Computing the channel gain based only LOS");
    // 1 * m
    let sv_t = steering_vector_conj(theta_t, num_antenna_t);
    let sv_t = sv_t.into_shape((1, num_antenna_t)).unwrap();
    println!("{}", sv_t);
    // n * 1
    let sv_r = steering_vector(theta_r, num_antenna_r);
    let sv_r = sv_r.into_shape((num_antenna_r, 1)).unwrap();
    println!("{}", sv_r);
    let sv = sv_r.dot(&sv_t);

    alpha * sv
}
/// 求矩阵的 m2 范数，便于求解，同时也比较快
pub fn norm_fro2_for_channel_gain(h: ChannelGain) -> f64 {
    let size = h.shape();
    let mut res = 0.;
    h.for_each(|x| {
        res += x.norm();
    });
    libm::sqrt(res)
}

/// Friis Low 路径衰落, Pr = Pt * path_loss
///
/// # 参数
/// -`gain_t`: 天线发端增益
/// -`gain_r`: 天线收端增益
/// -`dis`: 收发端物理距离 m
/// -`wavelength`: 波长 m
pub fn friis_path_loss(gain_t: f64, gain_r: f64, dis: f64, wavelength: f64) -> f64 {
    trace!("Computing the friis low based path loss!");
    let res = gain_t * gain_r * libm::pow((wavelength / 4. / PI / dis), 2.);
    trace!("The path loss is: {}", res);
    res
}

/// 求达到角为 theta(角度) 的 steering vector, 其中到达角在 aov_2d 中做了简单的假设,
/// 没有考虑仰角, 并且假设天线之间的距离 d = lambda / 2
/// 因此二维情况下关键的参数只取决于角度和天线数量, 相比 matlab 简单了许多
///
/// # 参数
/// -`theta`: 入射角, 角度制
/// -`num_antenna`: 天线数量
fn steering_vector(theta: f64, num_antenna: usize) -> ArrayBase<OwnedRepr<Complex<f64>>, Ix1> {
    trace!("Computing the steering vector!");
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

fn steering_vector_conj(theta: f64, num_antenna: usize) -> ArrayBase<OwnedRepr<Complex<f64>>, Ix1> {
    info!("Computing the steering vector conj!");
    // 需要转换为弧度
    let radian = theta * PI / 180.;
    let norm_angle = 0.5 * sin(radian);

    let mut sv = Array::<Complex<f64>, Ix1>::default((num_antenna));
    sv.slice_mut(s![0]).fill(Complex::new(1., 0.));
    for n in 1..num_antenna {
        // exp(-2*pi*theta * (n-1) * i) = i * sin(2*pi*theta * (n-1))
        sv.slice_mut(s![n])
            .fill(Complex::new(0., sin(2.0 * PI * norm_angle * (n as f64))).conj());
    }
    trace!(
        "The steering vector(conj) of theta: [{}] with n = {}, is: {}",
        theta,
        num_antenna,
        sv
    );
    sv
}

#[cfg(test)]
mod test {
    use crate::communication::mimo::{
        channel_gain_los, friis_path_loss, norm_fro2_for_channel_gain, steering_vector, ChannelGain,
    };
    use libm::{exp, sin};
    use ndarray::prelude::*;
    use num::pow::Pow;
    use num::{Complex, ToPrimitive};
    use std::f64::consts::PI;

    #[test]
    fn test_channel_gain() {
        // 依照假设, 用户到基站的角度永远是锐角, 基站到用户的角度永远是 90 度, 那么下行情况的信道增益为:
        let alpha = Complex::new(1e10 as f64, 1e10 as f64);
        let (n, m) = (1, 2);
        let h = channel_gain_los(alpha, m, n, 90., 90.);
        let cmp = array![[
            Complex::new(10000000000., 10000000000.),
            Complex::new(0.0000012246467991473532, -0.0000012246467991473532)
        ]];
        for x in 0..h.len() {
            assert_eq!(h[[0, x]], cmp[[0, x]])
        }

        println!("h in shape({}, {}): {}", n, m, h);

        let alpha = Complex::new(1 as f64, 1 as f64);
        let (n, m) = (2, 4);
        let h = channel_gain_los(alpha, m, n, 90., 30.);
        assert_eq!("[[1+1i, 0.00000000000000012246467991473532-0.00000000000000012246467991473532i, -0.00000000000000024492935982947064+0.00000000000000024492935982947064i, 0.00000000000000036739403974420594-0.00000000000000036739403974420594i],\n [-1+1i, 0.00000000000000012246467991473532+0.00000000000000012246467991473532i, -0.00000000000000024492935982947064-0.00000000000000024492935982947064i, 0.00000000000000036739403974420594+0.00000000000000036739403974420594i]]", 
                   format!("{}", h));
        println!("h in shape({}, {}): {}", n, m, h);
    }
    #[test]
    fn test_norm2_channel_gain() {
        let h: ChannelGain = array![
            [
                Complex::new(1., 2.),
                Complex::new(1., 2.),
                Complex::new(0., 1.)
            ],
            [
                Complex::new(2., 1.),
                Complex::new(2., 1.),
                Complex::new(1., 1.)
            ]
        ];

        let norm_fro = norm_fro2_for_channel_gain(h);
        assert_eq!(3.3702352250803287, norm_fro);
    }

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
    #[test]
    fn test_friis_pl() {
        let start = chrono::Local::now().timestamp_micros();
        // 26GHz下
        let hz = 26e9;
        let c = 3e8;

        // 波长, 可以看到增益应该在 10 次方级别
        let wavelength = c / hz;
        assert_eq!(
            0.00000000008430941686215238,
            friis_path_loss(1., 1., 100., wavelength)
        );

        // 正比于波长的平方分之一得到的结果也只是勉强可以接受, 还是有丢失
        let gain = libm::pow(wavelength, -2.) * 10.;
        assert_eq!(
            0.4756466676543079,
            friis_path_loss(gain, gain, 100., wavelength)
        );
        let end = chrono::Local::now().timestamp_micros();
        // println!("计算时间 {} 微秒", end - start);
    }
}
