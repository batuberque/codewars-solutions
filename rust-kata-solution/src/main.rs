mod exam;

use crate::exam::{print_publications, Book, Magazine, Publication};

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::usize;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

trait Ext {
    fn first(self, predicate: impl Fn(u8) -> bool) -> u8
    where
        Self: Sized;
}
impl Ext for RangeInclusive<u8> {
    fn first(self, predicate: impl Fn(u8) -> bool) -> u8
    where
        Self: Sized,
    {
        for element in self {
            if predicate(element) {
                return element;
            }
        }
        return 0;
    }
}

fn find_missing_letter(chars: &[char]) -> char {
    (chars[0] as u8..=*chars.last().unwrap() as u8).first(|x| !chars.contains(&(x as char))) as char
}

struct Solution;

impl Solution {
    // pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     if let Some(node) = root {
    //         let node = node.borrow();
    //         let left_val = node.left.as_ref().map_or(0, |n| n.borrow().val);
    //         let right_val = node.right.as_ref().map_or(0, |n| n.borrow().val);
    //         return node.val == left_val + right_val;
    //     }
    //     false
    // }
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let array = nums
            .into_iter()
            .map(|x| {
                sum += x;
                sum
            })
            .collect();
        array
    }

    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let sum_of_wealth: Vec<i32> = accounts
            .iter()
            .map(|customer| {
                let wealth = customer.iter().sum();
                println!("Customer wealth: {}", wealth);
                wealth
            })
            .collect();

        let max_wealth = sum_of_wealth.iter().max();
        let max_wealth_value = max_wealth.unwrap_or(&0);
        println!("Maximum wealth: {}", max_wealth_value);

        *max_wealth_value
    }

    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n)
            .map(|i| match (i % 3, i % 5) {
                (0, 0) => "FizzBuzz".to_string(),
                (0, _) => "Fizz".to_string(),
                (_, 0) => "Buzz".to_string(),
                _ => i.to_string(),
            })
            .collect()
    }

    // recursive solution
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 {
            0
        } else if num % 2 == 0 {
            1 + Self::number_of_steps(num / 2)
        } else {
            1 + Self::number_of_steps(num - 1)
        }
    }

    // bite solution:
    pub fn number_of_steps2(num: i32) -> i32 {
        match num > 1 {
            true => (num.count_zeros() - num.leading_zeros() + 2 * num.count_ones() - 1) as i32,
            false => num,
        }
    }

    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = slow.unwrap().next;
            fast = fast.unwrap().next.unwrap().next;
        }
        slow
    }

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut char_count: HashMap<char, i32> = HashMap::new();

        for ch in magazine.chars() {
            *char_count.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            if let Some(count) = char_count.get_mut(&ch) {
                if *count > 0 {
                    *count -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(max_count, current_count), &x| {
                if x == 1 {
                    let new_current_count = current_count + 1;
                    (max(max_count, new_current_count), new_current_count)
                } else {
                    (max_count, 0)
                }
            })
            .0
    }

    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter(|&x| x.to_string().len() % 2 == 0)
            .count() as i32
    }

    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut array: Vec<i32> = nums.into_iter().map(|x| x.pow(2)).collect();
        array.sort();
        array
    }

    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                if i + 1 < arr.len() {
                    for j in (i + 1..arr.len()).rev() {
                        arr[j] = arr[j - 1];
                    }

                    if i + 1 < arr.len() {
                        arr[i + 1] = 0;
                    }

                    i += 2;
                } else {
                    break;
                }
            } else {
                i += 1;
            }
        }
    }

    pub fn dublicate_zeros_2(arr: &mut Vec<i32>) {
        let mut result: Vec<i32> = arr
            .iter()
            .flat_map(|&x| if x == 0 { vec![0, 0] } else { vec![x] })
            .collect();

        result.truncate(arr.len());
        arr.clone_from_slice(&result);
    }

    pub fn merge_solution_1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);
        nums1.append(nums2);

        nums1.sort();

        println!("{:?}", nums1);
    }

    pub fn merge_solution_2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }

    // remove all value for vector
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }

    // remove duplicate for vector
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut seen = HashSet::new();

        for &value in &arr {
            if seen.contains(&(2 * value)) || (value % 2 == 0 && seen.contains(&(value / 2))) {
                return true;
            }
            seen.insert(value);
        }

        false
    }

    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let n = arr.len();
        if n < 3 {
            return false;
        }

        let mut i = 0;

        // Ascend to the peak
        while i + 1 < n && arr[i] < arr[i + 1] {
            i += 1;
        }

        // Peak can't be the first or the last
        if i == 0 || i == n - 1 {
            return false;
        }

        // Descend from the peak
        while i + 1 < n && arr[i] > arr[i + 1] {
            i += 1;
        }

        i == n - 1
    }

    pub fn replace_element(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        for i in (0..arr.len()).rev() {
            let current = arr[i];
            arr[i] = max;
            if current > max {
                max = current;
            }
        }
        arr
    }

    // dont forget, position_min()
    pub fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
        let mut numbers_vec = numbers.to_vec();
        if let Some(min_index) = numbers_vec
            .iter()
            .position(|&x| x == *numbers.iter().min().unwrap())
        {
            numbers_vec.remove(min_index);
        }
        numbers_vec
    }

    pub fn roman_as_num(roman: &str) -> u64 {
        let roman_values: HashMap<char, u64> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let mut result = 0;
        let mut prev_value = 0;
        for c in roman.chars().rev() {
            let value = *roman_values.get(&c).unwrap();
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }
        result
    }

    pub fn solution_break_camelcase(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!(" {}", c)
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    pub fn alphabet_position(text: &str) -> String {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        text.chars()
            .filter_map(|c| {
                if c.is_ascii_alphabetic() {
                    Some(
                        (alphabet
                            .iter()
                            .position(|&x| x == c.to_ascii_lowercase())
                            .unwrap()
                            + 1)
                        .to_string(),
                    )
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn how_much_i_love_you(nb_petals: u16) -> &'static str {
        let vecs = vec![
            "I love you",
            "a little",
            "a lot",
            "passionately",
            "madly",
            "not at all",
        ];

        let index = (nb_petals - 1) % vecs.len() as u16;

        vecs[index as usize]
    }

    pub fn maps(values: &Vec<i32>) -> Vec<i32> {
        let mut array = values.into_iter().map(|value| value * 2).collect();
        array
    }

    pub fn monkey_count(n: i32) -> Vec<i32> {
        let array = (1..=n).collect();
        array
    }

    pub fn grow(nums: Vec<i32>) -> i32 {
        let change = nums.iter().product();
        change
    }

    pub fn powers_of_two(n: u8) -> Vec<u128> {
        (0..=n).map(|exp| 2u128.pow(exp as u32)).collect()
    }

    pub fn find_multiples1(n: u32, limit: u32) -> Vec<u32> {
        let array = (n..=limit).map(|value| value + n).collect();
        array
    }

    pub fn find_multiples(n: u32, limit: u32) -> Vec<u32> {
        (1..=limit / n).map(|i| n * i).collect()
    }

    pub fn count_sheep(sheep: &[bool]) -> u8 {
        sheep.iter().filter(|&&value| value).count() as u8
    }

    pub fn three_vowels(word: &str) -> bool {
        let mut vowel_count = 0;
        for c in word.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowel_count += 1;
                    if vowel_count >= 3 {
                        return true;
                    }
                }
                _ => vowel_count = 0,
            }
        }
        false
    }

    pub fn slice_plus_slice(xs: &[i32], ys: &[i32]) -> i32 {
        //xs.iter().chain(ys).sum()

        let sum_xs: i32 = xs.iter().sum();
        let sum_ys: i32 = ys.iter().sum();

        sum_xs + sum_ys
    }

    pub fn invert(values: &[i32]) -> Vec<i32> {
        let mut val = values
            .into_iter()
            .map(|value| {
                if value > &0 {
                    return value * -1;
                } else {
                    return value.abs();
                }
            })
            .collect();
        return val;
    }

    pub fn collinearity(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        match (x1, y1, x2, y2) {
            (0, 0, 0, 0) => true,
            (0, 0, _, _) => true,
            (_, _, 0, 0) => true,
            (_, _, _, _) => {
                if let (0, 0) | (_, 0) | (0, _) = (x1, y1) {
                    (x1 == x2 && y1 == y2) || (x1 == 0 && x2 == 0) || (y1 == 0 && y2 == 0)
                } else {
                    x1 * y2 == x2 * y1
                }
            }
        }
    }

    pub fn pivot_index(num: Vec<i32>) -> i32 {
        let total_sum = num.iter().sum::<i32>();
        let mut left_sum = 0;

        for (i, &num) in num.iter().enumerate() {
            if left_sum == (total_sum - left_sum - num) {
                return i as i32;
            }
            left_sum += num;
        }

        -1
    }

    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let (mut max_index, mut max_number) = (0, nums[0]);
        for (i, &num) in nums.iter().enumerate() {
            if num > max_number {
                max_number = num;
                max_index = i;
            }
        }

        for &num in nums.iter() {
            if max_number < 2 * num && num != max_number {
                return -1;
            }
        }

        max_index as i32
    }

    pub fn solve_for_y(x: f64) -> f64 {
        (26.0 - 3.0 * x) / (x - 5.0)
    }

    pub fn multiples_three_or_five(num: i32) -> i32 {
        if num < 0 {
            0;
        }

        let value = (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum();

        value
    }

    pub fn parse_deadfish(code: &str) -> Vec<i32> {
        let mut return_array = vec![];
        let mut value: i32 = 0;

        for c in code.chars() {
            match c {
                'i' => value += 1,
                'd' => value -= 1,
                's' => value = value.pow(2),
                'o' => return_array.push(value),
                _ => (),
            }
        }

        return_array
    }

    // my solution
    pub fn find_outlier(values: &[i32]) -> i32 {
        let arr: Vec<_> = values
            .iter()
            .filter_map(|&x| if x % 2 == 0 { Some(x) } else { None })
            .collect();

        if arr.len() == 1 {
            arr[0]
        } else {
            *values.iter().find(|&&x| x % 2 != 0).unwrap()
        }
    }

    // 2.solution
    pub fn find_outlier_s2(values: &[i32]) -> i32 {
        let (even, odd): (Vec<_>, Vec<_>) = values.iter().partition(|&x| x % 2 == 0);
        if even.len() == 1 {
            *even.get(0).unwrap()
        } else {
            *odd.get(0).unwrap()
        }
    }

    // functional solution
    pub fn find_missing_letter(chars: &[char]) -> char {
        let first_char = chars[0] as u8;
        for (i, &char) in chars.iter().enumerate() {
            if char as u8 != first_char + i as u8 {
                return (first_char + i as u8) as char;
            }
        }
        unreachable!()
    }

    pub fn wave(s: &str) -> Vec<String> {
        let mut return_array = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        for (i, &char) in chars.iter().enumerate() {
            if char.is_alphabetic() {
                let mut wave_string = chars.clone();
                wave_string[i] = char.to_uppercase().nth(0).unwrap();
                return_array.push(wave_string.into_iter().collect());
            }
        }

        return_array
    }

    pub fn hamming(n: usize) -> u64 {
        let mut hamming_numbers = vec![1u64; n];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);
        let (mut next_multiple_of_2, mut next_multiple_of_3, mut next_multiple_of_5) = (2, 3, 5);

        for i in 1..n {
            let next_hamming_number = next_multiple_of_2
                .min(next_multiple_of_3)
                .min(next_multiple_of_5);
            hamming_numbers[i] = next_hamming_number;

            if next_hamming_number == next_multiple_of_2 {
                i2 += 1;
                next_multiple_of_2 = hamming_numbers[i2] * 2;
            }
            if next_hamming_number == next_multiple_of_3 {
                i3 += 1;
                next_multiple_of_3 = hamming_numbers[i3] * 3;
            }
            if next_hamming_number == next_multiple_of_5 {
                i5 += 1;
                next_multiple_of_5 = hamming_numbers[i5] * 5;
            }
        }

        hamming_numbers[n - 1]
    }

    pub fn hamming_solve_2(n: usize) -> u64 {
        let mut s: BTreeSet<u64> = BTreeSet::from([1]);
        for _ in 1..n {
            let x = *s.iter().next().unwrap();
            s.remove(&x);
            s.insert(x.checked_mul(2).unwrap_or(u64::MAX));
            s.insert(x.checked_mul(3).unwrap_or(u64::MAX));
            s.insert(x.checked_mul(5).unwrap_or(u64::MAX));
        }
        *s.iter().next().unwrap()
    }
}

fn main() {
    // let array = Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]);
    // println!("{:?}", array);

    // let result = Solution::fizz_buzz(15);
    // println!("{:?}", result);

    // let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    // let mut nums2 = vec![2, 5, 6];

    // let result2 = Solution::merge_solution_1(&mut nums1, 3, &mut nums2, 3);
    // println!("{:?}", result2);

    // let mut valid_pairs = 0;

    // // y için makul bir aralık belirleyelim. Örneğin, -100 ile 100 arasında.
    // for y in -100..=100 {
    //     let numerator = 5 * y + 26; // Pay
    //     let denominator = y + 3; // Payda

    //     // Paydanın sıfır olmadığından ve payın paydaya tam bölünebildiğinden emin olalım.
    //     if denominator != 0 && numerator % denominator == 0 {
    //         let x = numerator / denominator;
    //         println!("Geçerli bir çift (x, y): ({}, {})", x, y);
    //         valid_pairs += 1;
    //     }
    // }

    // println!("Toplam geçerli tam sayı çifti sayısı: {}", valid_pairs);

    let try_something = Solution::parse_deadfish("iiisdoso");
    println!("{:?}", try_something);
}
