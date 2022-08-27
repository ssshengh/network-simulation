use crate::communication::access::AccessMode;
use crate::communication::Transmit;
use crate::computing::computing_resource::ComputingResourceBlock;
use crate::terminal::ue::UE;
use crate::utils::packet::Packet;
use crate::utils::point::Point;

pub struct BS {
    id: String,
    location: Point,
    computing_resource: Vec<ComputingResourceBlock>,
    connected_users: Vec<usize>,
    access_mod: AccessMode,
}

impl BS {
    /// 设置一个基站, 使用 NOMA 模式,
    pub fn set_with_noma(id: String, position: Point) {}

    pub fn set_with_fdma(id: String, position: Point) {}

    /// 下行多播的具体实现, 通过抽象出来的下行计算策略的方式进行
    pub fn download_multicast<D>(&self, download_impl: D, packets: Vec<Packet>)
    where
        D: Transmit,
    {
    }
    /// 下行广播的具体实现, 通过抽象出来的下行计算策略的方式进行
    pub fn download_broadcast<D>(&self, download_impl: D, packets: Vec<Packet>)
    where
        D: Transmit,
    {
    }
}
