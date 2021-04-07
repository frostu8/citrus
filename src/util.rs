use na::{Vector2, Vector3};

use std::ops::Deref;

/// An owned version of [`MouseEvent`].
pub struct MouseEvent {
    pos: Vector2<f32>,
    buttons: MouseButtons,
    button: MouseButtons,
    modifiers: MouseModifiers,
}

impl MouseEvent {
    pub fn pos(&self) -> Vector2<f32> {
        self.pos
    }

    pub fn pos3(&self) -> Vector3<f32> {
        Vector3::new(self.pos.x, self.pos.y, 1.0)
    }

    pub fn buttons(&self) -> MouseButtons {
        self.buttons
    }

    pub fn button(&self) -> MouseButtons {
        self.button
    }

    pub fn modifiers(&self) -> MouseModifiers {
        self.modifiers
    }
}

impl Default for MouseEvent {
    fn default() -> MouseEvent {
        MouseEvent {
            pos: na::zero(),
            buttons: MouseButtons(0),
            button: MouseButtons(0),
            modifiers: MouseModifiers(0),
        }
    }
}

impl From<&web_sys::MouseEvent> for MouseEvent {
    fn from(e: &web_sys::MouseEvent) -> MouseEvent {
        MouseEvent {
            pos: Vector2::new(e.offset_x() as f32, e.offset_y() as f32),
            buttons: MouseButtons(e.buttons()),
            button: MouseButtons(1 << e.button()),
            modifiers: e.into(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct MouseButtons(u16);

impl MouseButtons {
    /// Gets the state of the primary mouse button
    pub fn left(&self) -> bool {
        self.0 & 1 > 0
    }

    /// Gets the state of the secondary mouse button
    pub fn right(&self) -> bool {
        self.0 & 2 > 0
    }
}

#[derive(Clone, Copy)]
pub struct MouseModifiers(u8);

impl MouseModifiers {
    const SHIFT: MouseModifiers = MouseModifiers(1);

    pub fn shift(&self) -> bool {
        self.0 & Self::SHIFT.0 > 0
    }
}

impl From<&web_sys::MouseEvent> for MouseModifiers {
    fn from(e: &web_sys::MouseEvent) -> MouseModifiers {
        MouseModifiers(
            boolean(e.shift_key()) & Self::SHIFT.0
        )
    }
}

/// An owned version of [`WheelEvent`].
pub struct WheelEvent {
    sup: MouseEvent,
    delta_y: f32,
}

impl WheelEvent {
    pub fn delta_y(&self) -> f32 {
        self.delta_y
    }
}

impl Deref for WheelEvent {
    type Target = MouseEvent;

    fn deref(&self) -> &MouseEvent {
        &self.sup
    }
}

impl Default for WheelEvent {
    fn default() -> WheelEvent {
        WheelEvent {
            sup: MouseEvent::default(),
            delta_y: 0.0,
        }
    }
}

impl From<&web_sys::WheelEvent> for WheelEvent {
    fn from(e: &web_sys::WheelEvent) -> WheelEvent {
        WheelEvent {
            sup: e.deref().into(),
            delta_y: e.delta_y() as f32,
        }
    }
}

fn boolean(b: bool) -> u8 {
    if b { u8::MAX } else { u8::MIN }
}

