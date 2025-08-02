use std::collections::VecDeque;

use crate::{edit::Edit, levenshtein::compute_edit_sequence};

mod edit;
mod levenshtein;

pub struct MorphingString {
    current_value: String,
    target: String,
    remaining_edits: VecDeque<Edit>,
    total_edits: usize,
}

impl MorphingString {
    pub fn new(value: String) -> Self {
        Self {
            current_value: value.chars().collect(),
            target: value,
            remaining_edits: VecDeque::new(),
            total_edits: 0,
        }
    }

    pub fn set_target(&mut self, target: String) {
        self.remaining_edits = compute_edit_sequence(&self.current_value, &target);
        self.total_edits = self.remaining_edits.len();
        self.target = target;
    }

    pub fn advance(&mut self) -> Progress {
        if let Some(edit) = self.remaining_edits.pop_front() {
            self.current_value = edit.apply(&self.current_value);
        };

        self.progress()
    }

    pub fn get_value(&self) -> String {
        self.current_value.clone()
    }

    pub fn progress(&self) -> Progress {
        Progress {
            total_edits: self.total_edits,
            remaining_edits: self.remaining_edits.len(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Progress {
    pub total_edits: usize,
    pub remaining_edits: usize,
}

impl Progress {
    pub fn is_complete(&self) -> bool {
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

        assert_eq!(string.get_value(), "1234");
    }
}
