// Wordle game library — a project showcasing iterators in action.
//
// The game picks a 5-letter word. The player gets 6 guesses.
// After each guess, each letter is colored:
// - Correct (right letter, right position)
// - Present (right letter, wrong position)
// - Absent  (letter not in word)

/// Feedback for a single letter in a guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterResult {
    /// Correct letter in the correct position.
    Correct,
    /// Letter exists in the word but in a different position.
    Present,
    /// Letter does not exist in the word.
    Absent,
}

/// Result of evaluating a full guess against the answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuessResult {
    pub letters: Vec<(char, LetterResult)>,
}

impl GuessResult {
    /// Returns true if all letters are Correct (i.e., the guess matches).
    pub fn is_winner(&self) -> bool {
        self.letters
            .iter()
            .all(|(_, r)| *r == LetterResult::Correct)
    }
}

impl std::fmt::Display for GuessResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (ch, result) in &self.letters {
            let symbol = match result {
                LetterResult::Correct => "+",
                LetterResult::Present => "?",
                LetterResult::Absent => "-",
            };
            write!(f, "[{ch}{symbol}]")?;
        }
        Ok(())
    }
}

/// Evaluate a guess against the answer.
///
/// # Arguments
///
/// * `answer` - The secret 5-letter word (lowercase ASCII)
/// * `guess` - The player's guess (lowercase ASCII, must be 5 letters)
///
/// # Returns
///
/// A `GuessResult` with per-letter feedback.
///
/// # Algorithm
///
/// 1. First pass: mark exact matches as Correct.
/// 2. Second pass: for remaining letters, mark Present if the letter
///    exists in unmatched answer positions, otherwise Absent.
///
/// # Examples
///
/// ```
/// use s21_wordle::{evaluate_guess, LetterResult};
/// let result = evaluate_guess("crane", "crate");
/// assert_eq!(result.letters[0].1, LetterResult::Correct); // c
/// assert_eq!(result.letters[3].1, LetterResult::Absent);  // t
/// ```
pub fn evaluate_guess(answer: &str, guess: &str) -> GuessResult {
    let answer_chars: Vec<char> = answer.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    let len = answer_chars.len();

    // Track which answer positions are still available for "Present" matches
    let mut available: Vec<bool> = vec![true; len];
    let mut results = vec![LetterResult::Absent; len];

    // First pass: find exact matches (Correct)
    for i in 0..len {
        if guess_chars[i] == answer_chars[i] {
            results[i] = LetterResult::Correct;
            available[i] = false;
        }
    }

    // Second pass: find present (wrong position) matches
    for i in 0..len {
        if results[i] == LetterResult::Correct {
            continue;
        }
        // Look for this guess letter in available answer positions
        if let Some(j) = available
            .iter()
            .enumerate()
            .position(|(j, &avail)| avail && answer_chars[j] == guess_chars[i])
        {
            results[i] = LetterResult::Present;
            available[j] = false;
        }
    }

    GuessResult {
        letters: guess_chars.into_iter().zip(results).collect(),
    }
}

/// A simple word list for the game.
pub const WORD_LIST: &[&str] = &[
    "crane", "slate", "adieu", "audio", "raise", "stare", "irate", "arise", "route", "outer",
    "saint", "trace", "crate", "gripe", "shale", "those", "plane", "globe", "dance", "chore",
];

/// Pick a word from the list given an index (wraps around).
///
/// # Arguments
///
/// * `index` - Index to select from the word list
///
/// # Returns
///
/// A word from `WORD_LIST`.
pub fn pick_word(index: usize) -> &'static str {
    WORD_LIST[index % WORD_LIST.len()]
}

/// Validate that a guess is exactly 5 lowercase ASCII letters.
///
/// # Arguments
///
/// * `guess` - The guess to validate
///
/// # Returns
///
/// `true` if the guess is valid, `false` otherwise.
pub fn is_valid_guess(guess: &str) -> bool {
    guess.len() == 5 && guess.chars().all(|c| c.is_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let result = evaluate_guess("crane", "crane");
        assert!(result.is_winner());
        assert!(
            result
                .letters
                .iter()
                .all(|(_, r)| *r == LetterResult::Correct)
        );
    }

    #[test]
    fn test_no_match() {
        let result = evaluate_guess("crane", "glyph");
        assert!(
            result
                .letters
                .iter()
                .all(|(_, r)| *r == LetterResult::Absent)
        );
    }

    #[test]
    fn test_partial_match() {
        let result = evaluate_guess("crane", "crate");
        assert_eq!(result.letters[0].1, LetterResult::Correct); // c
        assert_eq!(result.letters[1].1, LetterResult::Correct); // r
        assert_eq!(result.letters[2].1, LetterResult::Correct); // a
        assert_eq!(result.letters[3].1, LetterResult::Absent); // t not in answer
        assert_eq!(result.letters[4].1, LetterResult::Correct); // e
    }

    #[test]
    fn test_present_letters() {
        let result = evaluate_guess("crane", "nerdy");
        // n is in crane at pos 3, guess has it at pos 0 → Present
        assert_eq!(result.letters[0].1, LetterResult::Present); // n
        assert_eq!(result.letters[1].1, LetterResult::Present); // e
        assert_eq!(result.letters[2].1, LetterResult::Present); // r
        assert_eq!(result.letters[3].1, LetterResult::Absent); // d
        assert_eq!(result.letters[4].1, LetterResult::Absent); // y
    }

    #[test]
    fn test_duplicate_letters() {
        // answer "creep", guess "eerie"
        // answer: c r e e p
        // guess:  e e r i e
        let result = evaluate_guess("creep", "eerie");
        assert_eq!(result.letters[0].1, LetterResult::Present); // e → present (pos 2 or 3)
        assert_eq!(result.letters[1].1, LetterResult::Present); // e → present (other e)
        assert_eq!(result.letters[2].1, LetterResult::Present); // r → present (pos 1)
        assert_eq!(result.letters[3].1, LetterResult::Absent); // i → absent
        assert_eq!(result.letters[4].1, LetterResult::Absent); // e → no more e's available
    }

    #[test]
    fn test_display() {
        let result = evaluate_guess("crane", "crane");
        let s = format!("{result}");
        assert!(s.contains("[c+]"));
    }

    #[test]
    fn test_is_valid_guess() {
        assert!(is_valid_guess("crane"));
        assert!(is_valid_guess("hello"));
        assert!(!is_valid_guess("HELLO")); // uppercase
        assert!(!is_valid_guess("hi")); // too short
        assert!(!is_valid_guess("toolong")); // too long
        assert!(!is_valid_guess("he lo")); // space
    }

    #[test]
    fn test_pick_word() {
        assert_eq!(pick_word(0), "crane");
        assert_eq!(pick_word(WORD_LIST.len()), "crane"); // wraps
    }
}
