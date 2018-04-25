extern crate algorithm;
use algorithm::dynamic::longest_common_substring::get_longest_common_substring;

#[test]
fn test_substring() {
    let source = "hish";
    let candidates = vec!["vista", "fish", "hosh"]; // 'hosh' for testing equality of first charactor.
    let best_matches = get_longest_common_substring(source, &candidates);
    assert_eq!(best_matches, vec!["fish"]);
}

#[test]
fn test_substring_chinese() {
    let source = "我是小明";
    let candidates = vec!["他非小明", "他不是小明", "我非小明"]; 
    let best_matches = get_longest_common_substring(source, &candidates);
    assert_eq!(best_matches, vec!["他不是小明"]);
}