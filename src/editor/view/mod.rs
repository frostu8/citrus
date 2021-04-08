mod serde;

use std::cmp::max;
use std::rc::Rc;

use citrus_common::{
    field::{Field, PanelMut, PanelRef},
    Panel, PanelKind,
};

use na::{Matrix4, Vector2, Vector3, Vector4};

/// A view of a field.
///
/// This uses [`Rc`] for cheap cloning, and [`Rc::make_mut`] to perform
/// operations on its containing field.
#[derive(Clone, ::serde::Serialize, ::serde::Deserialize)]
pub struct EditorView {
    pub view: Matrix4<f32>,
    #[serde(with = "serde::field")]
    pub field: Rc<Field>,
    #[serde(with = "serde::panel_kind")]
    pub selected: PanelKind,
}

impl EditorView {
    pub const DEFAULT_PANEL: PanelKind = PanelKind::Neutral;

    pub const INITIAL_ZOOM: f32 = 128.;
    pub const MAX_ZOOM: f32 = Self::INITIAL_ZOOM * 2.;
    pub const MIN_ZOOM: f32 = Self::INITIAL_ZOOM / 4.;

    pub fn new_example() -> EditorView {
        const EMPTY: Panel = Panel::EMPTY;
        const HOME: Panel = Panel::new(PanelKind::Home);
        const BONUS: Panel = Panel::new(PanelKind::Bonus);
        const DRAW: Panel = Panel::new(PanelKind::Draw);
        const ENCOUNTER: Panel = Panel::new(PanelKind::Encounter);
        const DROP: Panel = Panel::new(PanelKind::Drop);

        EditorView {
            view: Matrix4::new_scaling(Self::INITIAL_ZOOM),
            field: Rc::new(
                Field::new_slice(&[
                    &[HOME,      BONUS, DRAW,      ENCOUNTER, DROP,  HOME],
                    &[DROP,      EMPTY, EMPTY,     EMPTY,     EMPTY, BONUS],
                    &[ENCOUNTER, EMPTY, EMPTY,     EMPTY,     EMPTY, DRAW],
                    &[DRAW,      EMPTY, EMPTY,     EMPTY,     EMPTY, ENCOUNTER],
                    &[BONUS,     EMPTY, EMPTY,     EMPTY,     EMPTY, DROP],
                    &[HOME,      DROP,  ENCOUNTER, DRAW,      BONUS, HOME],
                ])
            ),
            selected: Self::DEFAULT_PANEL,
        }
    }

    /// Borrows the field as mutable.
    pub fn field_mut(&mut self) -> &mut Field {
        Rc::make_mut(&mut self.field)
    }

    /// Translates the view.
    pub fn pan(&mut self, pan: Vector2<f32>) {
        self.view = self.view
            .append_translation(&Vector3::new(pan.x, pan.y, 0.));
    }

    /// Scales the view by a point.
    pub fn scale(&mut self, factor: f32, at: Vector2<f32>) {
        let at = Vector3::new(at.x, at.y, 0.);

        self.view = self.view
            .append_translation(&-at)
            .append_scaling(factor)
            .append_translation(&at);
    }

    /// Gets the scale value.
    pub fn get_scale(&self) -> Vector2<f32> {
        Vector2::new(
            (self.view.m11.powi(2) + self.view.m12.powi(2)).sqrt(),
            (self.view.m21.powi(2) + self.view.m22.powi(2)).sqrt(),
        )
    }

