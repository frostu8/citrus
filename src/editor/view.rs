use citrus_common::{Field, Panel, PanelKind};

use na::{Matrix4, Vector2, Vector3};

/// A view of a field.
pub struct EditorView {
    pub view: Matrix4<f32>,
    pub field: Field,
}

impl EditorView {
    pub fn new() -> EditorView {
        const EMPTY: Panel = Panel::EMPTY;
        const HOME: Panel = Panel::new(PanelKind::Home);
        const BONUS: Panel = Panel::new(PanelKind::Bonus);
        const DRAW: Panel = Panel::new(PanelKind::Draw);
        const ENCOUNTER: Panel = Panel::new(PanelKind::Encounter);
        const DROP: Panel = Panel::new(PanelKind::Drop);

        EditorView {
            view: Matrix4::new_scaling(128.),
            field: Field::new_slice(&[
                &[HOME,      BONUS, DRAW,      ENCOUNTER, DROP,  HOME],
                &[DROP,      EMPTY, EMPTY,     EMPTY,     EMPTY, BONUS],
                &[ENCOUNTER, EMPTY, EMPTY,     EMPTY,     EMPTY, DRAW],
                &[DRAW,      EMPTY, EMPTY,     EMPTY,     EMPTY, ENCOUNTER],
                &[BONUS,     EMPTY, EMPTY,     EMPTY,     EMPTY, DROP],
                &[HOME,      DROP,  ENCOUNTER, DRAW,      BONUS, HOME],
            ]),
        }
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

            let field_bb = field_size * scale;

            let translate = (bb - field_bb) / 2.;

            self.view = Matrix4::new_scaling(scale)
                .append_translation(&Vector3::new(translate.x, translate.y, 0.));
        }
    }
}
