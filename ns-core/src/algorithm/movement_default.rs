use crate::communication::access::AccessMode;
use crate::location_distribution::Location;
use crate::utils::point::Point;

/// 固定速度大小的移动模型实现
pub struct FixedVelocity {
    tick: f64,
    num_ue: usize,
    num_bs: usize,
    velocity: Vec<Point>,
    ue_last_position: Vec<Point>,
    bs_position: Vec<Point>,
    access_mode: Vec<AccessMode>,
}

impl FixedVelocity {
    pub fn new(
        tick: f64,
        num_ue: usize,
        num_bs: usize,
        access: Option<Vec<AccessMode>>,
        velocity: Vec<Point>,
        ue: Vec<Point>,
        bs: Vec<Point>,
    ) -> Self {
        log::info!("Creating new fixed velocity movement module.");
        let access = if let Some(a) = access {
            a
        } else {
            let mut a = Vec::with_capacity(num_bs);
            (0..num_bs).for_each(|_| a.push(AccessMode::NOMA));
            a
        };
        Self {
            tick,
            num_ue,
            num_bs,
            velocity,
            ue_last_position: ue,
            bs_position: bs,
            access_mode: access,
        }
    }

    pub fn set_positions(&mut self, ue: Vec<Point>, bs: Vec<Point>) {
        log::info!("Set the ues' and bss' location into module");
        self.num_ue = ue.len();
        self.num_bs = bs.len();
        self.ue_last_position = ue;
        self.bs_position = bs;
    }

    pub fn set_velocity(&mut self, velocity: Vec<Point>) {
        log::info!("Set the velocity vector to module");
        self.velocity = velocity;
    }

    fn get_ues(&self) -> Vec<Point> {
        log::info!("Get the ues' location back.");
        self.ue_last_position.clone()
    }

    fn get_bss(&self) -> (Vec<Point>, Vec<AccessMode>) {
        log::info!("Get the bss' location and access mode back.");
        (self.bs_position.clone(), self.access_mode.clone())
    }

    fn update(&mut self) -> Vec<Point> {
        log::info!("Update the ues' location. The timestamp is: {}", self.tick);
        let mut new_position = self
            .ue_last_position
            .iter()
            .zip(self.velocity.iter())
            // 所有点都是一倍的速度匀直运动, 直接相加即可
            .map(|x| {
                let mut t_dot_v = x.1.clone();
                t_dot_v.dot_const(self.tick);

                Point::add_to_new(x.0, &t_dot_v)
            })
            .collect::<Vec<Point>>();

        self.ue_last_position = new_position.clone();
        new_position
    }
}

impl Default for FixedVelocity {
    fn default() -> Self {
        FixedVelocity::new(1., 0, 0, None, vec![], vec![], vec![])
    }
}

impl Location for FixedVelocity {
    fn setting_bss(&mut self) -> (Vec<Point>, Vec<AccessMode>) {
        self.get_bss()
    }

    fn init_ues(&mut self) -> Vec<Point> {
        self.get_ues()
    }

    fn get_tick(&self) -> f64 {
        self.tick
    }

    fn update_ues(&mut self) -> Vec<Point> {
        self.update()
    }
}

pub fn gen_test_fixed_velocity() -> FixedVelocity {
    let bs_location = vec![Point::init_2d(0., 0.), Point::init_2d(100., 100.)];
    let ue_location = vec![
        Point::init_2d(50., 0.),
        Point::init_2d(-10., 10.),
        Point::init_2d(-20., -20.),
    ];
    let velocity = vec![
        Point::init_2d(10., 0.),
        Point::init_2d(-2., 2.),
        Point::init_2d(-4., -4.),
    ];
    let mut fl = FixedVelocity::new(
        1.2,
        ue_location.len(),
        bs_location.len(),
        None,
        velocity,
        ue_location,
        bs_location,
    );

    fl
}

#[cfg(test)]
mod test {
    use crate::algorithm::movement_default::gen_test_fixed_velocity;
    use crate::location_distribution::Location;
    use crate::utils::point::Point;
    use ns_log::init;

    #[test]
    fn test() {
        init(None).unwrap();
        let mut movement = gen_test_fixed_velocity();
        assert_eq!(
            movement.update_ues(),
            vec![
                Point::init_2d(62., 0.),
                Point::init_2d(-12.4, 12.4),
                Point::init_2d(-24.8, -24.8),
            ]
        );
    }
}
