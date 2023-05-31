use super::{Paused, PlayerState, Stopped};
use crate::Player;

use cursive::views::TextView;

pub struct Playing;

impl PlayerState for Playing {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.pause();

        // Playing -> Paused.
        Box::new(Paused)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.pause();
        player.rewind();

        // Playing -> Stopped.
        Box::new(Stopped)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Playing] {} - {} sec",
            player.current_track().title,
            player.current_track().duration
        ))
    }
}
