// fearless concurrency
use std::thread;
use std::time::Duration;

// channel
use std::sync::mpsc;

// The API of Mutex<T>
use std::sync::Mutex;

use playground::Draw;
use playground::{Button, Screen};

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

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 11,
            height: 2,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

    // Smart pointers

    //let temp_x = 5;
    //let temp_y = MyBox::new(x);

    //assert_eq!(5, x);
    //assert_eq!(5, *y);

    // TODO: keep learning iterators from: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html
    //

    // fearless concurrency

    print!(" =========== FEARLESS CONCURRENCY ========== \n");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // closure ownership

    let closure_v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", closure_v);
    });
    handle2.join().unwrap();

    // message passing
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("xiaomi"),
            String::from("'s lastday"),
        ];

        for temp in vals {
            tx.send(temp).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("state"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // mutex lock
    let m = Mutex::new(5);

    println!("before mutex lock, m = {:?}", m);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result {:}", *counter.lock().unwrap());
    //
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 32,
                height: 32,
                options: vec![],
            }),
            Box::new(Button {
                width: 32,
                height: 32,
                label: String::from("Ok"),
            }),
        ],
    };
    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Select Box drawing");
    }
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// OOP

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    fn update_average(&mut self) {
        //let mut sum = 0;
        //for val in self.list {
        //    sum += val;
        //}

        let total: i32 = self.list.iter().sum();

        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
