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
}

struct Point {
    x: i32,
    y: i32,
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
