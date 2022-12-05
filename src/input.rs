use sdl2::event::Event;
pub use sdl2::{keyboard::Keycode as KeyCode, mouse::MouseButton};
use std::{collections::HashSet, hash::Hash};

struct InputState<T> {
    pressed: HashSet<T>,
    just_pressed: HashSet<T>,
    just_released: HashSet<T>,
}

impl<T: Eq + Hash> Default for InputState<T> {
    fn default() -> Self {
        Self {
            pressed: Default::default(),
            just_pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

impl<T: Copy + Eq + Hash> InputState<T> {
    fn press(&mut self, code: T) {
        self.pressed.insert(code);
        self.just_pressed.insert(code);
    }

    fn release(&mut self, code: T) {
        self.pressed.remove(&code);
        self.just_released.insert(code);
    }

    fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

#[derive(Default)]
pub struct InputContext {
    key_state: InputState<KeyCode>,
    mouse_state: InputState<MouseButton>,
    mouse_position: glam::Vec2,
    mouse_offset: glam::Vec2,
}

impl InputContext {
    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        self.key_state.pressed.contains(&key_code)
    }

    pub fn key_just_pressed(&self, key_code: KeyCode) -> bool {
        self.key_state.just_pressed.contains(&key_code)
    }

    pub fn key_just_released(&self, key_code: KeyCode) -> bool {
        self.key_state.just_released.contains(&key_code)
    }

    pub fn key_direction(
        &self,
        left: KeyCode,
        right: KeyCode,
        up: KeyCode,
        down: KeyCode,
    ) -> glam::Vec2 {
        glam::ivec2(
            self.key_pressed(right) as i32 - self.key_pressed(left) as i32,
            self.key_pressed(down) as i32 - self.key_pressed(up) as i32,
        )
        .as_vec2()
    }

    pub fn mouse_pressed(&self, mouse_code: MouseButton) -> bool {
        self.mouse_state.pressed.contains(&mouse_code)
    }

    pub fn mouse_just_pressed(&self, mouse_code: MouseButton) -> bool {
        self.mouse_state.just_pressed.contains(&mouse_code)
    }

    pub fn mouse_just_released(&self, mouse_code: MouseButton) -> bool {
        self.mouse_state.just_released.contains(&mouse_code)
    }

    pub fn mouse_position(&self) -> glam::Vec2 {
        self.mouse_position
    }

    pub fn mouse_offset(&self) -> glam::Vec2 {
        self.mouse_offset
    }

    /// Update an internal state with an SDL event
    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => self.key_state.press(*keycode),
            Event::KeyUp {
                keycode: Some(keycode),
                ..
            } => self.key_state.release(*keycode),
            Event::MouseButtonUp { mouse_btn, .. } => self.mouse_state.press(*mouse_btn),
            Event::MouseButtonDown { mouse_btn, .. } => self.mouse_state.release(*mouse_btn),
            Event::MouseMotion {
                xrel, yrel, x, y, ..
            } => {
                self.mouse_offset = glam::vec2(*xrel as f32, *yrel as f32);
                self.mouse_position = glam::vec2(*x as f32, *y as f32);
            }
            _ => (),
        }
    }

    /// Clear one frame only interal state from previous frame
    pub fn update(&mut self) {
        self.mouse_state.update();
        self.key_state.update();
        self.mouse_offset = glam::Vec2::ZERO;
    }
}
