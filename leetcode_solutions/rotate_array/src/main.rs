fn main() {
    let mut test_vec = vec![1, 2, 3, 4, 5, 6];
    //println!("{:?}", rotate_array(&mut test_vec, 3));
    println!("{:?}", test_vec);
    rotate_array_reverse(&mut test_vec, 2);
    println!("{:?}", test_vec);
}

pub fn rotate_array(nums: &mut Vec<i32>, k: usize) -> Vec<i32> {
    let len = nums.len();
    let mut temp: Vec<i32> = Vec::new();

    for i in 0..len {
        let mut curr = len - k + i;
        if curr >= len {
            curr = curr - len;
        }
        temp.push(nums[curr]);
    }
    temp
}

pub fn rotate_array_reverse(nums: &mut [i32], k: usize) {
    let len = nums.len();
    let k = k % len;
    print!("{}", k);

    // nums.reverse();
    // nums[0..k].reverse();
    // nums[k..len].reverse();
    //
    reverse(nums, 0, len);

    reverse(nums, 0, k);
    reverse(nums, k, len);
}

pub fn reverse(nums: &mut [i32], left: usize, right: usize) {
    let (mut i, mut j) = (left, right);
    while i < j {
        nums.swap(i, j - 1);
        i = i + 1;
        j = j - 1;
    }
}

#[cfg(test)]
mod test {
    use crate::rotate_array_reverse;

    #[test]
    fn basic_test() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let answer = vec![4, 5, 6, 1, 2, 3];
        rotate_array_reverse(&mut vec, 3);

        assert_eq!(vec, answer);
    }

    #[test]
    fn basic_test2() {
        let mut vec = vec![1, 2, 3, 4, 5, 6];
        let answer = vec![5, 6, 1, 2, 3, 4];
        rotate_array_reverse(&mut vec, 2);

        assert_eq!(vec, answer);
    }

    #[test]
    fn basic_test3() {
        let mut vec = vec![1];
        let answer = vec![1];
        rotate_array_reverse(&mut vec, 3);

        assert_eq!(vec, answer);
    }

    #[test]
    fn test_k_greater_than_length() {
        let mut vec = vec![1, 2];
        let answer = vec![2, 1];
        rotate_array_reverse(&mut vec, 3);

        assert_eq!(vec, answer);
    }
}
