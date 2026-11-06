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

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut hm = HashMap::new();

        for (i, &number) in nums.iter().enumerate(){

            let c = target - number;
            if let Some(&j) = hm.get(&c){
                return vec![j as i32, i as i32]
            }
            hm.insert(number,i);
        }
        unreachable!();
    }}


impl Solution2 {
    pub fn is_palindrome(x: i32) -> bool {
        let b = x.to_string();
        let c = b.len() - 1;
        let mut f = true;
        for i in 0..b.len()/2{
            if b.as_bytes()[i] != b.as_bytes()[c-i]{
                f = false;
            }
        }
        f
    }
}

impl Solution3 {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = 0;
        loop{


            if prefix == strs[0].len() {
                return String::from(&strs[0][0..prefix]);
            }

            let sub = &strs[0][0..prefix + 1];


            for word in &strs{
                if prefix == word.len() || &word[0..prefix+1] != sub{
                    return String::from(&strs[0][0..prefix]);
                }
            }
            prefix+=1;
        }
    }
}

fn find_opposite(c:char)->char {
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
        let mut open:Vec<char> = vec![];

        for sign in s.chars(){
            let opp = find_opposite(sign);

            if sign == '(' || sign == '{' || sign == '[' {
                open.push(sign);
            }
            else {
                if(open.len()==0){
                    return false;
                }
                if open[open.len()-1]!=opp{
                    return false;
                }
                open.pop();

            }
        }
        open.len()==0
    }
}

//Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
impl Solution5 {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut b:Vec<i32> = vec![];
        let mut counter = 0;

        while counter<nums.len(){
            let x = nums[counter];
            let mut check = false;
            for y in &b{
                if *y == x{
                    check = true;
                }
            }
            if check{
                nums.remove(counter);
            }
            else{
                b.push(x);
                counter+=1;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_longest_common_prefix() {
        let strs = vec!["flower".to_string(), "flow".to_string(), "flight".to_string(), ];

        assert_eq!(
            Solution3::longest_common_prefix(strs),
            "fl".to_string()
        );
    }


}

struct Solution{}
struct Solution2{}
struct Solution3{}
struct Solution4{}
struct Solution5{}

fn main() {

}
