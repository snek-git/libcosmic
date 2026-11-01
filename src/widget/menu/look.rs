//! Tones an app sets for its menus to draw with instead of the theme's components: a header slab
//! naming what was clicked, the row under the pointer in one dark, a cursor mark beside it and an
//! offset drop shadow. Without a look the menus draw as COSMIC draws them.

use std::sync::RwLock;

use iced_core::Color;

#[derive(Clone, Copy, Debug)]
pub struct Look {
    /// the menu body
    pub panel: Color,
    /// label text at rest
    pub text: Color,
    /// shortcut keys at rest
    pub keys: Color,
    /// rules between groups
    pub rule: Color,
    /// the header slab and the row under the pointer
    pub dark: Color,
    /// text on the dark
    pub light: Color,
    /// destructive labels at rest, and the fill under them when hovered
    pub urgent: Color,
    /// the offset drop shadow under the menu and the hovered row
    pub shadow: Color,
    /// the cursor mark beside the hovered row
    pub cursor: Color,
}

static LOOK: RwLock<Option<Look>> = RwLock::new(None);

pub fn set_look(look: Option<Look>) {
    *LOOK.write().unwrap() = look;
}

pub fn look() -> Option<Look> {
    *LOOK.read().unwrap()
}

/// Room left of the menu body for the cursor mark: the mark's 30px and an 8px gap.
pub const GUTTER: f32 = 38.0;
/// The drop shadow's offset, right and down.
pub const SHADOW: f32 = 2.0;
/// A row's height.
pub const ROW: f32 = 28.0;
/// The header slab's height.
pub const HEADER: f32 = 24.0;

/// How a row takes the pointer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kind {
    /// headers, rules, disabled entries: nothing lights
    #[default]
    Inert,
    /// an entry that lights in the dark
    Row,
    /// a destructive entry that lights in the urgent tone
    Urgent,
}

/// The cursor mark as one-pixel scanlines, relative to its top left: an arrowhead 22px wide and
/// 10px tall pointing right, then two 3px squares stacked behind its tip. 30 by 12 in all.
pub fn cursor_mark() -> impl Iterator<Item = (f32, f32, f32, f32)> {
    let head = (1..11).map(|y| {
        let reach = 1.0 - ((y as f32 + 0.5) - 6.0).abs() / 5.0;
        let left = 14.0 * (1.0 - reach);
        let right = 14.0 + 8.0 * reach;
        (left, y as f32, right - left, 1.0)
    });
    head.chain([(25.0, 1.0, 3.0, 3.0), (25.0, 8.0, 3.0, 3.0)])
}
