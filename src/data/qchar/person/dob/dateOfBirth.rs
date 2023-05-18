
use crate::data::qtimeline::time::timestamp::TimeStamp;

enum TimeTense {
    On,
    Before,
    After
}

pub struct dateOfBirth {
    pub timestamp: TimeStamp,
    pub DMT: TimeTense,
    pub MMT: TimeTense,
    pub YMT: TimeTense,
}

// impl dateOfBirth {
//     fn new()
// }