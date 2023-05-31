mod paused;
mod playing;
mod stopped;

pub use paused::Paused;
pub use playing::Playing;
pub use stopped::Stopped;

use crate::Player;

use cursive::views::TextView;

pub trait PlayerState {
    /// 再生ボタンが押されたときの処理．状態遷移を伴う
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState>;
    /// 停止ボタンが押されたときの処理．状態遷移を伴う．
    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState>;
    /// 状態・処理を記述する．
    fn render(&self, player: &Player, view: &mut TextView);
}

/// トレイトオブジェクトのうちオーバーライドしてほしくない実装
impl dyn PlayerState {
    /// nextボタンが押されたときの処理．状態遷移を伴わない．
    pub fn advance(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.advance_track();

        self
    }
    /// prevボタンが押されたときの処理．状態遷移は伴わない．
    pub fn back(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.back_track();

        self
    }
}
