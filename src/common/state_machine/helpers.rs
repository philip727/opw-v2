use std::time::Duration;

use bevy::prelude::*;

pub trait State {
    fn timer(&self) -> &Timer;
    fn mut_timer(&mut self) -> &mut Timer;
    fn transition_to(&self) -> Option<String>;
    fn r#loop(&self) -> bool;

    /// If timer finishes, this returns true
    fn tick(&mut self, tick_by: Duration) -> bool {
        self.mut_timer().tick(tick_by);

        if self.timer().just_finished() {
            return true;
        }

        false
    }
}
