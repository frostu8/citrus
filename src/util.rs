use na::{Vector2, Vector3};

/// An owned version of [`MouseEvent`].
pub struct MouseEvent {
    pos: Vector2<f32>,
    buttons: MouseButtons,
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
}

impl Default for MouseEvent {
    fn default() -> MouseEvent {
        MouseEvent {
            pos: na::zero(),
            buttons: MouseButtons(0),
        }
    }
}

impl From<web_sys::MouseEvent> for MouseEvent {
    fn from(e: web_sys::MouseEvent) -> MouseEvent {
        MouseEvent {
            pos: Vector2::new(e.offset_x() as f32, e.offset_y() as f32),
            buttons: MouseButtons(e.buttons()),
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