extern crate algorithm;
use algorithm::dynamic::longest_common_subset::get_longest_common_subset;

#[test]
fn test_subset() {
    let source = "hish";
    let candidates = vec!["vista", "fish", "hosh"]; // 'hosh' for testing equality of first charactor.
    let best_matches = get_longest_common_subset(source, &candidates);
    assert_eq!(best_matches, vec!["fish", "hosh"]);
}

#[test]
fn test_subset_chinese() {
    let source = "我是小明";
    let candidates = vec!["他非小明", "他不是小明", "我非小明"];
    let best_matches = get_longest_common_subset(source, &candidates);
    assert_eq!(best_matches, vec!["他不是小明", "我非小明"]);
}
