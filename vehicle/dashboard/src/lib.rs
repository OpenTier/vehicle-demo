use log::error;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod led_pwm;

#[derive(Default)]
pub struct LedManager {
    is_running: Mutex<bool>,
}

impl LedManager {
    // Method to lock the light
    pub fn lock_light(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Lock light task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::lock_light().await {
                    error!("Error locking light: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }

    // Method to unlock the light
    pub fn unlock_light(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Unlock light task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::unlock_light().await {
                    error!("Error unlocking light: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }

    // Method for blinker LED
    pub fn blinker_led(self: &Arc<Self>, state: bool) -> Result<(), Box<dyn Error>> {
        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            let manager = Arc::clone(self);
            tokio::spawn(async move {
                let mut is_running = manager.is_running.lock().await;
                if *is_running {
                    error!("Blinker LED task is already running");
                    return;
                }
                *is_running = true; // Mark task as running

                if let Err(e) = led_pwm::blinker_led(state).await {
                    error!("Error in blinker_led: {:?}", e);
                }

                *is_running = false; // Mark task as finished
            });
            return Ok(());
        }
        #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
        {
            return Ok(());
        }
    }
}
