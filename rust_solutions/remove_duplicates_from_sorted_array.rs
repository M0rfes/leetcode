fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut insert_index = 1;

    for i in 1..nums.len() {
        if nums[i] != nums[insert_index - 1] {
            nums[insert_index] = nums[i];
            insert_index += 1;
        }
    }

    insert_index as i32
}
