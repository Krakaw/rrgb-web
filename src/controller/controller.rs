#[cfg(target_arch = "arm")]
use rs_ws281x::Controller;
use crate::error::RrbgError;
use rustc_serialize::json::Json;

pub struct ControllerInstance;

impl ControllerInstance {

    #[cfg(target_arch = "arm")]
    pub fn get_controller() ->  Option<rs_ws281x::Controller> {
        let mut controller = rs_ws281x::ControllerBuilder::new()
            // default
            .freq(800_000)
            // default
            .dma(5)
            .channel(
                0,
                rs_ws281x::ChannelBuilder::new()
                    .pin(18)
                    .count(32)
                    .strip_type(rs_ws281x::StripType::Ws2812)
                    .brightness(255)
                    .build(),
            )
            .build().unwrap();
        Some(controller)

    }

    #[cfg(not(target_arch = "arm"))]
    pub fn get_controller() -> Option<()> {
       None
    }

    #[cfg(target_arch = "arm")]
    pub fn reset() {
        let mut controller = ControllerInstance::get_controller().unwrap();
        let mut leds = controller.leds_mut(0);
        for i in 0..32 {
            leds[i] = [0, 0, 0, 0];
        }

        controller.render();
    }


    #[cfg(not(target_arch = "arm"))]
    pub async fn reset() {}

    #[cfg(target_arch = "arm")]
    pub fn set_array(values: Json) -> Result<(), RrbgError> {

        if let Some(patterns) = values.as_array() {
            let mut controller = ControllerInstance::get_controller().unwrap();
            let mut leds = controller.leds_mut(0);
            for pattern in patterns {
                eprintln!("pattern = {:?}", pattern);
                let index = pattern.find("index").map(|r|r.as_u64().unwrap_or(0)).unwrap_or(0);
                let r = pattern.find("r").map(|r|r.as_u64().unwrap_or(0)).unwrap_or(0);
                let g = pattern.find("g").map(|g|g.as_u64().unwrap_or(0)).unwrap_or(0);
                let b = pattern.find("b").map(|b|b.as_u64().unwrap_or(0)).unwrap_or(0);
                eprintln!("{} {} {} {}", index, r, g, b);
                leds[index as usize] = [b as u8, g as u8, r as u8, 0];
                std::thread::sleep(150);
                println!("Set led");
            }
            controller.render();
            controller.wait();
        }


        Ok(())
    }


    #[cfg(not(target_arch = "arm"))]
    pub fn set_array( _values: Json) -> Result<(), RrbgError>{Ok(())}
}




