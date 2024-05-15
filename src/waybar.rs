use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use serde::Serialize;

/// waybar custom
/// https://github.com/Alexays/Waybar/wiki/Module:-Custom
#[derive(Debug, Clone, Serialize)]
pub struct WaybarData {
    pub class: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<usize>,
}

pub fn loop_stdout(data: WaybarData, sleep: Duration) {
    let re = serde_json::to_string(&data).unwrap();
    loop {
        println!("{}", re);
        io::stdout().flush().unwrap();
        thread::sleep(sleep);
    }
}
