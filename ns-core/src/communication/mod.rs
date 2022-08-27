use crate::communication::mimo::ChannelGain;
use crate::terminal::bs::BS;
use std::sync::Arc;

pub mod access;
pub(crate) mod mimo;
pub(crate) mod noma;

// pub fn sinr()
pub trait Connect {
    /// 定义连接的策略, 我们考虑从 ue 到 bs 进行通信连接的请求, 在调度侧处理该请求
    /// 如果持有了 UE 的 id, 则可以将这个 UE 注册到 BS 侧, 同时计算连接时所必须的
    /// 信道增益等信息, 同时 UE 也持有 BS 的可读访问权限, 使得可以进行上行通信。
    fn ue2bs(&self, ue_id: &str) -> (Arc<BS>, ChannelGain);
}

pub trait Transmit {
    fn up_stream();

    fn down_stream();
}
