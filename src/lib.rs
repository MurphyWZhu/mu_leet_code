pub mod mu_leet_code {
    pub mod two_sum {
        //给定一个整数数组nums和一个整数目标值target请你在该数组中找出和为目标值target的那两个整数，
        //并返回它们的数组下标。
        //你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
        //你可以按任意顺序返回答案。
        pub struct Solution {}

        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                let mut result = Vec::new();
                for (i, v) in nums.iter().enumerate() {
                    for (i1, v1) in nums.iter().enumerate() {
                        if v + v1 == target && i != i1 {
                            result.push(i as i32);
                            result.push(i1 as i32);
                            return result;
                        }
                    }
                }
                return result;
            }
            pub fn two_sum_test() {
                let nums = vec![2, 7, 11, 15];
                let target = 9;
                let result = super::two_sum::Solution::two_sum(nums, target);
                println!("{:?}", result)
            }
        }
    }

    pub mod length_of_longest_substring {
        //给定一个字符串，请你找出其中不含有重复字符的最长子串的长度。
        pub struct Solution {}

        impl Solution {
            pub fn length_of_longest_substring(s: String) -> i32 {
                let s_len = s.len();
                let mut tmp: Vec<&str> = Vec::new();
                let (mut result, mut left, mut right): (usize, usize, usize) = (0, 0, 0);
                let mut c;
                while right < s_len {
                    c = match s.get(right..right + 1) {
                        Some(c) => c,
                        None => panic!()
                    };
                    if !tmp.contains(&c) {
                        tmp.push(&c);
                        right += 1;
                        result = result.max(right - left);
                    } else {
                        while tmp[0] != c {
                            tmp.remove(0);
                            left += 1;
                        }
                        tmp.remove(0);
                        left += 1;
                    }
                }
                result as i32
            }
            pub fn length_of_longest_substring_test() {
                let s = " ".to_string();
                println!("{}", super::length_of_longest_substring::Solution::length_of_longest_substring(s));
            }
        }
    }
}