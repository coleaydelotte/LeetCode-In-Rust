/*
At First misunderstood the question, thought that we have to jump to 
the index of the value at the current index.
But found that the value at the current index is the maximum number of
steps you can make.

You are given an integer array nums. You are initially
positioned at the array's first index, and each element
in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.
*/
fn jump_game(nums: Vec<i32>) -> bool
{
    let mut index: usize = 0;
    while index < nums.len() - 1
    {
        let pos = nums[index] as usize;
        if index + pos >= nums.len() - 1
        {
            return true;
        }
        if pos == 0
        {
            return false;
        }
        let mut max_reach = 0;
        let mut best_index = index;
        for i in 1..=pos
        {
            if index + i < nums.len() && (index + i) + nums[index + i] as usize > max_reach
            {
                max_reach = (index + i) + nums[index + i] as usize;
                best_index = index + i;
            }
        }
        index = best_index;
    }
    true
}

fn main()
{
    // Output: true
    let nums: Vec<i32> = vec![2,3,1,1,4];
    let result = jump_game(nums);
    println!("Result: {}", result);

    // Output: false
    let nums: Vec<i32> = vec![3,2,1,0,4];
    let result = jump_game(nums);
    println!("Result: {}", result);

    // Output: true
    let nums: Vec<i32> = vec![2,0,0];
    let result = jump_game(nums);
    println!("Result: {}", result);

    // Output: true
    let nums: Vec<i32> = vec![1, 2, 5, 0];
    let result = jump_game(nums);
    println!("Result: {}", result);
}