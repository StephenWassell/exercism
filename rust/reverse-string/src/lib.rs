#[cfg(feature = "grapheme")]
extern crate unicode_segmentation;

#[cfg(feature = "grapheme")]
use crate::unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    #[cfg(feature = "grapheme")]
    let chars = input.graphemes(true);
    #[cfg(not(feature = "grapheme"))]
    let chars = input.chars();

    chars.rev().collect()
}
