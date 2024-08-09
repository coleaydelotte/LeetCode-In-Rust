// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32
{
    if nums.len() == 0 { return 0; }

    let mut unique_index = 0;

    for i in 1..nums.len()
    {
        if nums[i] != nums[unique_index]
        {
            unique_index += 1;
            nums[unique_index] = nums[i];
        }
    }

    (unique_index + 1) as i32
}

fn main()
{
    let mut nums: Vec<i32> = vec![0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 5];
    let mut nums_repeating: Vec<i32> = vec![0; 10];
    println!("{}", remove_duplicates(&mut nums));
    println!("{:?}", nums);
    println!("{}", remove_duplicates(&mut nums_repeating));
    println!("{:?}", nums_repeating);
}