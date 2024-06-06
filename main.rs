use std::cmp::Ordering;

// 1. Check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    s_chars == s_chars.iter().rev().cloned().collect::<Vec<char>>()
}

// 2. Index of the first occurrence of a number in a sorted array
fn first_occurrence(arr: &[i32], x: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == x {
            return Some(index);
        }
    }
    None
}

// 3. Shortest word in a string
fn shortest_word(s: &str) -> String {
    s.split_whitespace().min_by_key(|word| word.len()).unwrap_or("").to_string()
}

// 4. Check if a number is prime
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 5. Median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] as f64 + arr[len / 2] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// 6. Longest common prefix
fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let mut prefix = strs[0].to_string();
    for s in &strs[1..] {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return "".to_string();
            }
            prefix.pop();
        }
    }
    prefix
}

// 7. Kth smallest element in an array
fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    arr.sort();
    arr[k - 1]
}

// 8. Maximum depth of a binary tree
#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<&Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => 1 + std::cmp::max(max_depth(node.left.as_ref()), max_depth(node.right.as_ref())),
        None => 0,
    }
}

// Rust specific implementations

// 9. Reverse a string
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// 10. Check if a number is prime
// This is already implemented as is_prime

// 11. Merge two sorted arrays
fn merge_sorted_arrays(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    merged.extend_from_slice(&a[i..]);
    merged.extend_from_slice(&b[j..]);
    merged
}

// 12. Maximum subarray sum
fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];
    for &num in &nums[1..] {
        current_sum = current_sum.max(num).max(current_sum + num);
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

fn main() {
    // Test is_palindrome
    println!("'racecar' is palindrome: {}", is_palindrome("racecar"));
    println!("'hello' is palindrome: {}", is_palindrome("hello"));

    // Test first_occurrence
    let arr = [1, 2, 3, 4, 5, 5, 6];
    println!("First occurrence of 5: {:?}", first_occurrence(&arr, 5));

    // Test shortest_word
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("Shortest word: {}", shortest_word(sentence));

    // Test is_prime
    println!("7 is prime: {}", is_prime(7));
    println!("4 is prime: {}", is_prime(4));

    // Test median
    let sorted_array = [1, 2, 3, 4, 5];
    println!("Median: {}", median(&sorted_array));

    // Test longest_common_prefix
    let strs = vec!["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strs));

    // Test kth_smallest
    let mut unsorted_array = [7, 10, 4, 3, 20, 15];
    println!("3rd smallest element: {}", kth_smallest(&mut unsorted_array, 3));

    // Test max_depth
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode::new(4))),
            right: Some(Box::new(TreeNode::new(5))),
        })),
        right: Some(Box::new(TreeNode::new(3))),
    }));
    println!("Max depth of the tree: {}", max_depth(root.as_ref()));

    // Test reverse_string
    println!("Reverse of 'hello': {}", reverse_string("hello"));

    // Test merge_sorted_arrays
    let a = [1, 3, 5];
    let b = [2, 4, 6];
    println!("Merged array: {:?}", merge_sorted_arrays(&a, &b));

    // Test max_subarray_sum
    let nums = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("Maximum subarray sum: {}", max_subarray_sum(&nums));
}
