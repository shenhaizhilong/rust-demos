mod expression;
mod race;
mod dog_cat;
mod player;
mod point;
mod aa;
mod listnode;
mod hex_demos;
mod async_demo;
mod macros_demo;
mod clap_demo1;
mod Map_demos;
mod ls;
mod pwd;
mod format_demo;
mod str_demos;
mod list;
mod float_demo;
mod int_convert;
mod c_demo;
mod loop_demo;
mod array_demo;
mod const_fn;
mod extend_method;
mod trait_demo;
mod custom_print;
mod bb;
mod copy_demo;
mod drop_demo;

use std::ffi::c_int;
use std::fs::File;
use race::Race;
use dog_cat::Dog;
use dog_cat::Cat;
use crate::dog_cat::Pet;
use crate::player::Player;
use crate::point::Point;
use std::io::{Error, Read};
use log::info;


fn main() {
    log4rs::init_file("data/log_config.yml", Default::default()).unwrap();
    info!("Hello, world!, Info");



    test_for();
    // test_struct();
    test_enum(100);
    test_enum(101);

    sleep_for(-10.0);
    sleep_for(0.8);

    test_while_let();

    test_race();

    cat_dog();

    test_player();
    test_pick();

    test_generic_point();

    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
    let c = duplicate((10, 10));
    println!("{c:?}");
    let d = duplicate(Race::new("a"));
    println!("{d:?}");
    // test_unwrap();
    read_username_from_file().expect("not exist");
}

fn test_unwrap() {
    let file = File::open("data/hello.txt").unwrap();
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("data/hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => { Err(e) }
    }
}

fn read_username_from_file2() -> Result<String, Error> {
    let mut username_file_result = File::open("data/hello.txt")?;
    let mut username = String::new();
    username_file_result.read_to_string(&mut username)?;
    Ok(username)
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}


fn test_generic_point() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
}

fn test_pick() {
    println!("picked a number: {:?}", pick(2, 10, 11));
    println!("picked a tuple: {:?}", pick(3, ("dog", 1), ("cat", 2)));
}

fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else { odd }
}


fn test_player() {
    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = String::from("Tom");
    println!("{}", p2.name);
    println!("{}", p1.name);
    println!("{:?} vs {:?}", p1, p2);
}

fn cat_dog() {
    let cat = Cat { lives: 9 };
    let fido = Dog { name: String::from("Fido"), age: 5 };

    cat.greet();
    fido.greet();
}

fn test_race() {
    let mut r = Race::new("Monaco Grand Prix");
    r.add_lap(70);
    r.add_lap(68);
    r.print_laps();
    r.add_lap(71);
    r.print_laps();
    r.finish();
    // r.add_lap(89);
}

fn test_while_let() {
    let mut name = String::from("abcd");
    while let Some(x) = name.pop() {
        println!("character: {x}")
    }
}

fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else { std::time::Duration::from_millis(500) };
    std::thread::sleep(dur);
    println!("slept for {:?}", dur);
}

#[derive(Debug)]
enum MyResult {
    Ok(i32),
    MyErr(String),
}


// struct Point {
//     x: (u32, u32),
//     y: u32,
// }

fn divide_in_two(n: i32) -> MyResult {
    if n % 2 == 0 {
        return MyResult::Ok(n / 2);
    }
    return MyResult::MyErr(format!("can't divide {n} into two equal parts"));
}

fn test_enum(n: i32) {
    match divide_in_two(n) {
        MyResult::Ok(half) => println!("{n} divide in two is {half}"),
        MyResult::MyErr(msg) => println!("sorry, an error happened: {msg}"),
    }
}

fn test_for() {
    for i in 0..10 {
        println!("Hello, world!, {i}");
    }
}


// fn test_struct() {
//     let point = Point { x: (2, 2), y: 2 };
//
//     match point {
//         Point { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
//         Point { y: 2, x: i } => println!("y = 2, x = {i:?}"),
//         Point { y, .. } => println!("y = {y}, other fields were ignored")
//     }
// }

