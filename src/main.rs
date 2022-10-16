use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use simple_signal::{self, Signal};

use crate::fan_control::FanControl;

mod fan_control;

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));

    // When a SIGINT (Ctrl-C) or SIGTERM signal is caught, atomically set running to false.
    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let running = running.clone();
        move |_| {
            running.store(false, Ordering::SeqCst);
        }
    });

    let fan_control = FanControl::setup()?;

    // Loop
    while running.load(Ordering::SeqCst) {
        fan_control.update().expect("Could not update fan speed!");
        thread::sleep(Duration::from_secs(1))
    }

    Ok(())
}
