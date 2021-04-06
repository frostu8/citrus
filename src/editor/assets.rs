use citrus_common::PanelKind;
use PanelKind::*;

use crate::enum_map::EnumMap;

pub type PanelMap<T> = EnumMap<PanelKind, T>;

// `EnumKey` implementation
// Orphan rules mean we can't generate this with a macro, so we have to
// redefine all of this. That's fine, I had to do this like this in the
// previous crate too.
use crate::enum_map::{EnumKey};

impl<T> EnumKey<T> for PanelKind {
    type Storage = [T; 22];

    fn into_usize(kind: PanelKind) -> usize {
        match kind {
            Empty => 0,
            Neutral => 1,
            Home => 2,
            Encounter => 3,
            Draw => 4,
            Bonus => 5,
            Drop => 6,
            Warp => 7,
            Draw2x => 8,
            Bonus2x => 9,
            Drop2x => 10,
            Deck => 11,
            Encounter2x => 12,
            Move => 13,
            Move2x => 14,
            WarpMove => 15,
            WarpMove2x => 16,
            Ice => 17,
            Heal => 18,
            Heal2x => 19,
            Damage => 20,
            Damage2x => 21,
        }
    }

    fn from_usize(index: usize) -> PanelKind {
        match index {
            0 => Empty,
            1 => Neutral,
            2 => Home,
            3 => Encounter,
            4 => Draw,
            5 => Bonus,
            6 => Drop,
            7 => Warp,
            8 => Draw2x,
            9 => Bonus2x,
            10 => Drop2x,
            11 => Deck,
            12 => Encounter2x,
            13 => Move,
            14 => Move2x,
            15 => WarpMove,
            16 => WarpMove2x,
            17 => Ice,
            18 => Heal,
            19 => Heal2x,
            20 => Damage,
            21 => Damage2x,
            // according to the gaurantees documented on the trait, this should
            // never be reachable
            _ => unreachable!(),
        }
    }
}
