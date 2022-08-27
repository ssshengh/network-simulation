use crate::communication::mimo::ChannelGain;
use crate::communication::{Connect, Transmit};
use crate::terminal::bs::BS;
use crate::utils::packet::Packet;
use crate::utils::point::Point;
use std::sync::Arc;

pub struct UE {
    id: String,
    position: Point,
    channel_gain: Option<ChannelGain>,
    bs: Option<Arc<BS>>,
}

impl UE {
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: Default::default(),
            channel_gain: None,
            bs: None,
        }
    }
    pub fn connect<C: Connect>(&mut self, connect_impl: C) {
        let connected = connect_impl.ue2bs(self.id.as_str());
        self.channel_gain = Some(connected.1.clone());
        self.bs = Some(connected.0.clone());
    }
    /// 数据上行, 考虑单播的具体实现
    pub fn upload<U>(&self, upload_impl: U, packets: Vec<Packet>)
    where
        U: Transmit,
    {
    }
}
