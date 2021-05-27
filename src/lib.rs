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
                let result = self::Solution::two_sum(nums, target);
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
                println!("{}", self::Solution::length_of_longest_substring(s));
            }
        }
    }

    pub mod median_of_two_sorted_arrays {
        //给定两个大小分别为m和n的正序（从小到大）数组nums1和nums2。请你找出并返回这两个正序数组的中位数 。
        pub struct Solution {}

        impl Solution {
            pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
                let mut tmp = vec![0, 0];
                let (nums1_len, nums2_len) = (nums1.len(), nums2.len());
                let median = (nums1_len + nums2_len) / 2;
                let (mut p1, mut p2): (usize, usize) = (0, 0);
                for i in 0..median + 1 {
                    match nums1.get(p1) {
                        Some(n1) => {
                            match nums2.get(p2) {
                                Some(n2) => {
                                    if n2 > n1 {
                                        tmp.push(n1.clone());
                                        tmp.remove(0);
                                        p1 += 1;
                                    } else {
                                        tmp.push(n2.clone());
                                        tmp.remove(0);
                                        p2 += 1;
                                    }
                                }
                                None => {
                                    tmp.push(n1.clone());
                                    tmp.remove(0);
                                    p1 += 1;
                                }
                            }
                        }
                        None => {
                            match nums2.get(p2) {
                                Some(n2) => {
                                    tmp.push(n2.clone());
                                    tmp.remove(0);
                                    p2 += 1;
                                }
                                None => return 0.0
                            }
                        }
                    }
                }
                if (nums1_len + nums2_len) % 2 != 0 {
                    tmp[1] as f64
                } else {
                    (tmp[0] as f64 + tmp[1] as f64) / 2.0
                }
            }
            pub fn find_median_sorted_arrays_test() {
                let nums1 = vec![1, 3];
                let nums2 = vec![2];
                println!("{}", self::Solution::find_median_sorted_arrays(nums1, nums2));
            }
        }
    }
}