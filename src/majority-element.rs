/*
Given an array nums of size n, return the majority element.
The majority element is the element that appears more than ⌊n / 2⌋ times. 
You may assume that the majority element always exists in the array.
*/
fn majority(nums: Vec<i32>) -> i32
{
    let mut candidate = 0;
    let mut count = 0;

    for num in nums
    {
        if count == 0
        {
            candidate = num;
        }
        if num == candidate
        {
            count += 1;
        }
        else
        {
            count -= 1;
        }
    }

    candidate
}
fn main()
{
    //Majority element is 0
    let nums: Vec<i32> = vec!(0;10);
    let number_returned: i32;
    number_returned = majority(nums.clone());
    println!("{:?}", nums);
    println!("number returned: {}", number_returned);

    //Majority element is 3
    let nums: Vec<i32> = vec![1, 1, 2, 2, 2, 3, 3, 3, 3];
    let number_returned: i32;
    number_returned = majority(nums.clone());
    println!("{:?}", nums);
    println!("number returned: {}", number_returned);
}