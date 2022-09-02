use crate::communication::mimo::{channel_gain_los, ChannelGain};
use crate::communication::{Connect, Transmit};
use crate::settings::{ANTENNA_BS, ANTENNA_UE, COMPLEX_CHANNEL_GAIN};
use crate::terminal::bs::BS;
use crate::utils::packet::Packet;
use crate::utils::point::Point;
use std::sync::{Arc, Weak};

pub struct UE {
    pub id: String,
    pub position: Point,
    pub channel_gain: Option<ChannelGain>,
    bs: Option<String>,
}

impl UE {
    pub fn new(id: String, position: Point) -> Self {
        Self {
            id,
            position,
            channel_gain: None,
            bs: None,
        }
    }
    pub fn connect<C: Connect>(&mut self, connect_impl: C) {}
    /// 数据上行, 考虑单播的具体实现
    pub fn upload<U>(&self, upload_impl: U, packets: Vec<Packet>)
    where
        U: Transmit,
    {
    }

    /// 连接到了一个基站, 保存基站的 ID, 便于传输阶段操作
    pub fn set_bs(&mut self, bs_id: String, bs_location: &Point) -> ChannelGain {
        self.bs = Some(bs_id.clone());
        let theta_t = 90.;
        let theta_r = self.position.aov_2d(bs_location);

        self.channel_gain = Some(channel_gain_los(
            COMPLEX_CHANNEL_GAIN,
            ANTENNA_BS,
            ANTENNA_UE,
            theta_t,
            theta_r,
        ));

        log::info!(
            "UE: {} is set with bs: {} and the channel gain: \n[---->]{:?}",
            self.id,
            bs_id,
            self.channel_gain,
        );

        // 此时一定有信道增益, unwrap 是安全的
        self.channel_gain.clone().unwrap()
    }

    pub fn target_bs(&self) -> Option<String> {
        self.bs.clone()
    }
}
