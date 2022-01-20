#[macro_use]
extern crate nickel;

use nickel::{HttpRouter, JsonBody, Nickel, StaticFilesHandler};
use serde::{Deserialize, Serialize};

extern crate rust_gpiozero;
use rust_gpiozero::*;

#[derive(Serialize, Deserialize, Clone)]
struct Gpio {
    pin: u64,
    value: bool,
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("./static"));

    server.post(
        "/gpio",
        middleware! { |request, response|
            let gpio = request.json_as::<Gpio>().unwrap();
            let mut led = LED::new(gpio.pin);
            if gpio.value{
                led.on();
            }else{
                led.off();
            }
            format!("{}", "ok")
        },
    );

    server.get(
        "/gpio/:pin",
        middleware! { |request|
            let number = request.param("pin").unwrap().parse::<u64>().unwrap();
            let gpio = GPIODevice::new(number);
            format!("{}", gpio.value())
        },
    );

    server.get(
        "/",
        middleware! {
            format!("hello {}", "world")
        },
    );

    server.listen("0.0.0.0:7789").unwrap();
}
