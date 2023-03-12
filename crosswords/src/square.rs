use crate::CrosswordsSquare;
use colors::Rgba;
use std::sync::Arc;

/// Dynamically allocated cell content.
///
/// This storage is reserved for cell attributes which are rarely set. This allows reducing the
/// allocation required ahead of time for every cell, with some additional overhead when the extra
/// storage is actually required.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct CellExtra {
    zerowidth: Vec<char>,
    // underline_color: Option<colors::Color>,

    // hyperlink: Option<Hyperlink>,
}

/// Content and attributes of a single cell in the terminal grid.
#[derive(Clone, Debug, PartialEq)]
pub struct Square {
    pub c: char,
    pub fg: Rgba,
    pub bg: Rgba,
    pub extra: Option<Arc<CellExtra>>,
}

impl CrosswordsSquare for Square {
    #[inline]
    fn is_empty(&self) -> bool {
        (self.c == ' ' || self.c == '\t')
            && self.extra.as_ref().map(|extra| extra.zerowidth.is_empty()) != Some(false)
    }

    #[inline]
    fn reset(&mut self, template: &Self) {
        *self = Square {
            bg: template.bg,
            ..Square::default()
        };
    }
}

impl Default for Square {
    #[inline]
    fn default() -> Square {
        Square {
            c: ' ',
            bg: Rgba::default(),
            fg: Rgba::default(),
            extra: None,
        }
    }
}

impl Square {
    #[inline]
    pub fn zerowidth(&self) -> Option<&[char]> {
        self.extra.as_ref().map(|extra| extra.zerowidth.as_slice())
    }

    /// Write a new zerowidth character to this cell.
    #[inline]
    pub fn push_zerowidth(&mut self, character: char) {
        let extra = self.extra.get_or_insert(Default::default());
        Arc::make_mut(extra).zerowidth.push(character);
    }
}

pub trait ResetDiscriminant<T> {
    /// Value based on which equality for the reset will be determined.
    fn discriminant(&self) -> T;
}

impl<T: Copy> ResetDiscriminant<T> for T {
    fn discriminant(&self) -> T {
        *self
    }
}

// impl ResetDiscriminant<Color> for Cell {
//     fn discriminant(&self) -> Color {
//         self.bg
//     }
// }