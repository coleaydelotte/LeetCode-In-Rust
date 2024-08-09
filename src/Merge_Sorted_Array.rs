pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) 
{
    let mut merged_index: i32 = (nums1.len() - 1) as i32;
    let mut index_one = m - 1;
    let mut index_two = n - 1;
    
    while merged_index >= 0
    {
        if index_one >= 0 && (index_two < 0 || nums1[index_one as usize] > nums2[index_two as usize])
        {
            nums1[merged_index as usize] = nums1[index_one as usize];
            index_one -= 1;
        }
        else
        {
            nums1[merged_index as usize] = nums2[index_two as usize];
            index_two -= 1;
        }
        merged_index -= 1;
    }
}

fn main() 
{
    let mut numbers1 = vec![1, 2, 3, 0, 0, 0];
    let mut numbers2 = vec![2, 5, 6];
    merge(&mut numbers1, 3, &mut numbers2, 3);
    println!("{:?}", numbers1); // Output: [1, 2, 2, 3, 5, 6]

    let mut numbers1 = vec![1];
    let mut numbers2 = vec![2, 0];
    merge(&mut numbers1, 1, &mut numbers2, 1);
    println!("{:?}", numbers1); // Output: [1, 2]

    let mut numbers1 = vec![0];
    let mut numbers2 = vec![1];
    merge(&mut numbers1, 0, &mut numbers2, 1);
    println!("{:?}", numbers1); // Output: [1]
}
