use citrus_common::PanelKind;
use PanelKind::*;

use crate::enum_map::EnumMap;

pub type PanelMap<T> = EnumMap<PanelKind, T>;

macro_rules! match_img {
    { $init:ident, $( $pattern:pat => $lit:literal ),* } => {
        match $init {
            $( $pattern => concat!("./img/", $lit), )*
            _ => return None,
        }
    }
}

/// Gets the image sources of a panel kind.
///
/// Returns `None` if a panel does not have an image representation.
pub fn panel_source(kind: PanelKind) -> Option<&'static str> {
    Some(
        match_img! {
            kind,
            Neutral => "neutral.png",
            Home => "home.png",
            Encounter => "encounter.png",
            Draw => "draw.png",
            Bonus => "bonus.png",
            Drop => "drop.png",
            Warp => "warp.png",
            Draw2x => "draw2x.png",
            Bonus2x => "bonus2x.png",
            Drop2x => "drop2x.png",
            Encounter2x => "encounter2x.png",
            Move => "move.png",
            Move2x => "move2x.png",
            WarpMove => "warpmove.png",
            WarpMove2x => "warpmove2x.png",
            Ice => "ice.png",
            Heal => "heal.png",
            Heal2x => "heal2x.png",
            Damage => "damage.png",
            Damage2x => "damage2x.png"
        }
    )
}

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
            Deck => 1,
            Neutral => 2,
            Home => 3,
            Draw => 4,
            Draw2x => 5,
            Bonus => 6,
            Bonus2x => 7,
            Drop => 8,
            Drop2x => 9,
            Encounter => 10,
            Encounter2x => 11,
            Move => 12,
            Move2x => 13,
            Warp => 14,
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
            1 => Deck,
            2 => Neutral,
            3 => Home,
            4 => Draw,
            5 => Draw2x,
            6 => Bonus,
            7 => Bonus2x,
            8 => Drop,
            9 => Drop2x,
            10 => Encounter,
            11 => Encounter2x,
            12 => Move,
            13 => Move2x,
            14 => Warp,
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
