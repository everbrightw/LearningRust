fn main() {
    println!("Hello, world!");
    let v = vec![-1, 0, 1, 2, 3, 4, 5, 100];
    let ret = search_insert_position(v, 3);

    print!("{ret} \n");
}

pub fn search_insert_position(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());
    let mut mid: usize = nums.len() / 2;

    while left < right {
        if nums[mid] < target {
            left = mid;
        } else if nums[mid] > target {
            right = mid;
        } else {
            return mid as i32;
        }
        mid = (left + right) / 2;
    }

    return mid as i32;
}
