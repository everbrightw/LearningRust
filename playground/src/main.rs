fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");

    change(&mut s);
    println!("string now is= {} ", s);
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
