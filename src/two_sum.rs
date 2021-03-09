pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    return nums
}

#[test]
fn test_two_sum_positive() {
    // Arrange
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];

    // Act
    let actual = two_sum(nums, target);

    // Assert
    assert_eq!(expected, actual)
}