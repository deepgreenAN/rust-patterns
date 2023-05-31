use super::{PlayerState, Playing};
use crate::Player;

use cursive::views::TextView;
pub struct Stopped;

impl PlayerState for Stopped {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.play();

        // Stopped -> Playing.
        Box::new(Playing)
    }

    fn stop(self: Box<Self>, _: &mut Player) -> Box<dyn PlayerState> {
        // なにも変更しない
        self
    }

    fn render(&self, _: &Player, view: &mut TextView) {
        view.set_content("[Stopped] Press 'Play'")
    }
}
