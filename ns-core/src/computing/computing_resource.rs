use crate::settings::{CORE_CAPACITY, MULTI_CORE_GAIN, NUM_CORE};

pub type ComputingTask = Box<dyn Fn() -> ()>;

/// 计算资源块的抽象, 对于每一个计算资源块来说, 同一时间只能处理一个任务, 而处理任务本身这个
/// 行为只需要通过一个函数来进行抽象即可, 接收到任务之后, 可以抽象为大概相同的计算策略去分布给
/// 多个计算模块, 然后统计时间即可, 关键在于需要给到一个具体的计算能力的抽象, 这个抽象提供了 ns 异构
/// 的基站的表达能力
pub struct ComputingResourceBlock {
    // 计算资源的多核抽象
    multi_core: usize,
    // 单核计算能力抽象
    core_capacity: f64,
    // 多核增益严格来说不是倍数关系, 用一个系数来进行抽象即可
    multi_core_gain: f64,
}

impl ComputingResourceBlock {
    pub fn new() -> Self {
        Self {
            multi_core: NUM_CORE,
            core_capacity: CORE_CAPACITY,
            multi_core_gain: MULTI_CORE_GAIN,
        }
    }

    // pub fn decode_noma(&self) -> (Vec<f64>, f64) {}
}

impl Default for ComputingResourceBlock {
    fn default() -> Self {
        ComputingResourceBlock::new()
    }
}
