use gpui::{Point, Pixels, px};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use text::Anchor;

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

#[derive(Clone, Debug, Default)]
pub struct CursorAnimations {
    animations: HashMap<Anchor, CursorAnimationState>,
}

impl CursorAnimations {
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }

    pub fn update_cursor_position(
        &mut self,
        cursor_id: Anchor,
        old_position: Point<Pixels>,
        new_position: Point<Pixels>,
    ) -> bool {
        if old_position == new_position {
            self.animations.remove(&cursor_id);
            return false;
        }

        let distance = (new_position - old_position).magnitude();

        if distance < 1.0 {
            self.animations.remove(&cursor_id);
            return false;
        }

        self.animations
            .insert(cursor_id, CursorAnimationState::new(old_position, new_position));

        true
    }

    pub fn get_animated_position(
        &self,
        cursor_id: &Anchor,
        target_position: Point<Pixels>,
    ) -> Point<Pixels> {
        if let Some(animation) = self.animations.get(cursor_id) {
            animation.current_position()
        } else {
            target_position
        }
    }

    pub fn has_active_animations(&self) -> bool {
        !self.animations.is_empty()
    }

    pub fn cleanup_completed_animations(&mut self) {
        self.animations.retain(|_, animation| !animation.is_complete());
    }

    pub fn clear(&mut self) {
        self.animations.clear();
    }
}
