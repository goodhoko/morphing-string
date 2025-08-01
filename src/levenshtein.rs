use std::collections::VecDeque;

use crate::edit::Edit;

/// Compute a sequence of [`Edit`]s that when applied onto `start` will turn it into `target`.
/// The Edits have to be applied front to back.
pub fn compute_edit_sequence(start: &str, target: &str) -> VecDeque<Edit> {
    let start_chars: Vec<char> = start.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let start_len = start_chars.len();
    let target_len = target_chars.len();

    // Compute a matrix where dp[i][j] = minimal number of edits to convert a prefix of
    // start[0..i] to prefix of target[0..j].
    let mut dp = vec![vec![0; target_len + 1]; start_len + 1];

    #[expect(clippy::needless_range_loop)]
    for i in 1..=start_len {
        // Converting string of length i to an empty string takes i deletions.
        dp[i][0] = i;
    }
    for j in 1..=target_len {
        // Converting an empty string into a string of length j takes j insertions.
        dp[0][j] = j;
    }

    for i in 1..=start_len {
        for j in 1..=target_len {
            let substitution_distance = if start_chars[i - 1] == target_chars[j - 1] {
                // Chars actually match. Same distance as of the two shorter prefixes.
                dp[i - 1][j - 1]
            } else {
                // Chars differ so this is an actual substitutions for a *different* char.
                dp[i - 1][j - 1] + 1
            };
            let deletion_distance = dp[i - 1][j] + 1;
            let insertion_distance = dp[i][j - 1] + 1;

            dp[i][j] = substitution_distance
                .min(deletion_distance)
                .min(insertion_distance);
        }
    }

    // Do a gradient-descent through the dp matrix backtracking the edits along the way.
    let mut i = start_len;
    let mut j = target_len;
    let mut edits: VecDeque<Edit> = VecDeque::new();

    while i > 0 || j > 0 {
        let edit = if i == 0 {
            // Getting to a non-empty string from an empty one can only be done with Insertions.
            Edit::Insert {
                c: target_chars[j - 1],
                index: 0,
            }
        } else if j == 0 {
            // Getting to an empty string from some non-empty one can only be done with Deletions.
            Edit::Delete { index: i - 1 }
        } else if start_chars[i - 1] == target_chars[j - 1] {
            // Chars are equal. Just move on in both strings.
            i -= 1;
            j -= 1;
            continue;
        } else {
            // chars are not equal and we have the choice of choosing any Edit. Choose the one that
            // moves us to a position in the matrix that has the lowest Levenshtein distance.
            [
                (
                    dp[i - 1][j - 1],
                    Edit::Substitute {
                        c: target_chars[j - 1],
                        index: i - 1,
                    },
                ),
                (
                    dp[i][j - 1],
                    Edit::Insert {
                        c: target_chars[j - 1],
                        index: i,
                    },
                ),
                (dp[i - 1][j], Edit::Delete { index: i - 1 }),
            ]
            .iter()
            .min_by_key(|(distance, _)| distance)
            .expect("this is a non-empty list")
            .1
        };

        match edit {
            Edit::Insert { .. } => {
                j -= 1;
            }
            Edit::Delete { .. } => {
                i -= 1;
            }
            Edit::Substitute { .. } => {
                i -= 1;
                j -= 1;
            }
        }

        edits.push_front(edit);
    }

    // The Edits' indexes does not account for shifts caused by previously applied Inserts or
    // Deletions. Correct for that.
    let mut shift = 0i64;
    for edit in edits.iter_mut() {
        match edit {
            Edit::Insert { index, .. } => {
                *index = (*index as i64 + shift) as usize;
                shift += 1;
            }
            Edit::Delete { index } => {
                *index = (*index as i64 + shift) as usize;
                shift -= 1;
            }
            Edit::Substitute { index, .. } => {
                *index = (*index as i64 + shift) as usize;
            }
        }
    }

    edits
}

#[cfg(test)]
mod tests {
    use super::*;
    use Edit::*;

    struct Case {
        name: &'static str,
        start: &'static str,
        target: &'static str,
        expected_edits: Vec<Edit>,
    }

