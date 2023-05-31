use super::{PlayerState, Playing, Stopped};
use crate::Player;

use cursive::views::TextView;
pub struct Paused;

impl PlayerState for Paused {
    fn play(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.pause();

        // Paused -> Playing.
        Box::new(Playing)
    }

    fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn PlayerState> {
        player.pause();
        player.rewind();

        // Paused -> Stopped.
        Box::new(Stopped)
    }

    fn render(&self, player: &Player, view: &mut TextView) {
        view.set_content(format!(
            "[Paused] {} - {} sec",
            player.current_track().title,
            player.current_track().duration
        ))
    }
}