    /// Translates and scales the field so that it rests entirely within a
    /// bounding box.
    pub fn center(&mut self, bb: &Vector2<f32>) {
        if self.field.width() > 0 && self.field.height() > 0 {
            const MARGIN: f32 = 64.;

            let field_size = Vector2::new(
                self.field.width() as f32,
                self.field.height() as f32,
            );

            let room = bb - Vector2::new(MARGIN, MARGIN);

            let scale = room.component_div(&field_size);
            let scale = scale.x.min(scale.y);
            let scale = scale.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM);

            let field_bb = field_size * scale;

            let translate = (bb - field_bb) / 2.;

            self.view = Matrix4::new_scaling(scale)
                .append_translation(&Vector3::new(translate.x, translate.y, 0.));
        }
    }

    /// Gets a panel reference using a mouse position as a reference, scaling
    /// the field if needed.
    pub fn flex_mut(&mut self, pos: &Vector2<f32>) -> PanelMut {
        let (x, y) = self.flex(pos);

        self.field_mut().get_mut(x, y)
    }

    /// Collapses the field into the smallest bounding box it can.
    pub fn collapse(&mut self) {
        // get bounds
        let left_bound = self.field.columns_iter()
            .position(|mut x| !x.all(empty))
            .unwrap_or(0);
        let right_bound = self.field.columns_iter()
            .rposition(|mut x| !x.all(empty))
            .map(|x| x+1)
            .unwrap_or(0);
        let top_bound = self.field.rows_iter()
            .position(|mut x| !x.all(empty))
            .unwrap_or(0);
        let bottom_bound = self.field.rows_iter()
            .rposition(|mut x| !x.all(empty))
            .map(|x| x+1)
            .unwrap_or(0);

        // resize field to fit
        self.resize_field(
            (right_bound - left_bound, bottom_bound - top_bound)
                .map(|x| x as isize),
            (left_bound, top_bound)
                .map(|x| -(x as isize)),
        );
    }

    fn field_size(&self) -> (isize, isize) {
        (self.field.width(), self.field.height()).map(|x| x as isize)
    }

    fn flex(&mut self, pos: &Vector2<f32>) -> (usize, usize) {
        let pos = self.pos(pos);

        if in_bounds(&self.field, pos) {
            pos.map(|x| x as usize)
        } else {
            // flex the field
            // get an offset of the field
            let offset = pos.map(|x| max(-x, 0));

            // get resize
            let resize = pos.apply(self.field_size(), |x, s| max(x+1 - s, 0));
            let resize = resize.add(offset);

            let new_size = self.field_size().add(resize);

            // do resize
            self.resize_field(new_size, offset);

            // there should be no negative indices anymore
            pos.add(offset.map(|x| x as isize)).map(|x| x as usize)
        }
    }

    fn resize_field(&mut self, size: (isize, isize), offset: (isize, isize)) {
        debug_assert!(size.0 >= 0, "size cannot be negative");
        debug_assert!(size.1 >= 0, "size cannot be negative");

        let field = &self.field;
        let field = Field::new_iter(
            (0..size.1).map(move |y| {
                (0..size.0).map(move |x| {
                    let pos = (x,y).sub(offset);

                    if in_bounds(field, pos) {
                        let (x,y) = pos.map(|x| x as usize);
                        field.get(x, y).clone()
                    } else {
                        Panel::EMPTY
                    }
                })
            })
        );
        *Rc::make_mut(&mut self.field) = field;

        // translate field
        self.view = self.view
            .prepend_translation(&-Vector3::new(offset.0 as f32, offset.1 as f32, 0.));
    }

    fn pos(&self, pos: &Vector2<f32>) -> (isize, isize) {
        let inverse = self.view.try_inverse().unwrap() * Vector4::new(pos.x, pos.y, 1., 1.);

        (inverse.x, inverse.y).map(|x| x.floor() as isize)
    }
}

impl Default for EditorView {
    fn default() -> EditorView {
        EditorView {
            field: Rc::new(Field::new()),
            selected: EditorView::DEFAULT_PANEL,
            view: Matrix4::identity(),
        }
    }
}

impl PartialEq for EditorView {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.field, &other.field)
            & (self.selected == other.selected)
            & (self.view == other.view)
    }
}

trait TupleExt<T>:
where Self: Sized {
    fn map<F, U>(self, mapper: F) -> (U, U)
    where F: FnMut(T) -> U;

    fn apply<F, V, U>(self, other: (V, V), apply: F) -> (U, U)
    where F: FnMut(T, V) -> U;

    fn add(self, other: (T, T)) -> (T::Output, T::Output)
    where T: std::ops::Add {
        self.apply(other, std::ops::Add::add)
    }

    fn sub(self, other: (T, T)) -> (T::Output, T::Output)
    where T: std::ops::Sub {
        self.apply(other, std::ops::Sub::sub)
    }
}

impl<T> TupleExt<T> for (T, T) {
    fn map<F, U>(self, mut mapper: F) -> (U, U)
    where F: FnMut(T) -> U {
        (mapper(self.0), mapper(self.1))
    }

    fn apply<F, V, U>(self, other: (V, V), mut apply: F) -> (U, U)
    where F: FnMut(T, V) -> U {
        (apply(self.0, other.0), apply(self.1, other.1))
    }
}

fn in_bounds(field: &Field, pos: (isize, isize)) -> bool {
    if pos.0 >= 0 && pos.1 >= 0 {
        (pos.0 as usize) < field.width() && (pos.1 as usize) < field.height()
    } else {
        false
    }
}

fn empty(panel: PanelRef) -> bool {
    panel.kind == PanelKind::Empty
}
