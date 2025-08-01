#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Edit {
    Insert { c: char, index: usize },
    Delete { index: usize },
    Substitute { c: char, index: usize },
}

impl Edit {
    pub fn apply(&self, string: &str) -> String {
        let mut chars: Vec<char> = string.chars().collect();

        match self {
            Edit::Insert { c, index } => {
                chars.insert(*index, *c);
            }
            Edit::Delete { index } => {
                chars.remove(*index);
            }
            Edit::Substitute { c, index } => {
                chars[*index] = *c;
            }
        }

        String::from_iter(chars.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Edit::*;

    #[test]
    #[should_panic]
    fn insert_out_of_bounds_panics() {
        Insert { c: 'a', index: 1 }.apply("");
    }

    #[test]
    #[should_panic]
    fn delete_out_of_bounds_panics() {
        Delete { index: 1 }.apply("");
    }

    #[test]
    #[should_panic]
    fn substitute_out_of_bounds_panics() {
        Substitute { c: 'a', index: 1 }.apply("");
    }
}
