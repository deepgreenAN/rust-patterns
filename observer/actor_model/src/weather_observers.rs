use crate::weather_publisher::WeatherInfo;

use actix::prelude::*;

// -------------------------------------------------------------------------------------------------
// カスタムオブザーバー
pub struct CustomWeatherObserver {
    inner_func: Box<dyn FnMut(WeatherInfo)>,
}

impl CustomWeatherObserver {
    pub fn new<F: FnMut(WeatherInfo) + 'static>(func: F) -> Self {
        Self {
            inner_func: Box::new(func) as Box<dyn FnMut(WeatherInfo)>,
        }
    }
}

impl Actor for CustomWeatherObserver {
    type Context = Context<Self>;
}

impl Handler<WeatherInfo> for CustomWeatherObserver {
    type Result = ();
    fn handle(&mut self, msg: WeatherInfo, _ctx: &mut Self::Context) -> Self::Result {
        (self.inner_func)(msg);
    }
}

// -------------------------------------------------------------------------------------------------
// 以下は各種のオブザーバー．

pub struct DisplayJa;

impl Actor for DisplayJa {
    type Context = Context<Self>;
}

impl Handler<WeatherInfo> for DisplayJa {
    type Result = ();
    fn handle(&mut self, msg: WeatherInfo, _ctx: &mut Self::Context) -> Self::Result {
        println!(
            "現在の気象情報: 温度:{:>3.0}度, 湿度:{:>3.0}％, 気圧: {:>4.0}Hpa",
            msg.temperature, msg.humidity, msg.pressure
        );
    }
}

pub struct DisplayUsa;

impl Actor for DisplayUsa {
    type Context = Context<Self>;
}

impl Handler<WeatherInfo> for DisplayUsa {
    type Result = ();
    fn handle(&mut self, msg: WeatherInfo, _ctx: &mut Self::Context) -> Self::Result {
        println!(
            "Weather Information: Temperature: {:>3.0}℉, Humidity: {:>3.0}%, Pressure: {:>4.0}Hpa",
            msg.temperature * 1.8 + 32.0,
            msg.humidity,
            msg.pressure
        );
    }
}
