use crate::error::RrbgError;
use crate::server::server::LedValue;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref DMA: i32 = env::var("DMA")
        .unwrap_or("5".to_string())
        .parse()
        .unwrap_or(5);
    static ref LED_COUNT: i32 = env::var("LED_COUNT")
        .unwrap_or("32".to_string())
        .parse()
        .unwrap_or(32);
    static ref BRIGHTNESS: u8 = env::var("BRIGHTNESS")
        .unwrap_or("255".to_string())
        .parse()
        .unwrap_or(255);
}

pub struct ControllerInstance;

impl ControllerInstance {
    #[cfg(target_arch = "arm")]
    pub fn get_controller() -> Option<rs_ws281x::Controller> {
        let mut controller = rs_ws281x::ControllerBuilder::new()
            // default
            .freq(800_000)
            // default
            .dma(*DMA)
            .channel(
                0,
                rs_ws281x::ChannelBuilder::new()
                    .pin(18)
                    .count(*LED_COUNT)
                    .strip_type(rs_ws281x::StripType::Ws2812)
                    .brightness(*BRIGHTNESS)
                    .build(),
            )
            .build()
            .unwrap();
        Some(controller)
    }

    #[cfg(target_arch = "arm")]
    pub fn reset() {
        let mut controller = ControllerInstance::get_controller().unwrap();
        let mut leds = controller.leds_mut(0);
        for i in 0..leds.len() {
            leds[i] = [0, 0, 0, 0];
        }

        controller.render();
    }

    #[cfg(not(target_arch = "arm"))]
    pub fn reset() {
        println!("{} {} {}", *DMA, *LED_COUNT, *BRIGHTNESS);
    }

    #[cfg(target_arch = "arm")]
    pub fn set_array(values: HashMap<usize, LedValue>) -> Result<(), RrbgError> {
        let mut controller = ControllerInstance::get_controller().unwrap();
        let mut leds = controller.leds_mut(0);
        for (index, pattern) in values.into_iter() {
            let r = pattern.rgb[0];
            let g = pattern.rgb[1];
            let b = pattern.rgb[2];
            let o = pattern.rgb[3];
            eprintln!("i:{} r:{} g:{} b:{} o:{}", index, r, g, b, o);
            if index < leds.len() {
                leds[index] = [b, g, r, o];
            }
            // This sleep stops the leds being set incorrectly for some reason
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        controller.render();
        controller.wait();

        Ok(())
    }

    #[cfg(not(target_arch = "arm"))]
    pub fn set_array(_values: HashMap<usize, LedValue>) -> Result<(), RrbgError> {
        Ok(())
    }
}
