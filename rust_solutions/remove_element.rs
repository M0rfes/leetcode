pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut writer = 0;
    for reader in 0..nums.len() {
        if nums[reader] != val {
            nums[writer] = nums[reader];
            writer += 1;
        }
    }
    writer as i32
}
