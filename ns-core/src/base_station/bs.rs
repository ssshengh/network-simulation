use crate::communication::access::AccessMode;
use crate::computing::computing_resource::ComputingResourceBlock;
use crate::utils::point::Point;

struct BS {
    id: usize,
    location: Point,
    computing_resource: Vec<ComputingResourceBlock>,
    connected_users: Vec<usize>,
    access_mod: AccessMode,
}
