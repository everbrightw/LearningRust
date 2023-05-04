fn main() {
    println!("Hello, world!");
    let v = vec![-1, 0, 1, 2, 3, 4, 5, 100];

    // let test2 = Solution::search_insert_logn(v, -1);
    let test = Solution::search_insert_logn(&vec![-1, 0, 2, 3, 4, 5, 100], -1);

    print!("{test} \n");
}

struct Solution {}

impl Solution {
    // O(n) solution
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, &num) in nums.iter().enumerate() {
            if num >= target {
                return i as i32;
            }
        }
        return nums.len() as i32;
    }

    // O(logn) solution
    pub fn search_insert_logn(nums: &Vec<i32>, target: i32) -> i32 {
        //let (mut left, mut right) = (0, nums.len() - 1);
        let mut left = 0;
        let mut right = nums.len();

        let mut mid = 0;

        while left < right {
            mid = left + (right - left) / 2;
            if nums[mid] > target {
                // right could underflow, since the langauge infer the right to be usize
                // but if mid==0, there will be an underflow occur
                right = mid;
            } else if nums[mid] < target {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }
        return mid as i32;
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn search_insert_basic_test() {
        assert_eq!(
            Solution::search_insert(vec![-1, 0, 1, 2, 3, 4, 5, 100], 101),
            8
        );
        assert_eq!(
            Solution::search_insert(vec![-1, 0, 1, 2, 3, 4, 5, 100], -2),
            0
        );
    }

    #[test]
    fn search_insert_logn_basic_test() {
        let test_vec = vec![-1, 0, 1, 2, 3, 4, 5, 100];
        assert_eq!(Solution::search_insert_logn(&test_vec, 5), 6);
        assert_eq!(Solution::search_insert_logn(&test_vec, -2), 0);
        assert_eq!(Solution::search_insert_logn(&test_vec, 100), 7);
        assert_eq!(Solution::search_insert_logn(&test_vec, 6), 7);
    }
}
