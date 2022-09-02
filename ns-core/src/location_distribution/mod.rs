use crate::communication::access::AccessMode;
use crate::utils::point::Point;

pub trait Location {
    /// 通过一个自定义算法获取到实际上 num 个基站的分布位置
    fn setting_bss(&mut self) -> (Vec<Point>, Vec<AccessMode>);

    /// 设置 num 个 ue, 并设置其初始位置
    fn init_ues(&mut self) -> Vec<Point>;

    /// 获取到时隙的长度, ue 的移动的周期便是以 tick 为准
    fn get_tick(&self) -> f64;

    /// 更新 ue 下一个时隙所处的位置
    fn update_ues(&mut self) -> Vec<Point>;
}
