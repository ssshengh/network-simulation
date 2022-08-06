pub type ComputingTask = Box<dyn Fn() -> ()>;

pub struct ComputingResourceBlock {
    id: usize,
    task: ComputingTask,
    is_from_bs: bool,
}

impl ComputingResourceBlock {
    pub fn create(id: usize, task: ComputingTask, is_from_bs: bool) -> ComputingResourceBlock {
        Self {
            id,
            task,
            is_from_bs,
        }
    }
}
