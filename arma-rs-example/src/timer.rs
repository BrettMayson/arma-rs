use std::{thread, time::Duration};

use arma_rs::Group;

use crate::arma_callback;

pub fn sleep(duration: u64, id: String) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(duration));
        arma_callback("example_timer", "done", Some(id));
    });
}

pub fn group() -> Group {
    Group::new("timer").command("sleep", sleep)
}
