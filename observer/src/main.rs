use observer::{weather_observers, Publisher, WeatherContext};
use rand::{thread_rng, Rng};

fn main() {

    // 平均を計算するためのクロージャー．
    let mut history = Vec::<WeatherContext>::new();

    let calc_mean = move |context: &WeatherContext| {
        history.push(context.clone());

        let (temperature_sum, pressure_sum) =
            history
                .iter()
                .fold((0.0, 0.0), |(temperature_sum, pressure_sum), context| {
                    (
                        temperature_sum + context.temperature,
                        pressure_sum + context.pressure,
                    )
                });

        println!(
            "これまでの平均です．平均気温: {:>3.0}度, 平均気圧: {:>4.0}Hpa",
            temperature_sum / history.len() as f32,
            pressure_sum / history.len() as f32
        );
    };

    let mut weather_publisher = Publisher::<WeatherContext>::new();

    // Publisherにオブザーバーを追加
    weather_publisher
        .subscribe("display_ja", weather_observers::display_ja)
        .unwrap();
    weather_publisher
        .subscribe("display_usa", weather_observers::display_usa)
        .unwrap();
    weather_publisher.subscribe("calc_mean", calc_mean).unwrap();

    let get_random_context = || {
        let mut rng = thread_rng();
        WeatherContext {
            temperature: rng.gen_range(0.0..35.0),
            humidity: rng.gen_range(0.0..100.0),
            pressure: rng.gen_range(900.0..1050.0),
        }
    };

    // コンテキストを通知
    weather_publisher.notify_observers(&get_random_context());
    weather_publisher.notify_observers(&get_random_context());

    // オブザーバーをリストから削除．
    let _ = weather_publisher.unsubscribe("display_usa").unwrap();

    // コンテキストを通知
    weather_publisher.notify_observers(&get_random_context());
}
