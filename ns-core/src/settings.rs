use num::Complex;

/// 信道复增益大小
pub const COMPLEX_CHANNEL_GAIN: Complex<f64> = Complex::new(1e10 as f64, 1e10 as f64);

/// 用户天线数量设置
pub const ANTENNA_UE: usize = 2;

/// 基站天线设置
pub const ANTENNA_BS: usize = 36;

/// 基站处理器核数
pub const NUM_CORE: usize = 32;

/// 基站单核计算能力, GFLOA/s
pub const CORE_CAPACITY: f64 = 1000.;

/// 基站多核增益
pub const MULTI_CORE_GAIN: f64 = 1.3;
