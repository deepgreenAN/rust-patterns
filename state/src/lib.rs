mod state;

use cursive::{views::TextView, Cursive};
use std::borrow::Cow;

/// 楽曲のトラック
pub struct Track {
    pub title: String,
    pub duration: u32,
    cursor: u32,
}

impl Track {
    pub fn new<'a, S: Into<Cow<'a, str>>>(title: S, duration: u32) -> Self {
        let title: Cow<'a, str> = title.into();

        Self {
            title: title.into_owned(),
            duration,
            cursor: 0,
        }
    }
}

/// ミュージックプレーヤー
pub struct Player {
    playlist: Vec<Track>,
    current_track_number: usize,
}

impl Player {
    pub fn new(playlist: Vec<Track>) -> Self {
        Self {
            playlist,
            current_track_number: 0,
        }
    }

    /// `current_track_number`を一つ進める
    pub fn advance_track(&mut self) {
        self.current_track_number = (self.current_track_number + 1) % self.playlist.len()
    }
    /// `current_track_number`を一つ戻す
    pub fn back_track(&mut self) {
        self.current_track_number =
            (self.playlist.len() + self.current_track_number - 1) % self.playlist.len();
        // マイナスにならないように
    }
    // トラックを開始する
    pub fn play(&mut self) {
        self.current_track_mut().cursor = 10; // 擬似的にカーソルを変更
    }
    // トラックをボーズする
    pub fn pause(&mut self) {
        self.current_track_mut().cursor = 43; // 擬似的にカーソルを変更
    }
    // トラックを巻き戻す
    pub fn rewind(&mut self) {
        self.current_track_mut().cursor = 0;
    }
    // 現在のトラックの可変参照を取得
    pub fn current_track(&self) -> &Track {
        &self.playlist[self.current_track_number]
    }

    fn current_track_mut(&mut self) -> &mut Track {
        &mut self.playlist[self.current_track_number]
    }
}

/// アプリケーションのコンテキスト
pub struct PlayerApplication {
    player: Player,
    state: Box<dyn state::PlayerState>,
}

impl PlayerApplication {
    pub fn new(player: Player) -> Self {
        PlayerApplication {
            player,
            state: Box::new(state::Stopped),
        }
    }
}

pub fn on_button_click(app: &mut Cursive, button_str: &str) {
    let PlayerApplication {
        mut player,
        mut state,
    } = app.take_user_data().unwrap();

    let mut view = app.find_name::<TextView>("Player Status").unwrap();

    state = match button_str {
        "Play" => state.play(&mut player),
        "Stop" => state.stop(&mut player),
        "Prev" => state.back(&mut player),
        "Next" => state.advance(&mut player),
        _ => unreachable!(),
    };

    state.render(&player, &mut view);

    app.set_user_data(PlayerApplication { player, state });
}
