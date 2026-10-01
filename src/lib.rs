//! Morph one string into another, one edit at a time.
//!
//! [`MorphingString`] computes a minimal sequence of single-character edits
//! that turn one string into another, and applies them one per call to
//! [`MorphingString::advance`]. Driving it from a render loop produces a
//! text-morphing effect.
//!
//! ```
//! use morphing_string::MorphingString;
//!
//! let mut string = MorphingString::new("kitten");
//! string.set_target("mittens");
//!
//! while !string.advance().is_complete() {
//!     println!("{}", string.value());
//! }
//!
//! assert_eq!(string.value(), "mittens");
//! ```
//!
//! Each call to [`MorphingString::advance`] does an amount of work independent
//! of how different the two strings are, so the morph can be advanced on any
//! cadence the caller likes. Advancing is a no-op once the morph is complete.
//!
//! # Example
//!
//! A terminal demo that morphs between lines of a poem:
//!
//! ```sh
//! cargo run --example tui_poem
//! ```
//!
//! # License
//!
//! Licensed under the fuck around and find out license v0.1. It is not an
//! OSI-approved or SPDX-identified license, so this crate carries no license
//! badge on crates.io. See the `LICENSE` file for the full text.

use std::collections::VecDeque;

use crate::{edit::Edit, levenshtein::compute_edit_sequence};

mod edit;
mod levenshtein;

/// A string that morphs into a target string one edit at a time.
///
/// Created with an initial value, pointed at a target with
/// [`set_target`](Self::set_target), then advanced with
/// [`advance`](Self::advance) until [`Progress::is_complete`] returns `true`.
pub struct MorphingString {
    current_value: String,
    remaining_edits: VecDeque<Edit>,
    total_edits: usize,
}

impl MorphingString {
    /// Creates a `MorphingString` with an initial value and no target set.
    ///
    /// A fresh `MorphingString` has no edits queued, so its
    /// [`Progress`] reports completion until a target is set.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            current_value: value.into(),
            remaining_edits: VecDeque::new(),
            total_edits: 0,
        }
    }

    /// Sets the string to morph into, recomputing the sequence of edits.
    ///
    /// The sequence is computed from the *current* value, so calling this
    /// part-way through a morph abandons any edits that had not yet been
    /// applied and morphs from wherever the string is now.
    pub fn set_target(&mut self, target: impl Into<String>) {
        self.remaining_edits = compute_edit_sequence(&self.current_value, &target.into());
        self.total_edits = self.remaining_edits.len();
    }

    /// Applies the next edit, if there is one, and returns the current
    /// [`Progress`].
    ///
    /// Does nothing once the morph is complete, so it is safe to call
    /// indefinitely.
    pub fn advance(&mut self) -> Progress {
        if let Some(edit) = self.remaining_edits.pop_front() {
            self.current_value = edit.apply(&self.current_value);
        };

        self.progress()
    }

    /// The current value of the string.
    ///
    /// This is the starting value until it changes mid-morph, and equals the
    /// target once the morph is complete.
    pub fn value(&self) -> &str {
        &self.current_value
    }

    /// The current [`Progress`] of the morph, without advancing it.
    pub fn progress(&self) -> Progress {
        Progress {
            total_edits: self.total_edits,
            remaining_edits: self.remaining_edits.len(),
        }
    }
}

/// How far along a morph is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Progress {
    /// The number of edits in the sequence computed by the last
    /// [`set_target`](MorphingString::set_target) call.
    pub total_edits: usize,

    /// How many of those edits are yet to be applied to reach the target.
    pub remaining_edits: usize,
}

impl Progress {
    /// Whether every remaining edit has been applied.
    ///
    /// This is `true` for a `MorphingString` that has never had a target set.
    pub fn is_complete(self) -> bool {
        self.remaining_edits == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut string = MorphingString::new("abcd".to_string());
        string.set_target("1234".to_string());

        while !string.progress().is_complete() {
            string.advance();
        }

        assert_eq!(string.value(), "1234");
    }

    #[test]
    fn a_fresh_string_is_complete() {
        let string = MorphingString::new("abcd");

        assert!(string.progress().is_complete());
    }

    #[test]
    fn advancing_past_completion_is_a_no_op() {
        let mut string = MorphingString::new("abcd");
        string.set_target("abcd");

        let complete = Progress {
            total_edits: 0,
            remaining_edits: 0,
        };
        for _ in 0..3 {
            assert_eq!(string.advance(), complete);
            assert_eq!(string.value(), "abcd");
        }
    }

    #[test]
    fn progress_counts_down_to_zero() {
        let mut string = MorphingString::new("kitten");
        string.set_target("mittens");

        let total_edits = 2;
        assert_eq!(string.progress().total_edits, total_edits);
        assert_eq!(string.progress().remaining_edits, 2);

        assert_eq!(string.advance().remaining_edits, 1);
        assert_eq!(string.advance().remaining_edits, 0);
        assert!(string.progress().is_complete());
        assert_eq!(string.progress().total_edits, total_edits);
    }

    #[test]
    fn set_target_part_way_through_restarts_from_the_current_value() {
        let mut string = MorphingString::new("kitten");
        string.set_target("mittens");
        string.advance();
        assert_eq!(string.value(), "mitten");

        // Re-aiming mid-morph abandons the queued edits rather than applying
        // them, and morphs from the current value to the new target.
        string.set_target("sitting");
        while !string.advance().is_complete() {}

        assert_eq!(string.value(), "sitting");
    }
}
