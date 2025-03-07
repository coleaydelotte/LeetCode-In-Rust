/*
Problem: Remove Element
Given an integer array nums and an integer val, remove all occurrences of val
in nums in-place. The relative order of the elements may be changed.
*/
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k: usize = 0 as usize;
    for x in 0..nums.len() {
        if nums[x] != val {
            nums[k] = nums[x];
            k += 1 as usize;
        }
    }
    k as i32
}

pub fn main() {
    // Output: 2
    let mut nums: Vec<i32> = vec![3, 2, 2, 3];
    let val: i32 = 3;
    let result = remove_element(&mut nums, val);
    println!("Result: {}", result);

    // Output: 5
    let mut nums: Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let val: i32 = 2;
    let result = remove_element(&mut nums, val);
    println!("Result: {}", result);
}