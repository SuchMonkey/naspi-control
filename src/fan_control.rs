use std::error::Error;

use rppal::pwm::{Channel, Polarity, Pwm};
use systemstat::{Platform, System};

pub struct FanControl {
    pwm: Pwm,
    sys: System,
    temp_range: (f32, f32),
}

impl FanControl {
    pub fn setup() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pwm: Pwm::with_frequency(Channel::Pwm0, 25000.0, 1.0, Polarity::Normal, true)?,
            sys: System::new(),
            temp_range: (30.0, 70.0),
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
