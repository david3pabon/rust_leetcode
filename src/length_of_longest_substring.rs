pub fn length_of_longest_substring(s: String) -> i32 {
    let s = s.as_bytes();
    let mut seen = std::collections::HashSet::<u8>::new();
    let mut result: i32 = 0;
    let mut add = 0;
    let mut del = 0;

    while del < s.len() && add < s.len() {
        if !seen.contains(&s[add]) {
            seen.insert(s[add]);
            result = std::cmp::max(result, add as i32 - del as i32 + 1);
            add += 1;
        } else {
            seen.remove(&s[del]);
            del += 1;
        }
    }

    result
}

#[test]
fn test_length_of_longest_substring() {
    // Arrange


    // Act
    let result = length_of_longest_substring(String::from("abcabcbb"));

    // Assert
    assert_eq!(3, result)
}