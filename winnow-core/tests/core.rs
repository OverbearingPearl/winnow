use winnow_core::synthesize_with_annotation;
use regex::Regex;

#[test]
fn letters_with_common_prefix() {
    let pos = vec!["buy milk".to_string(), "buy eggs".to_string()];
    let neg = vec!["call mom".to_string()];
    let pattern = synthesize_with_annotation(pos, neg);
    let re = Regex::new(&pattern).unwrap();
    assert!(re.is_match("buy milk"));
    assert!(re.is_match("buy eggs"));
    assert!(!re.is_match("call mom"));
}

#[test]
fn digits_only() {
    let pos = vec!["123".to_string(), "456".to_string()];
    let neg = vec!["abc".to_string()];
    let pattern = synthesize_with_annotation(pos, neg);
    let re = Regex::new(&pattern).unwrap();
    assert!(re.is_match("123"));
    assert!(re.is_match("456"));
    assert!(!re.is_match("abc"));
}

#[test]
fn identical_positive() {
    let pos = vec!["hello".to_string()];
    let neg = vec!["world".to_string()];
    let pattern = synthesize_with_annotation(pos, neg);
    let re = Regex::new(&pattern).unwrap();
    assert!(re.is_match("hello"));
    assert!(!re.is_match("world"));
}

#[test]
fn empty_positives_fallback() {
    let pos: Vec<String> = vec![];
    let neg = vec!["anything".to_string()];
    let pattern = synthesize_with_annotation(pos, neg);
    assert_eq!(pattern, ".*");
}
