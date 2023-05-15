fn main() {
    // println!("Hello, world!");
    // let mut s = String::from("hello");
    //
    // change(&mut s);
    // println!("string now is= {} ", s);

    // learning matching pattern
    let match_x = Some(10);
    let match_y = 5;

    match match_x {
        Some(5) => println!("matched 5!!"),
        Some(match_y) => println!("matched y!!!  y= {match_y}"),
        _ => println!("matching anything, x={:?}", match_x),
    }
    println!("at the end: x = {:?}, y = {match_y}", match_x);

    // destructure?
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("=====================destructure===================");
    println!("x = {x}, y ={y}");
    println!("px= {}, py= {}", p.x, p.y);

    // structs
    let user1 = User {
        name: String::from("yuse wang"),
        age: 18,
        email: String::from("yusenw2@illinois.edu"),
    };

    let user2 = User { ..user1 };

    println!("user1 name = moved, user2 name = {}", user2.name);

    // closure

    let mut list = vec![1, 2, 3];
    println!("Before closure: {:?}", list);
    let mut test_closure = || list.push(4);
    test_closure();
    //push_a_vector(&mut list);

    println!("After closure: {:?} \n", list);

    println!("===== playing with iterators =====");
    playing_with_iterators();

    // Functional language features playground
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);

    println!(
        "===== userpref= {:?}, givingout= {:?}",
        user_pref1,
        store.giveaway(user_pref1)
    );

    let user_pref2 = None;

    println!(
        "===== userpref= {:?}, givingout= {:?}",
        user_pref2,
        store.giveaway(user_pref2)
    );
}

struct Point {
    x: i32,
    y: i32,
}

struct User {
    name: String,
    age: usize,
    email: String,
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn push_a_vector(nums: &mut Vec<i32>) {
    nums.push(4);
}

fn playing_with_iterators() {
    let mut vec = vec![1, 2, 3, 4, 5];

    for item in vec.iter() {
        println!("{item}");
    }

    let v2: Vec<_> = vec.iter().map(|x| x + 1).collect();

    println!("===== after map x+1 ====");

    for item in v2.iter() {
        println!("{item}");
    }
}

// 13. Functional language Features

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
