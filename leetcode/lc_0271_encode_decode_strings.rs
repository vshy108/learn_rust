// LeetCode 271: Encode and Decode Strings [Medium] (Premium)
// https://leetcode.com/problems/encode-and-decode-strings/
//
// First benchmark: 100_000 iterations, 2025-07-07
//   My solution avg: 1.079us
//   AI solution avg: 0.933us
//   Ratio: AI 1.16x faster
//
// Design an algorithm to encode a list of strings to a single string,
// and decode the single string back to the original list of strings.
//
// Example 1: ["Hello","World"] -> encode -> decode -> ["Hello","World"]
// Example 2: [""]              -> encode -> decode -> [""]
//
// Constraints:
//   - 1 <= strs.length <= 200
//   - 0 <= strs[i].length <= 200
//   - strs[i] contains any possible characters out of 256 valid ASCII characters
//
// Reference: https://github.com/doocs/leetcode/blob/main/solution/0200-0299/0271.Encode%20and%20Decode%20Strings/README_EN.md
//
// Run:  cargo run --example lc_0271_encode_decode_strings
// Test: cargo test --example lc_0271_encode_decode_strings
// Bench: cargo test --example lc_0271_encode_decode_strings bench -- --nocapture

/// My solution — Fixed 4-char length prefix (from doocs).
///
/// Encode each string as `{4-digit-length}{string}`. Decode reads the
/// first 4 chars to get the length, then extracts that many chars.
/// Works because max string length is 200, which fits in 4 digits.
///
/// Time:  O(n) where n = total chars across all strings
/// Space: O(n)
mod my_solution {
    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Encode a list of strings to a single string.
        pub fn encode(&self, strs: Vec<String>) -> String {
            let mut encoded = String::new();
            for s in &strs {
                // Write length as zero-padded 4-digit decimal
                encoded.push_str(&format!("{:04}", s.len()));
                encoded.push_str(s);
            }
            encoded
        }

        /// Decode a single string back to a list of strings.
        pub fn decode(&self, s: String) -> Vec<String> {
            let mut result = Vec::new();
            let bytes = s.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                // Parse the 4-digit length prefix
                let len_str = &s[i..i + 4];
                let size: usize = len_str.parse().expect("valid 4-digit length");
                i += 4;
                result.push(s[i..i + size].to_string());
                i += size;
            }
            result
        }
    }
}

/// AI solution — Variable-length numeric prefix with '#' delimiter.
///
/// Encode each string as `{length}#{string}`. The '#' delimiter marks
/// the end of the length field, allowing variable-width length encoding.
/// This approach is more space-efficient for short strings and works for
/// arbitrarily long strings (not limited to 9999 chars).
///
/// Time:  O(n) where n = total chars across all strings
/// Space: O(n)
mod ai_solution {
    pub struct Codec;

    impl Codec {
        pub fn new() -> Self {
            Codec
        }

        /// Encode a list of strings to a single string.
        pub fn encode(&self, strs: Vec<String>) -> String {
            let mut encoded = String::new();
            for s in &strs {
                // Variable-length prefix: "len#content"
                encoded.push_str(&s.len().to_string());
                encoded.push('#');
                encoded.push_str(s);
            }
            encoded
        }

