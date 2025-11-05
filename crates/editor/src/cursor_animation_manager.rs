use gpui::{Point, Pixels};
use std::time::{Duration, Instant};

const CURSOR_ANIMATION_DURATION: Duration = Duration::from_millis(150);

#[derive(Clone, Debug, Copy)]
pub struct CursorAnimationState {
    pub from_position: Point<Pixels>,
    pub to_position: Point<Pixels>,
    pub start_time: Instant,
}

impl CursorAnimationState {
    pub fn new(from: Point<Pixels>, to: Point<Pixels>) -> Self {
        Self {
            from_position: from,
            to_position: to,
            start_time: Instant::now(),
        }
    }

    pub fn current_position(&self) -> Point<Pixels> {
        let elapsed = self.start_time.elapsed();
        if elapsed >= CURSOR_ANIMATION_DURATION {
            return self.to_position;
        }

        let progress = elapsed.as_secs_f32() / CURSOR_ANIMATION_DURATION.as_secs_f32();
        let eased_progress = ease_out_quint(progress);

        Point {
            x: self.from_position.x
                + (self.to_position.x - self.from_position.x) * eased_progress,
            y: self.from_position.y
                + (self.to_position.y - self.from_position.y) * eased_progress,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.start_time.elapsed() >= CURSOR_ANIMATION_DURATION
    }
}

fn ease_out_quint(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(5)
}
