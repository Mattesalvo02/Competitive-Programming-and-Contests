
/* 
You are given an array arr of positive integers. Your task is to find all the leaders in the array. 
An element is considered a leader if it is greater than or equal to all elements to its right. 
The rightmost element is always a leader.


Examples:

Input: arr = [16, 17, 4, 3, 5, 2]
Output: [17, 5, 2]
Explanation: Note that there is nothing greater on the right side of 17, 5 and, 2.

Input: arr = [10, 4, 2, 4, 1]
Output: [10, 4, 4, 1]
Explanation: Note that both of the 4s are in output, as to be a leader an equal element is also allowed on the right. side

Input: arr = [5, 10, 20, 40]
Output: [40]
Explanation: When an array is sorted in increasing order, only the rightmost element is leader.

Input: arr = [30, 10, 10, 5]
Output: [30, 10, 10, 5]
Explanation: When an array is sorted in non-increasing order, all elements are leaders.
*/


pub fn leaders(arr: &Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let mut res: Vec<i32> = Vec::new();
    
    // L'ultimo elemento è sempre leader
    res.push(arr[n - 1]);
    let mut max_from_right = arr[n - 1];
    
    // Traverse from right to left
    for i in (0..n - 1).rev() {
        if arr[i] >= max_from_right {
            res.push(arr[i]);
            max_from_right = arr[i];
        }
    }
    
    // At this point, the leaders are in reverse order
    res.reverse();
    
    return res;
}

fn main() {
    let arr1 = vec![16, 17, 4, 3, 5, 2];
    let arr2 = vec![10, 4, 2, 4, 1];
    let arr3 = vec![5, 10, 20, 40];
    let arr4 = vec![30, 10, 10, 5];
    
    println!("Leaders in {:?} are {:?}", arr1, leaders(&arr1));
    println!("Leaders in {:?} are {:?}", arr2, leaders(&arr2));
    println!("Leaders in {:?} are {:?}", arr3, leaders(&arr3));
    println!("Leaders in {:?} are {:?}", arr4, leaders(&arr4));
}