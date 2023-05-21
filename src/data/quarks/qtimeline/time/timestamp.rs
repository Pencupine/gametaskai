#[derive(Debug, PartialEq, Eq)]
pub struct TimeStamp {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl TimeStamp {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self {
            year,
            month,
            day,
        }
    }
}

// impl Hash for TimeStamp {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.year.hash(state);
//         self.month.hash(state);
//         self.day.hash(state);
//     }
// }

// fn main() {
//     let mut timeline: HashMap<TimeStamp, String> = HashMap::new();
    
// }