    #[test]
    fn compute_and_apply() {
        let test_cases = &[
            Case {
                name: "empty to empty",
                start: "",
                target: "",
                expected_edits: vec![],
            },
            Case {
                name: "start equal to target",
                start: "abcd",
                target: "abcd",
                expected_edits: vec![],
            },
            Case {
                name: "single insert",
                start: "",
                target: "a",
                expected_edits: vec![Insert { c: 'a', index: 0 }],
            },
            Case {
                name: "single delete",
                start: "a",
                target: "",
                expected_edits: vec![Delete { index: 0 }],
            },
            Case {
                name: "single substitution",
                start: "a",
                target: "b",
                expected_edits: vec![Substitute { c: 'b', index: 0 }],
            },
            Case {
                name: "multiple inserts",
                start: "",
                target: "012",
                expected_edits: vec![
                    Insert { c: '0', index: 0 },
                    Insert { c: '1', index: 1 },
                    Insert { c: '2', index: 2 },
                ],
            },
            Case {
                name: "multiple deletes",
                start: "abc",
                target: "",
                expected_edits: vec![
                    Delete { index: 0 },
                    Delete { index: 0 },
                    Delete { index: 0 },
                ],
            },
            Case {
                name: "multiple substitutions",
                start: "abc",
                target: "012",
                expected_edits: vec![
                    Substitute { c: '0', index: 0 },
                    Substitute { c: '1', index: 1 },
                    Substitute { c: '2', index: 2 },
                ],
            },
            Case {
                name: "indexes account for previous insertions",
                start: "01234",
                target: "x0123a",
                expected_edits: vec![Insert { c: 'x', index: 0 }, Substitute { c: 'a', index: 5 }],
            },
            Case {
                name: "kitten mittens",
                start: "kitten",
                target: "mittens",
                expected_edits: vec![Substitute { c: 'm', index: 0 }, Insert { c: 's', index: 6 }],
            },
            Case {
                name: "mixed insert delete substitute",
                start: "abcdef",
                target: "xazced",
                expected_edits: vec![
                    Edit::Insert { c: 'x', index: 0 },
                    Edit::Substitute { c: 'z', index: 2 },
                    Edit::Delete { index: 4 },
                    Edit::Substitute { c: 'd', index: 5 },
                ],
            },
            Case {
                name: "multiple edits",
                start: "sunday",
                target: "saturday",
                expected_edits: vec![
                    Edit::Insert { c: 'a', index: 1 },
                    Edit::Insert { c: 't', index: 2 },
                    Edit::Substitute { c: 'r', index: 4 },
                ],
            },
            Case {
                name: "random garbage",
                start: "daskdas dasd sadjasnd dsdjfh sd fadfbasdjnf",
                target: "nfad ad f sasdkmfpsmdfasM Ksmdnfkdskmnflsokdsfnlsdknffs",
                expected_edits: vec![
                    Insert { c: 'n', index: 0 },
                    Substitute { c: 'f', index: 1 },
                    Insert { c: 'd', index: 3 },
                    Substitute { c: ' ', index: 4 },
                    Substitute { c: 'a', index: 5 },
                    Substitute { c: ' ', index: 7 },
                    Substitute { c: 'f', index: 8 },
                    Substitute { c: 's', index: 10 },
                    Insert { c: 'k', index: 14 },
                    Insert { c: 'm', index: 15 },
                    Insert { c: 'f', index: 16 },
                    Substitute { c: 'p', index: 17 },
                    Substitute { c: 'm', index: 19 },
                    Substitute { c: 'f', index: 21 },
                    Delete { index: 24 },
                    Substitute { c: 'M', index: 24 },
                    Substitute { c: 'K', index: 26 },
                    Insert { c: 'm', index: 28 },
                    Substitute { c: 'n', index: 30 },
                    Substitute { c: 'k', index: 32 },
                    Substitute { c: 'd', index: 33 },
                    Insert { c: 'k', index: 35 },
                    Substitute { c: 'm', index: 36 },
                    Substitute { c: 'n', index: 37 },
                    Insert { c: 'l', index: 39 },
                    Insert { c: 's', index: 40 },
                    Insert { c: 'o', index: 41 },
                    Substitute { c: 'k', index: 42 },
                    Insert { c: 's', index: 44 },
                    Substitute { c: 'n', index: 46 },
                    Substitute { c: 'l', index: 47 },
                    Substitute { c: 'k', index: 50 },
                    Insert { c: 'f', index: 52 },
                    Insert { c: 's', index: 54 },
                ],
            },
        ];

        for Case {
            name,
            start,
            target,
            expected_edits,
        } in test_cases.iter()
        {
            let edits = compute_edit_sequence(start, target);
            assert_eq!(edits, *expected_edits, "{name}: edits match expectation");

            let mut string = start.to_string();
            for edit in edits {
                string = edit.apply(&string);
            }

            assert_eq!(
                &string, target,
                "{name}: edits applied to start produce target",
            );
        }
    }
}
