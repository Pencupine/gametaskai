// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
// use std::fs::{File, OpenOptions};
// use std::io::{BufReader, BufWriter};
// use std::path::Path;

// use crate::data::quarks::qtimeline::time::timestamp::TimeStamp;
// use crate::data::quarks::qtimeline::time::timeline::TimeLine;

use crate::services::electron::runner::run_electron;
use crate::services::utils::configloader::{load_config};

use crate::services::gametask::engine::maineng::start_gtai_server;


pub fn master() {

    let config = load_config();
    println!("{}", config.openai.account.api_key.ApiKey);

    run_electron(config.electron);

    println!("MASTER:Starting server");
    start_gtai_server();

    // let mut timeline = TimeLine::new();

    // let timestamp1 = TimeStamp::new(2023, 5, 15);
    // let timestamp2= TimeStamp::new(2023, 5, 28);
    // timeline.add_event(timestamp1, "Zero is born".to_string());
    // timeline.add_event(timestamp2, "Zero is successfull".to_string());

    // let lookup_timestamp = TimeStamp::new(2023, 5, 28);
    // if let Some(event) = timeline.get_event(&lookup_timestamp) {
    //     println!("Event at custom timestamp: {}", event);
    // }
}