        /// Decode a single string back to a list of strings.
        pub fn decode(&self, s: String) -> Vec<String> {
            let mut result = Vec::new();
            let bytes = s.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                // Find the '#' delimiter to extract the length
                let hash_pos = s[i..].find('#').expect("valid encoding has '#'");
                let size: usize =
                    s[i..i + hash_pos].parse().expect("valid numeric length");
                i += hash_pos + 1; // skip past '#'
                result.push(s[i..i + size].to_string());
                i += size;
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: round-trip encode then decode, assert equal to original.
    macro_rules! test_codec {
        ($name:ident, $input:expr) => {
            #[test]
            fn $name() {
                let strs: Vec<String> =
                    $input.iter().map(|s| s.to_string()).collect();

                let my_codec = my_solution::Codec::new();
                let my_encoded = my_codec.encode(strs.clone());
                let my_decoded = my_codec.decode(my_encoded);
                assert_eq!(my_decoded, strs, "my_solution round-trip failed");

                let ai_codec = ai_solution::Codec::new();
                let ai_encoded = ai_codec.encode(strs.clone());
                let ai_decoded = ai_codec.decode(ai_encoded);
                assert_eq!(ai_decoded, strs, "ai_solution round-trip failed");
            }
        };
    }

    // Example 1
    test_codec!(test_example_1, ["Hello", "World"]);

    // Example 2: single empty string
    test_codec!(test_example_2, [""]);

    // Multiple empty strings
    test_codec!(test_multiple_empty, ["", "", ""]);

    // Single non-empty string
    test_codec!(test_single_string, ["abc"]);

    // String with spaces
    test_codec!(test_with_spaces, ["hello world", "foo bar"]);

    // String with special characters
    test_codec!(test_special_chars, ["a#b", "c:d", "e\nf"]);

    // String with digits (could confuse length prefix)
    test_codec!(test_digits_in_content, ["1234", "5678"]);

    // String containing '#' (could confuse AI solution delimiter)
    test_codec!(test_hash_in_content, ["##", "#abc#", "x#y"]);

    // String containing '0000' (could confuse my_solution length prefix)
    test_codec!(test_zeros_in_content, ["0000", "00001234"]);

    // Mixed empty and non-empty
    test_codec!(test_mixed_empty, ["", "a", "", "bc", ""]);

    // Long string (200 chars)
    #[test]
    fn test_long_string() {
        let long = "a".repeat(200);
        let strs = vec![long.clone()];

        let my_codec = my_solution::Codec::new();
        let my_decoded = my_codec.decode(my_codec.encode(strs.clone()));
        assert_eq!(my_decoded, strs, "my_solution failed on long string");

        let ai_codec = ai_solution::Codec::new();
        let ai_decoded = ai_codec.decode(ai_codec.encode(strs.clone()));
        assert_eq!(ai_decoded, strs, "ai_solution failed on long string");
    }

    // Many strings
    #[test]
    fn test_many_strings() {
        let strs: Vec<String> = (0..200).map(|i| format!("str_{i}")).collect();

        let my_codec = my_solution::Codec::new();
        let my_decoded = my_codec.decode(my_codec.encode(strs.clone()));
        assert_eq!(my_decoded, strs, "my_solution failed on many strings");

        let ai_codec = ai_solution::Codec::new();
        let ai_decoded = ai_codec.decode(ai_codec.encode(strs.clone()));
        assert_eq!(ai_decoded, strs, "ai_solution failed on many strings");
    }

    // String with all printable ASCII chars
    #[test]
    fn test_all_ascii() {
        let all_ascii: String = (32u8..127).map(|b| b as char).collect();
        let strs = vec![all_ascii.clone()];

        let my_codec = my_solution::Codec::new();
        let my_decoded = my_codec.decode(my_codec.encode(strs.clone()));
        assert_eq!(my_decoded, strs);

        let ai_codec = ai_solution::Codec::new();
        let ai_decoded = ai_codec.decode(ai_codec.encode(strs.clone()));
        assert_eq!(ai_decoded, strs);
    }

    // Single character strings
    test_codec!(test_single_chars, ["a", "b", "c"]);

    // String with newlines and tabs
    test_codec!(test_whitespace_chars, ["\t", "\n", "\r\n", "  "]);
}

fn main() {
    println!("LeetCode 271: Encode and Decode Strings");

    let strs: Vec<String> =
        vec!["Hello".to_string(), "World".to_string()];

    let my_codec = my_solution::Codec::new();
    let encoded = my_codec.encode(strs.clone());
    println!("my_solution encode: {:?} -> {:?}", strs, encoded);
    println!("my_solution decode: {:?}", my_codec.decode(encoded));

    let ai_codec = ai_solution::Codec::new();
    let encoded = ai_codec.encode(strs.clone());
    println!("ai_solution encode: {:?} -> {:?}", strs, encoded);
    println!("ai_solution decode: {:?}", ai_codec.decode(encoded));
}

#[cfg(test)]
mod bench {
    use super::*;

    #[test]
    fn bench_solutions() {
        let strs: Vec<String> =
            vec!["Hello", "World", "LeetCode", "Rust", ""]
                .into_iter()
                .map(String::from)
                .collect();
        let iterations = 100_000;

        let my_codec = my_solution::Codec::new();
        let ai_codec = ai_solution::Codec::new();

        // Warm up
        for _ in 0..1_000 {
            let e = my_codec.encode(strs.clone());
            std::hint::black_box(my_codec.decode(e));
            let e = ai_codec.encode(strs.clone());
            std::hint::black_box(ai_codec.decode(e));
        }

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let e = my_codec.encode(strs.clone());
            std::hint::black_box(my_codec.decode(e));
        }
        let my_dur = start.elapsed();

        let start = std::time::Instant::now();
        for _ in 0..iterations {
            let e = ai_codec.encode(strs.clone());
            std::hint::black_box(ai_codec.decode(e));
        }
        let ai_dur = start.elapsed();

        let my_avg = my_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ai_avg = ai_dur.as_nanos() as f64 / iterations as f64 / 1000.0;
        let ratio = if my_avg < ai_avg {
            format!("My {:.2}x faster", ai_avg / my_avg)
        } else {
            format!("AI {:.2}x faster", my_avg / ai_avg)
        };

        println!("\n--- Benchmark ({iterations} iterations) ---");
        println!("My solution avg: {my_avg:.3}us");
        println!("AI solution avg: {ai_avg:.3}us");
        println!("Ratio: {ratio}");
    }
}
