use crate::communication::access::AccessMode::NOMA;
use crate::communication::mimo::ChannelGain;
use crate::communication::Connect;
use crate::terminal::bs::BS;
use crate::terminal::ue::UE;
use crate::utils::point::Point;
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// 考虑用户与基站连接的最大距离
const DISTANCE: f64 = 100.;

pub struct DefaultConnectImpl {}

impl Connect for DefaultConnectImpl {
    /// 假设每个基站都是一个圆形的控制范围, 所有的用户都可以被基站服务到, 那么一定会存在一些用户
    /// 处在交界区域内。那么需要对交界区域的用户进行一定的处理, 这里的策略是, 交界区域的用户随机
    /// 连接一个即可。
    ///
    /// # 注意
    /// 如果存在用户没有被基站服务到, 则可能出现意想不到的情况.
    fn connect(&self, ues: &mut BTreeMap<String, UE>, bss: &mut HashMap<String, BS>) {
        log::info!(
            "Into the connect phase, where contains nums of ue: {}, bs: {}",
            ues.len(),
            bss.len()
        );

        // 使用最简单的暴力求解策略, 如果存在性能瓶颈再在这里优化, 因为基站是个位数, 用户也只是
        // 十位数左右, 数百的计算量比较小
        // safety: 注意, 需要确保所有的用户是一定能够被服务到的
        ues.iter_mut().for_each(|ue| {
            let position = ue.1.position;
            let mut min_dis = f64::MAX;
            let mut dis_2_bs_id = HashMap::with_capacity(bss.len());

            bss.iter_mut().for_each(|bs| {
                let location_bs = bs.1.location;
                let dis = location_bs.distance_with(&position);
                // 必须小于限制范围才行
                if dis <= DISTANCE {
                    min_dis = min_dis.min(dis);
                    dis_2_bs_id.insert(dis.to_string(), bs.1);
                }
            });
            // println!("{:?}", dis_2_bs_id);

            // 只要所有的节点一定在基站服务范围内, 那么一定存在这个值
            let bs = dis_2_bs_id.get_mut(&min_dis.to_string()).unwrap();
            let channel_gain = ue.1.set_bs(bs.id.clone(), &bs.location);
            bs.connect_ue(ue.1.id.clone(), channel_gain);
        });

        log::trace!("The connected res is: {:?}", bss);
    }
}

#[cfg(test)]
mod test {
    use crate::algorithm::connect_default::DefaultConnectImpl;
    use crate::communication::access::AccessMode::NOMA;
    use crate::communication::Connect;
    use crate::terminal::bs::BS;
    use crate::terminal::ue::UE;
    use crate::utils::point::Point;
    use std::collections::{BTreeMap, HashMap};

    fn gen_test_connect_ue_bs() -> (BTreeMap<String, UE>, HashMap<String, BS>) {
        let (mut ue, mut bs) = (BTreeMap::new(), HashMap::new());
        ue.insert(
            "11".to_string(),
            UE::new("11".to_string(), Point::init_2d(50., 20.)),
        );
        ue.insert(
            "22".to_string(),
            UE::new("22".to_string(), Point::init_2d(-30., -10.)),
        );
        ue.insert(
            "33".to_string(),
            UE::new("33".to_string(), Point::init_2d(-10., 20.)),
        );

        bs.insert(
            "BS1".to_string(),
            BS::new("BS1".to_string(), Point::init_2d(-10., 10.), NOMA),
        );
        bs.insert(
            "BS2".to_string(),
            BS::new("BS2".to_string(), Point::init_2d(50., 10.), NOMA),
        );
        (ue, bs)
    }

    #[test]
    fn test_connect() {
        let c = DefaultConnectImpl {};
        let (mut ue, mut bs) = gen_test_connect_ue_bs();
        c.connect(&mut ue, &mut bs);
        // for x in bs {
        //     println!("{:?}", x.1);
        //     println!("{:?}", x.1.connected_ues());
        // }
        assert!(bs
            .get_mut("BS1")
            .unwrap()
            .connected_ues()
            .contains_key(&"22".to_string()));
        assert!(bs
            .get_mut("BS1")
            .unwrap()
            .connected_ues()
            .contains_key(&"33".to_string()));
        assert!(bs
            .get_mut("BS2")
            .unwrap()
            .connected_ues()
            .contains_key(&"11".to_string()));
    }
}
