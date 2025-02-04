use rppal::gpio::Gpio;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

const GPIO_PWM: u8 = 26;

// Period: 10 ms (100 Hz).
const PERIOD_US: u64 = 10000;

const PULSE_MIN_US: u64 = 0;
const PULSE_MAX_US: u64 = PERIOD_US - 1000;

fn led_on(
    pin: &mut rppal::gpio::OutputPin,
    period: u64,
    pulse_width: u64,
) -> Result<(), Box<dyn Error>> {
    pin.set_pwm(
        Duration::from_micros(period),
        Duration::from_micros(pulse_width),
    )?;
    Ok(())
}

fn led_off(pin: &mut rppal::gpio::OutputPin, period: u64) -> Result<(), Box<dyn Error>> {
    pin.set_pwm(Duration::from_micros(period), Duration::from_micros(0))?;
    Ok(())
}

pub async fn blinker_led(state: bool) -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    if state {
        led_on(&mut pin, PERIOD_US, PULSE_MAX_US)?;
    } else {
        led_off(&mut pin, PERIOD_US)?;
    }
    Ok(())
}

async fn fade_in(
    pin: &mut rppal::gpio::OutputPin,
    period: u64,
    pulse_width_min: u64,
    pulse_width_max: u64,
) -> Result<(), Box<dyn Error>> {
    for pulse in (pulse_width_min..=pulse_width_max).step_by(100) {
        pin.set_pwm(Duration::from_micros(period), Duration::from_micros(pulse))?;
        sleep(Duration::from_millis(30)).await;
    }
    Ok(())
}

async fn fade_out(
    pin: &mut rppal::gpio::OutputPin,
    period: u64,
    pulse_width_min: u64,
    pulse_width_max: u64,
) -> Result<(), Box<dyn Error>> {
    for pulse in (pulse_width_min..=pulse_width_max).rev().step_by(100) {
        pin.set_pwm(Duration::from_micros(period), Duration::from_micros(pulse))?;
        sleep(Duration::from_millis(30)).await;
    }
    Ok(())
}

pub async fn lock_light() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    fade_out(&mut pin, PERIOD_US, PULSE_MIN_US, PULSE_MAX_US).await?;
    Ok(())
}

pub async fn unlock_light() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    fade_in(&mut pin, PERIOD_US, PULSE_MIN_US, PULSE_MAX_US).await?;
    Ok(())
}
