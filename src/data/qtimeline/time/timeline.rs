use std::collections::HashMap;
use std::hash::{Hash, Hasher};
// ------------------------------------------------------------------

use crate::data::qtimeline::time::timestamp::TimeStamp;

impl Hash for TimeStamp {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.year.hash(state);
        self.month.hash(state);
        self.day.hash(state);
    }
}

pub struct TimeLine {
    events: HashMap<TimeStamp, String>,
    // eventCount: u32
}

#[allow(dead_code)]
impl TimeLine {
    pub fn new() -> Self{
        Self{
            events: HashMap::new()
            // eventCount: 0
        }
    }

    pub fn add_event(&mut self, timestamp: TimeStamp, event: String) {
        self.events.insert(timestamp, event);
    }

    pub fn get_event(&self, timestamp: &TimeStamp) -> Option<&String> {
        self.events.get(timestamp)
    }
    
    pub fn remove_event(&mut self, timestamp: &TimeStamp) -> Option<String> {
        self.events.remove(timestamp)
    }
}
