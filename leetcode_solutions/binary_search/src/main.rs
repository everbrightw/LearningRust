use std::cmp::Ordering;

fn main() {
    println!("hello binary search");

    // vector playaroud
    let v: Vec<i32> = vec![-2, 0, 1, 2, 3, 4, 5, 100];

    //let ret = search(v, 4);
    let ret2 = binary_search(v, 100);

    // print!("{ret}");
    print!("{ret2} \n");
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0;
    for num in nums {
        if num == target {
            return i;
        }
        i += 1;
    }
    return -1;
}

pub fn search_with_iter(nums: Vec<i32>, target: i32) -> i32 {
    for (idx, &num) in nums.iter().enumerate() {
        if num == target {
            return idx as i32;
        }
    }
    return -1;
}

pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right): (usize, usize) = (0, nums.len());
    while left < right {
        let mid: usize = (left + right) / 2;
        if nums[mid] > target {
            right = mid;
        } else if nums[mid] < target {
            left = mid;
        } else {
            return mid as i32;
        }
    }
    return -1;
}

pub fn binary_search_matching_pattern(nums: &Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len() - 1);

    while left < right {
        let mid = left + (right - left) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
            Ordering::Equal => {
                return mid as i32;
            }
        }
    }
    -1
}

#[cfg(test)]

mod test {
    use crate::*;

    #[test]
    fn basic_test() {
        let vec_test = vec![-1, 0, 1, 2, 3, 4, 5, 100];
        assert_eq!(binary_search_matching_pattern(&vec_test, -1), 0);
        assert_eq!(binary_search_matching_pattern(&vec_test, 0), 1);
        assert_eq!(binary_search_matching_pattern(&vec_test, 1), 2);
        assert_eq!(binary_search_matching_pattern(&vec_test, 2), 3);
        assert_eq!(binary_search_matching_pattern(&vec_test, 3), 4);
        assert_eq!(binary_search_matching_pattern(&vec_test, 101), -1);
    }
}
