use std::f32::consts::E;

/// 对于数据包的抽象, 对于每一个数据包来说, 通信层面最关心的就是其大小对传输以及计算的影响
/// 一个具体的业务传输的数据应该是包括多个数据包, 因此不论业务怎么分配, 仍旧可以把数据包看
/// 为其最小组成部分。
/// 对于额外的需要考虑的情况来说, 数据包还需要关注的就是不同编解码方式带来的计算开销
///
/// # 注意
/// 1. 数据包大小单位为 MB, 默认大小为 10MB
/// 2. 计算能力的抽象为 MB/ms
/// 3. 默认的编码方式为 95% 全压缩编码
pub struct Packet {
    size: usize,
    encode_style: EncodeStyle,
}

impl Packet {
    pub fn new(encode_style: EncodeStyle, size: usize) -> Self {
        log::trace!("Create a new packet with another settings!");
        Self { size, encode_style }
    }
    /// 求编码过后的大小, 为本身计算乘以编码率即可
    pub fn encode_size(&self) -> f64 {
        log::trace!("Compute the encoded size of packet");
        let encode_ratio = match self.encode_style {
            EncodeStyle::FullCompress(ratio, _) => ratio,
            EncodeStyle::PartCompress(ratio, _) => ratio,
        };
        self.size as f64 * encode_ratio
    }
    /// 计算解码需要的时间, 受到解码能力的限制
    pub fn decode_time(&self, decode_ability: f64) -> f64 {
        log::trace!("Compute the decode time of packet");
        let coefficient = match self.encode_style {
            EncodeStyle::FullCompress(_, a) => a,
            EncodeStyle::PartCompress(_, a) => a,
        };
        self.encode_size() / (decode_ability * coefficient)
    }
}

impl Default for Packet {
    fn default() -> Self {
        log::trace!("Create a default packet!");
        Self {
            size: 10,
            encode_style: Default::default(),
        }
    }
}

pub enum EncodeStyle {
    FullCompress(f64, f64),
    PartCompress(f64, f64),
}

impl Default for EncodeStyle {
    fn default() -> Self {
        EncodeStyle::default_compress()
    }
}

impl EncodeStyle {
    /// 默认的编码压缩方式为完全压缩, 因此此时数据包的实际大小将会减小很多, 同样的解码时需要消耗
    /// 更多的时间. 这个时间大小我们看为一倍于解压计算能力.
    pub fn default_compress() -> Self {
        Self::FullCompress(0.95, 1.)
    }
    /// 其他压缩方式则可以表征为部分压缩, 抽象出了对于各种压缩率情况下的表征. 比如对于 VR 视野中
    /// 就存在牺牲部分压缩率, 用传输时间换计算时间的情况.
    pub fn part_compress(compress_ratio: f64, decode_ratio: f64) -> Self {
        Self::PartCompress(compress_ratio, decode_ratio)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::packet::{EncodeStyle, Packet};

    #[test]
    fn test_default_packet() {
        let p = Packet::default();
        assert_eq!(p.encode_size(), 9.5);
        assert_eq!(p.decode_time(10.), 0.95); // ms
    }
    #[test]
    fn test_setting_packet() {
        // 0.8 的压缩率, 1.2 倍的计算倍率加成
        let encode = EncodeStyle::PartCompress(0.8, 1.8);
        let p = Packet::new(encode, 10);

        assert_eq!(p.encode_size(), 8.);
        assert_eq!(p.decode_time(10.), 0.4444444444444444);
    }
}
