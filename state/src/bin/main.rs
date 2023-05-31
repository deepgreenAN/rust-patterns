fn main() {
    use state::{on_button_click, Player, PlayerApplication, Track};

    use cursive::{
        view::Nameable,
        views::{Dialog, TextView},
    };

    let mut app = cursive::crossterm();

    let player = Player::new(vec![
        Track::new("Track 1", 180),
        Track::new("Track 2", 165),
        Track::new("Track 3", 197),
        Track::new("Track 4", 205),
    ]);

    let app_ctx = PlayerApplication::new(player);

    app.set_user_data(app_ctx);

    app.add_layer(
        Dialog::around(TextView::new("Press Play").with_name("Player Status"))
            .title("Music Player")
            .button("Play", |s| on_button_click(s, "Play"))
            .button("Stop", |s| on_button_click(s, "Stop"))
            .button("Prev", |s| on_button_click(s, "Prev"))
            .button("Next", |s| on_button_click(s, "Next"))
            .button("Quit", |s| s.quit()),
    );

    app.run();
}
