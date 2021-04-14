/// An untransformed rectangle.
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    /// A rectagle with an origin of zero, and a width and height of 1.
    pub const UNIT: Rect = Rect::new(0., 0., 1., 1.);

    /// Create a new rectangle from its components.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect { x, y, width, height }
    }

    /// Gets the `x` value offset by `width`.
    pub fn far_x(&self) -> f32 {
        self.x + self.width
    }

    /// Gets the `y` value offset by `height`.
    pub fn far_y(&self) -> f32 {
        self.y + self.height
    }

    /// Offsets this rectangle.
    pub fn offset(self, offset: na::Vector2<f32>) -> Rect {
        Rect {
            x: self.x + offset.x,
            y: self.y + offset.y,
            ..self
        }
    }
}
