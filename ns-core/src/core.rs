use crate::communication::{Connect, Transmit};
use crate::location_distribution::Location;
use crate::terminal::bs::BS;
use crate::terminal::ue::UE;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
/// 单线程事件轮询机制的调度策略实现
///
/// # Example
/// ```
/// use ns_core::algorithm::connect_default::DefaultConnectImpl;
/// use ns_core::algorithm::movement_default::FixedVelocity;
/// use ns_core::algorithm::transmit_default::TransmitDefault;
/// use ns_core::core::Scheduler;
///
/// // 设置基站位置以及用户移动模型
/// let bs_location = vec![Point::init_2d(0., 0.), Point::init_2d(100., 100.)];
/// let ue_location = vec![
///     Point::init_2d(50., 0.),
///     Point::init_2d(-10., 10.),
///     Point::init_2d(-20., -20.),
///  ];
/// let velocity = vec![
///     Point::init_2d(10., 0.),
///     Point::init_2d(-2., 2.),
///     Point::init_2d(-4., -4.),
///  ];
/// // 移动模型定义, 采用默认的匀速直线运动
/// let mut fl = FixedVelocity::new(
///     1.2,
///     ue_location.len(),
///     bs_location.len(),
///     None,
///     velocity,
///     ue_location,
///     bs_location,
///  );
/// // 连接策略设置
/// let connect = DefaultConnectImpl {};
/// // 传输策略设置
/// let mut transmit = TransmitDefault::init();
///
/// let total_simulation_time: f64 = 100.0;
/// // 初始化调度器
/// let mut  schedule = Scheduler::init(fl, connect, transmit, total_simulation_time);
///
/// // 使用默认值, 直接跑到仿真结束
/// // schedule.run();
///
/// // 自行定义仿真的每一个时隙, 可以在每一个时隙修改连接、传输以及移动模型
/// // 每一个时隙只需要做两个阶段的事: 连接和传输
/// schedule.connecting(None);
/// schedule.communication(None);
/// // 使用 next_state 进入下一个时隙
/// schedule.next_state();
///
/// ```
pub struct Scheduler<C, T, L>
where
    C: Connect,
    T: Transmit,
    L: Location,
{
    ues: BTreeMap<String, UE>,
    bss: HashMap<String, BS>,
    // 每一次时隙的长度
    tick: f64,
    location_impl: L,
    connect_impl: C,
    transmit_impl: T,
    total_simulation_time: f64,
}

impl<C, T, L> Scheduler<C, T, L>
where
    C: Connect,
    T: Transmit,
    L: Location,
{
    pub fn init(mut location: L, connect: C, mut transmit: T, total_time: f64) -> Self {
        let (ues, bss) = (location.init_ues(), location.setting_bss());

        let mut bss_id = vec![];

        let bss = bss
            .0
            .into_iter()
            .zip(bss.1.into_iter())
            .map(|(p, a)| {
                let id = format!("BS-{:?}", p);
                bss_id.push(id.clone());
                (id.clone(), BS::new(id, p, a))
            })
            .collect::<HashMap<String, BS>>();
        let ues = ues
            .into_iter()
            .enumerate()
            .map(|p| {
                let id = format!("UE-{}", p.0);
                (id.clone(), UE::new(id, p.1))
            })
            .collect::<BTreeMap<String, UE>>();

        transmit.set_bs_id(bss_id);

        Self {
            ues,
            bss,
            tick: location.get_tick(),
            location_impl: location,
            connect_impl: connect,
            transmit_impl: transmit,
            total_simulation_time: total_time,
        }
    }

    pub fn run(&mut self) {
        let num = (self.total_simulation_time / self.tick) as usize;
        for i in 0..num {
            self.connecting(None);
            self.communication(None);
            self.next_state();
        }
    }

    pub fn connecting(&mut self, method: Option<C>) {
        if let Some(m) = method {
            self.connect_impl = m;
        }
        // 实现连接策略
        self.connect_impl.connect(&mut self.ues, &mut self.bss);
    }

    pub fn communication(&mut self, method: Option<T>) {
        if let Some(m) = method {
            self.transmit_impl = m;
        }

        // 每一个阶段中的数据传输与处理
        self.bss.iter_mut().for_each(|bs| {
            bs.1.download_multicast(&mut self.transmit_impl);
        })
    }

    /// 基于移动模型进入下一个阶段
    /// TODO: test 覆盖这个接口, 可能存在问题
    pub fn next_state(&mut self) {
        log::info!("Update ues' location from the location_impl.");
        let new_location = self.location_impl.update_ues();
        self.ues
            .iter_mut()
            .zip(new_location.into_iter())
            .for_each(|(ue, new)| {
                ue.1.position = new;
            });
    }
}
