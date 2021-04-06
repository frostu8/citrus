use std::cell::Cell;
use std::rc::Rc;

use crate::enum_map::EnumMap;
use citrus_common::PanelKind;

#[test]
pub fn test_enum_map() {
    // test allocation
    let mut panel_map = EnumMap::<PanelKind, String>::new(|kind| {
        match kind {
            PanelKind::Bonus => String::from("awesome"),
            _ => String::new(),
        }
    });

    assert_eq!(panel_map[PanelKind::Bonus], "awesome");
    assert_ne!(panel_map[PanelKind::Drop], "awesome");

    // test saving to field
    panel_map[PanelKind::Draw] = String::from("pretty awesome");

    assert_eq!(panel_map[PanelKind::Bonus], "awesome");
    assert_ne!(panel_map[PanelKind::Drop], "awesome");
    assert_eq!(panel_map[PanelKind::Draw], "pretty awesome");
}

#[test]
pub fn test_enum_map_safety() {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    // we use a special type to tell us if the the item was successfully
    // dropped
    struct DropTest<'a>(&'a Cell<usize>);

    impl<'a> Drop for DropTest<'a> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    let drop_count = Cell::new(0);

    let result = catch_unwind(AssertUnwindSafe(|| {
        EnumMap::<PanelKind, DropTest>::new(|kind| {
            match kind {
                PanelKind::Drop2x => panic!("should be removed imo"),
                _ => DropTest(&drop_count),
            }
        });
    }));
    
    assert!(result.is_err());
    assert!(drop_count.get() > 0);
}
