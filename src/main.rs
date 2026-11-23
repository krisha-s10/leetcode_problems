// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for (x,xval) in nums.iter().enumerate(){
//             for (y,yval) in nums.iter().enumerate(){
//                 if x!=y{
//                     let sum = xval+yval;
//                     if sum == target{
//                         return vec![x as i32, y as i32]
//                     }
//                 }
//             }
//         }
//         unreachable!()
//     }
// }

use std::collections::HashMap;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//
//         let mut hm = HashMap::new();
//
//         for (i, &number) in nums.iter().enumerate(){
//
//             let c = target - number;
//             if let Some(&j) = hm.get(&c){
//                 return vec![j as i32, i as i32]
//             }
//             hm.insert(number,i);
//         }
//         unreachable!();
//     }}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();

        for (i, &number) in nums.iter().enumerate() {
            let c = target - number;
            if let Some(&j) = hm.get(&c) {
                return vec![j as i32, i as i32];
            }
            hm.insert(number, i);
        }
        unreachable!();
    }
}

impl Solution2 {
    pub fn is_palindrome(x: i32) -> bool {
        let b = x.to_string();
        let c = b.len() - 1;
        let mut f = true;
        for i in 0..b.len() / 2 {
            if b.as_bytes()[i] != b.as_bytes()[c - i] {
                f = false;
            }
        }
        f
    }
}

impl Solution3 {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = 0;
        loop {
            if prefix == strs[0].len() {
                return String::from(&strs[0][0..prefix]);
            }

            let sub = &strs[0][0..prefix + 1];

            for word in &strs {
                if prefix == word.len() || &word[0..prefix + 1] != sub {
                    return String::from(&strs[0][0..prefix]);
                }
            }
            prefix += 1;
        }
    }
}

fn find_opposite(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '{' => '}',
        '}' => '{',
        ']' => '[',
        '[' => ']',
        _ => ' ',
    }
}
//Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

impl Solution4 {
    pub fn is_valid(s: String) -> bool {
        let mut open: Vec<char> = vec![];

        for sign in s.chars() {
            let opp = find_opposite(sign);

            if sign == '(' || sign == '{' || sign == '[' {
                open.push(sign);
            } else {
                if (open.len() == 0) {
                    return false;
                }
                if open[open.len() - 1] != opp {
                    return false;
                }
                open.pop();
            }
        }
        open.len() == 0
    }
}

/// Given an integer array nums sorted in non-decreasing order,
/// remove the duplicates in-place such that each unique element appears only once.
/// The relative order of the elements should be kept the same.
impl Solution5 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut counter = 0;
//112
        while counter < nums.len()-1 {
            let x = nums[counter];
            if x == nums[counter+1]{
                nums.remove(counter);
            }
            else {
                counter += 1;
            }
        }
        nums.len() as i32
    }
}


impl Solution6{
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut currentlongest = 0;
        let mut letters: Vec<char> = Vec::new();

        for c in s.chars(){
            if letters.contains(&c){
                while letters.contains(&c) {
                    letters.remove(0);
                }
            }

            letters.push(c);

            if letters.len()>currentlongest{
                currentlongest=letters.len();
            }
        }
        currentlongest as i32
    }
}


impl Solution7 {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let mut a: Vec<i32> = Vec::new();
        a.append(&mut nums1);
        a.append(&mut nums2);
        a.sort();
        let m = a.len()/2;
        if a.len()%2!=0{
            return a[m] as f64
        }
        else{
            return (a[m-1] as f64+a[m] as f64)/2.0;
        }



        //unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_longest_common_prefix() {
        let mut tst:Vec<i32> = vec![
            1,1,2
        ];


        assert_eq!(Solution5::remove_duplicates(&mut tst), 2);
        assert_eq!(tst, vec![1,2])
    }
}

struct Solution {}
struct Solution2 {}
struct Solution3 {}
struct Solution4 {}
struct Solution5 {}

struct Solution6 {}
struct Solution7 {}

fn main() {}
