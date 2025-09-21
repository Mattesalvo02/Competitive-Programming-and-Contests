
/* 
Given an integer array nums, find the subarray with the largest sum, and return its sum.

 
Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: The subarray [4,-1,2,1] has the largest sum 6.

Example 2:

Input: nums = [1]
Output: 1
Explanation: The subarray [1] has the largest sum 1.

Example 3:

Input: nums = [5,4,-1,7,8]
Output: 23
Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
*/

pub fn max_sub_array(nums: &Vec<i32>) -> i32 {
    let mut currentSum = 0;
    let mut maxSum = i32::MIN;
    let mut maxNum = i32::MIN;

    for &num in nums {
        maxNum = maxNum.max(num);
        currentSum += num;

        if currentSum < 0 {
            currentSum = 0;
        }
        maxSum = maxSum.max(currentSum);
    }
    if maxNum < 0 {
        return maxNum;
    }
    return maxSum;
}

fn main() {
    let nums1 = vec![1, -2, 3, 4, -1, 2, 1, -5, 4];
    let nums2 = vec![-2, -3, -1, -5];
    let nums3 = vec![5, 4, -1, 7, 8];

    println!("Max subarray sum of {:?} is {}", nums1, max_sub_array(&nums1));
    println!("Max subarray sum of {:?} is {}", nums2, max_sub_array(&nums2));
    println!("Max subarray sum of {:?} is {}", nums3, max_sub_array(&nums3));
}