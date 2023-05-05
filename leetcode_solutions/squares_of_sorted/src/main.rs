fn main() {
    println!("Hello, world!");
    let mut test_vec = vec![2, 1, 3, 4, 5, 6, -1];
    println!("{:?}", test_vec);
    sort_array(&mut test_vec);
    println!("{:?}", test_vec);
    square_array(&mut test_vec);
    println!("{:?}", test_vec);
}

// bubble_sort
pub fn sort_array(nums: &mut Vec<i32>) {
    let len = nums.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

pub fn square_array(nums: &mut Vec<i32>) {
    sort_array(nums);

    for i in 0..nums.len() {
        nums[i] = nums[i] * nums[i];
    }
}

#[cfg(test)]
mod test {
    // use crate::*;

    use crate::sort_array;

    #[test]
    fn baisc_test() {
        let mut test_vec = vec![2, 1, 3, 4, 5, 6, -1];
        let mut test_vec2 = vec![2, 4, 3, 4, 5, 6, -1, 100];
        let mut test_vec3 = vec![2, 1, 3, -3, 2, 6, -1];
        let mut test_vec4 = vec![2];
        let mut test_vec5: Vec<i32> = Vec::new();
        let mut test_vec6 = vec![6, -1];
        let mut test_vec7 = vec![2, 3];

        assert_eq!(sort_array(&mut test_vec), test_vec.sort());
        assert_eq!(sort_array(&mut test_vec2), test_vec2.sort());
        assert_eq!(sort_array(&mut test_vec3), test_vec3.sort());
        assert_eq!(sort_array(&mut test_vec4), test_vec4.sort());
        assert_eq!(sort_array(&mut test_vec5), test_vec5.sort());
        assert_eq!(sort_array(&mut test_vec6), test_vec6.sort());
        assert_eq!(sort_array(&mut test_vec7), test_vec7.sort());
    }
}
