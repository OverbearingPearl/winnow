use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn synthesize(examples: Vec<String>) -> String {
    synthesize_with_annotation(examples, Vec::new())
}

#[wasm_bindgen]
pub fn synthesize_with_annotation(pos: Vec<String>, neg: Vec<String>) -> String {
    if pos.is_empty() {
        return ".*".to_string();
    }

    let prefix_len = longest_common_prefix(&pos).len();
    let suffix_len_orig = longest_common_suffix(&pos).len();
    let min_remaining = pos
        .iter()
        .map(|s| s.len().saturating_sub(prefix_len))
        .min()
        .unwrap_or(0);
    let suffix_len = suffix_len_orig.min(min_remaining);

    let prefix = &pos[0][..prefix_len];
    let suffix = &pos[0][pos[0].len() - suffix_len..];

    let middles: Vec<&str> = pos
        .iter()
        .map(|s| {
            let start = prefix_len;
            let end = s.len() - suffix_len;
            &s[start..end]
        })
        .collect();

    if middles.iter().all(|m| m.is_empty()) {
        let pattern = format!("{}{}", regex::escape(prefix), regex::escape(suffix));
        if all_neg_not_match(&pattern, &neg) {
            return pattern;
        }
        return ".*".to_string();
    }

    let middle_pattern = infer_middle_pattern(&middles);
    let pattern = format!(
        "{}{}{}",
        regex::escape(prefix),
        middle_pattern,
        regex::escape(suffix)
    );

    if all_neg_not_match(&pattern, &neg) {
        pattern
    } else {
        ".*".to_string()
    }
}

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = strings[0].as_bytes();
    let mut len = 0;
    'outer: for i in 0..first.len() {
        let c = first[i];
        for s in &strings[1..] {
            if i >= s.len() || s.as_bytes()[i] != c {
                break 'outer;
            }
        }
        len = i + 1;
    }
    strings[0][..len].to_string()
}

fn longest_common_suffix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first = strings[0].as_bytes();
    let n = first.len();
    let mut len = 0;
    'outer: for i in 0..n {
        let c = first[n - 1 - i];
        for s in &strings[1..] {
            let sb = s.as_bytes();
            if i >= sb.len() || sb[sb.len() - 1 - i] != c {
                break 'outer;
            }
        }
        len = i + 1;
    }
    strings[0][n - len..].to_string()
}

fn infer_middle_pattern(middles: &[&str]) -> String {
    if middles.iter().any(|m| m.is_empty()) {
        return ".*".to_string();
    }
    let all_alpha = middles
        .iter()
        .all(|m| m.chars().all(|c| c.is_ascii_alphabetic()));
    let all_digit = middles
        .iter()
        .all(|m| m.chars().all(|c| c.is_ascii_digit()));
    if all_alpha {
        "[a-zA-Z]+".to_string()
    } else if all_digit {
        "\\d+".to_string()
    } else {
        ".*".to_string()
    }
}

fn all_neg_not_match(pattern: &str, neg: &[String]) -> bool {
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(_) => return false,
    };
    neg.iter().all(|s| !re.is_match(s))
}
