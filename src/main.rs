use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};
use simple_signal::{self, Signal};
use systemstat::{Platform, System};

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

struct FanControl {
    pwm: Pwm,
    sys: System,
    temp_range: (f32, f32),
}

impl FanControl {
    pub fn setup() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pwm: Pwm::with_frequency(Channel::Pwm0, 25000.0, 1.0, Polarity::Normal, true)?,
            sys: System::new(),
            temp_range: (30.0, 75.0),
        })
    }

    pub fn update(&self) -> Result<(), Box<dyn Error>> {
        let mut duty_cycle = 1.0;

        let (min, max) = self.temp_range;

        if let Ok(temp) = self.sys.cpu_temp() {
            let temp = temp.clamp(min, max);

            duty_cycle = (temp - min) / (max - min);
        }

        self.pwm.set_duty_cycle(duty_cycle as f64)?;

        Ok(())
    }
}
