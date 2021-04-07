use citrus_common::{Field, Panel, PanelKind};

use na::Matrix4;

/// A view of a field.
pub struct EditorView {
    pub view: Matrix4<f32>,
    pub field: Field,
}

impl Default for EditorView {
    fn default() -> EditorView {
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
}
