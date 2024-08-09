use std::collections::HashMap;

/*
Given a non-empty array of integers nums, every element 
appears twice except for one. Find that single one.

You must implement a solution with a linear runtime complexity
and use only constant extra space.
 */

//Time complexity: O(n)
pub fn single_number(nums: Vec<i32>) -> i32
{
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len()
    {
        if let Some(value) = map.get_mut(&nums[i])
        {
            *value += 1;
        }
        else
        {
            map.insert(nums[i], 1);
        }
    }
    for (key, value) in map.iter()
    {
        if *value == 1
        {
            return *key;
        }
    }
    0

    //Time complexity: O(nlogn)
    // Slightly slower but simpler solution:
    /*
        let mut nums = nums;
        nums.sort();
        for i in (0..nums.len() - 1).step_by(2)
        {
            if nums[i] != nums[i + 1]
            {
                return nums[i];
            }
        }
        nums[nums.len() - 1]
    */

}

fn main()
{
    let nums: Vec<i32> = vec![2, 2, 1];
    println!("{}", single_number(nums)); // Output: 1

    let nums: Vec<i32> = vec![4, 1, 2, 1, 2];
    println!("{}", single_number(nums)); // Output: 4

    let nums: Vec<i32> = vec![1];
    println!("{}", single_number(nums)); // Output: 1
}