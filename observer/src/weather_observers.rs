#[derive(Clone)]
pub struct WeatherContext {
    pub temperature: f32,
    pub humidity: f32,
    pub pressure: f32,
}

pub fn display_ja(context: &WeatherContext) {
    println!(
        "現在の気象情報: 温度:{:>3.0}度, 湿度:{:>3.0}％, 気圧: {:>4.0}Hpa",
        context.temperature, context.humidity, context.pressure
    );
}

pub fn display_usa(context: &WeatherContext) {
    println!(
        "Weather Information: Temperature: {:>3.0}℉, Humidity: {:>3.0}%, Pressure: {:>4.0}Hpa",
        context.temperature * 1.8 + 32.0,
        context.humidity,
        context.pressure
    );
}
