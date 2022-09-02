use crate::communication::Transmit;
use crate::terminal::bs::BS;
use crate::terminal::ue::UE;
use crate::utils::packet::Packet;
use std::collections::HashMap;
use std::sync::Arc;

pub struct TransmitDefault {
    bs_id: Vec<String>,
    decode_method: HashMap<String, Box<dyn Fn(&mut BS) -> ()>>,
    transmit_method: HashMap<String, Box<dyn Fn(&mut BS) -> ()>>,
}

impl TransmitDefault {
    pub fn init() -> Self {
        Self {
            bs_id: vec![],
            decode_method: Default::default(),
            transmit_method: Default::default(),
        }
    }

    pub fn set_method(&mut self) {
        let bs_num = self.bs_id.len();

        let decode = Box::new(|bs: &mut BS| {});
    }
}

impl Transmit for TransmitDefault {
    fn set_bs_id(&mut self, id: Vec<String>) {
        self.bs_id = id;
    }

    fn decode_impl(&mut self, bs: &mut BS) {
        let method = self.decode_method.get_mut(&bs.id).unwrap();
        method(bs)
    }

    fn computing(&mut self, bs: &mut BS) {}

    fn bs_down_transmit(&mut self, bs: &mut BS) {
        let method = self.transmit_method.get_mut(&bs.id).unwrap();
        method(bs)
    }
}
