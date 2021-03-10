pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if (nums[i] + nums[j]) == target && i != j {
                return if i <= j {
                    vec![i as i32, j as i32]
                } else {
                    vec![j as i32, i as i32]
                };
            }
        }
    }

    return nums
}

#[test]
fn test_two_sum_1() {
    // Arrange
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];

    // Act
    let actual = two_sum(nums, target);

    // Assert
    assert_eq!(expected, actual)
}

#[test]
fn test_two_sum_2() {
    // Arrange
    let nums = vec![3, 2, 4];
    let target = 6;
    let expected = vec![1, 2];

    // Act
    let actual = two_sum(nums, target);

    // Assert
    assert_eq!(expected, actual)
}