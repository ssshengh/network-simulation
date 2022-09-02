use crate::communication::access::AccessMode;
use crate::communication::mimo::ChannelGain;
use crate::communication::Transmit;
use crate::computing::computing_resource::ComputingResourceBlock;
use crate::terminal::ue::UE;
use crate::utils::packet::Packet;
use crate::utils::point::Point;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

pub struct BS {
    pub id: String,
    pub location: Point,
    computing_resource: ComputingResourceBlock,
    connected_users: HashMap<String, ChannelGain>,
    access_mod: AccessMode,
}

impl BS {
    pub fn new(id: String, position: Point, mode: AccessMode) -> Self {
        Self {
            id,
            location: position,
            computing_resource: ComputingResourceBlock::default(),
            connected_users: HashMap::new(),
            access_mod: mode,
        }
    }

    /// 设置一个基站, 使用 NOMA 模式,
    pub fn set_with_noma(id: String, position: Point) -> Self {
        Self {
            id,
            location: position,
            computing_resource: ComputingResourceBlock::default(),
            connected_users: HashMap::new(),
            access_mod: AccessMode::NOMA,
        }
    }

    pub fn set_with_fdma(id: String, position: Point) {}

    /// 将连接到的 UE id 存储进入 BS 中去
    pub fn connect_ue(&mut self, ue_id: String, channel_gain: ChannelGain) {
        self.connected_users.insert(ue_id, channel_gain);
    }

    /// 下行多播的具体实现, 通过抽象出来的下行计算策略的方式进行
    pub fn download_multicast<D>(&mut self, download_impl: &mut D)
    where
        D: Transmit,
    {
        download_impl.decode_impl(self);
        download_impl.bs_down_transmit(self);
    }

    /// 下行广播的具体实现, 通过抽象出来的下行计算策略的方式进行
    pub fn download_broadcast<D>(&self, download_impl: &mut D, packets: Vec<Packet>)
    where
        D: Transmit,
    {
    }

    pub fn connected_ues(&self) -> HashMap<String, ChannelGain> {
        self.connected_users.clone()
    }

    pub fn decode_packet(&mut self, packets: Vec<Packet>) {
        // self.computing_resource
    }
}

impl Debug for BS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BS: [id: {}, mode: {:?}] => connected: [{:?}]",
            self.id, self.access_mod, self.connected_users
        )
    }
}
