use crate::communication::mimo::ChannelGain;
use crate::terminal::bs::BS;
use crate::terminal::ue::UE;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

pub mod access;
pub(crate) mod mimo;
pub(crate) mod noma;

// pub fn sinr()
pub trait Connect {
    /// 定义连接的策略, 我们考虑从 ue 到 bs 进行通信连接的请求, 在调度侧处理该请求
    /// 如果持有了 UE 的 id, 则可以将这个 UE 注册到 BS 侧, 同时计算连接时所必须的
    /// 信道增益等信息, 同时 UE 也持有 BS 的可读访问权限, 使得可以进行上行通信。
    fn connect(&self, ues: &mut BTreeMap<String, UE>, bss: &mut HashMap<String, BS>);
}

pub trait Transmit {
    fn set_bs_id(&mut self, id: Vec<String>);

    /// 解码阶段的实现, 对于 NOMA 来说, 信号是在这个阶段被分离的
    fn decode_impl(&mut self, bs: &mut BS);

    /// 计算阶段的实现, 对于任意数据包的计算, 都可以在这个过程中抽象为多核并行计算
    fn computing(&mut self, bs: &mut BS);

    /// 基站传输阶段, 下行信道
    fn bs_down_transmit(&mut self, bs: &mut BS);
}
