fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut writer = 1;

    for reader in 1..nums.len() {
        if nums[reader] != nums[writer - 1] {
            nums[writer] = nums[reader];
            writer += 1;
        }
    }

    writer as i32
}
