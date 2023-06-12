use std::error::Error;

use actor_model::weather_publisher::WeatherInfo;
use rand::{thread_rng, Rng};

fn create_random_weather_info() -> WeatherInfo {
    let mut rng = thread_rng();
    WeatherInfo {
        temperature: rng.gen_range(0.0..35.0),
        humidity: rng.gen_range(0.0..100.0),
        pressure: rng.gen_range(900.0..1050.0),
    }
}
#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use actix::prelude::*;
    use actor_model::weather_observers::{CustomWeatherObserver, DisplayJa, DisplayUsa};
    use actor_model::weather_publisher::{SubscribeMsg, UnSubscribeMsg, WeatherPublisher};

    let display_ja = SubscribeMsg(DisplayJa.start().recipient());
    let display_usa = SubscribeMsg(DisplayUsa.start().recipient());

    // 平均を計算するためのカスタムオブザーバー．
    let display_mean = {
        let mut history = Vec::<WeatherInfo>::new();

        let calc_mean = move |msg: WeatherInfo| {
            history.push(msg.clone());

            let (temperature_sum, pressure_sum) =
                history
                    .iter()
                    .fold((0.0, 0.0), |(temperature_sum, pressure_sum), msg| {
                        (
                            temperature_sum + msg.temperature,
                            pressure_sum + msg.pressure,
                        )
                    });

            println!(
                "これまでの平均です．平均気温: {:>3.0}度, 平均気圧: {:>4.0}Hpa",
                temperature_sum / history.len() as f32,
                pressure_sum / history.len() as f32
            );
        };
        SubscribeMsg(CustomWeatherObserver::new(calc_mean).start().recipient())
    };

    // Publisherにオブザーバーを追加
    let publisher = WeatherPublisher::new().start();
    publisher.send(display_ja).await?;
    publisher.send(display_usa.clone()).await?;
    publisher.send(display_mean).await?;

    // WetherInfoを通知
    publisher.send(create_random_weather_info()).await?;
    publisher.send(create_random_weather_info()).await?;

    // PUblisherからオブザーバーを削除
    publisher.send(UnSubscribeMsg(display_usa.0)).await?;

    // WetherInfoを通知
    publisher.send(create_random_weather_info()).await?;

    System::current().stop();

    Ok(())
}
