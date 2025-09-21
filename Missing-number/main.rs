
/* 
Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.


Example 1:

Input: nums = [3,0,1]
Output: 2
Explanation:
n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.

Example 2:

Input: nums = [0,1]
Output: 2
Explanation:
n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.

Example 3:

Input: nums = [9,6,4,2,3,5,7,0,1]
Output: 8
Explanation:
n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.
*/

pub fn missing_number(nums: &Vec<i32>) -> i32 {
    let n: i32 = nums.len() as i32;
    let sumTot: i32 = (n*(n+1))/2;
    let mut sumPartial: i32 = 0;
    for &x in nums {
        sumPartial += x;
    }
    return sumTot-sumPartial;
}

fn main() {
    let nums1 = vec![3, 0, 1];
    let nums2 = vec![0, 1];
    let nums3 = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];

    println!("Missing number in {:?} is {}", nums1, missing_number(&nums1));
    println!("Missing number in {:?} is {}", nums2, missing_number(&nums2));
    println!("Missing number in {:?} is {}", nums3, missing_number(&nums3));
}