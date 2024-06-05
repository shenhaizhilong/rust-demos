use std::rc::Rc;
use std::sync::mpsc;
use std::{io, thread};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use rand::Rng;
use crate::change_struct_field::Demo;
use crate::droppable::Droppable;
use crate::point::Point;

mod point;
mod droppable;
mod foo;
mod bar;
mod borrow_demos;
mod spawn_demo;
mod string_demo;
mod person;
mod pair_demos;
mod lifetime_demos;
mod tokio_demos;
mod tokio_write_demos;
mod iter_demo;
mod closures_demo;
mod box_demo;
mod option_demo;
mod struct_demo;
mod struct_demo2;
mod Default_trait_demos;
mod rc_demo;
mod feature_demos;
mod change_struct_field;
mod integer_demos;
mod color_string;

static BANNER: &str = "hello, rust!";

fn print_line() {
    println!("***********************")
}


fn main() {
    // tokio_demos::main();
    // tokio_write_demos::main();

    color_string::welcome();
    print_line();

    let mut  d = Demo {
        name: "tom".to_string(),
        email: RefCell::new("tom@126.com".to_string()),
    };

    d.name = "alice".to_string();
    println!("{:?}", d);

    test();
    test_while();
    test_const();
    test_str_format();
    //   BANNER = "abc";
    println!("{BANNER}");

    let res: i16 = inter_product(1, 2, 3);
    println!("result: {res}");
    test_strings();

    let x = 10;
    let y = 20;
    take_u32(x);
    take_i8(y);
    take_u32(y as u32);
    // take_u32(y);

    test_assert();

    test_array();

    test_tuple();

    test_match('x');
    test_match('1');
    test_match('!');
    test_match('q');
    test_match('a');
    test_match('s');

    test_tuple_match((-1, 2));

    test_tuple_match((1, 2));

    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    println!("matrix: {:#?}", matrix);
    let t = transpose(matrix);
    println!("transposed: {:#?}", t);

    test_from();

    test_into();

    test_point_add();

    test_casting();
    test_ownership();

    test_drop();
    test_rc();

    test_slice();

    test_fib();

    foo::do_something();
    bar::do_something();

    test_thread();

    test_channel();

    // test_unbound_channel();
    test_bound_channel();

    // test_rand();

    test_map();
}

fn test_rand() {
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);
    println!("guess number game");

    while true {
        println!("please input a num");
        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str).expect("Failed to read line");
        println!("You guessed: {guess_str}");
        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}

fn test_bound_channel() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: send Message: {i}");
        }
        println!("{thread_id:?} : done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: get {msg}");
    }
}

fn test_unbound_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: send Message: {i}");
        }
        println!("{thread_id:?} : done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: get {msg}");
    }
}

fn test_channel() {
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(10).unwrap();
    println!("received: {:?}", rx.recv());
    println!("received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("received: {:?}", rx.recv());
}

fn test_thread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Count in thread: {i}");
            thread::sleep(Duration::from_millis(5));
        }
    });
    handle.join().expect("TODO: panic message");

    for i in 1..5 {
        println!("Mail thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next: u32 = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn test_fib() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(10) {
        println!("fib:({i}, {n})");
    }
}


fn test_slice() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    a[3] = 10;
    let s: &[i32] = &a[1..4];
    println!("s: {s:?}");
}

fn test_rc() {
    let mut a = Rc::new(10);
    let mut b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}

fn test_drop() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block b");
        }
        println!("Exiting block a");
    }

    drop(a);
    println!("Exiting test_drop");
}


// fn say_hello1(name: String) {
//     println!("Hello {name}")
// }

// 引用传递
fn say_hello2(name: &String) {
    println!("Hello {name}")
}

fn test_ownership() {
    {
        let p = Point { x: 10, y: 20 };
        println!("x:{}", p.x);
    }
    // println!("y:{}", p.y);

    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");

    let name = String::from("Alice");
    // 第一次调用 say_hello1 时， main 放弃了 name 的所有权。此后， name 不能再在 main 中使用
    // 为 name 分配的堆内存将在 say_hello 函数结束时释放
    // say_hello1(name);
    // say_hello(name);

    // 如果 main 将 name 作为引用传递 ( &name ) 并且 say_hello 接受引用作为参数，则它可以保留所有权。
    say_hello2(&name);
    say_hello2(&name);

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

fn test_casting() {
    let a: i64 = 1000;
    println!("as i32: {}", a as i32);
    println!("as i16: {}", a as i16);
    println!("as i8: {}", a as i8);
}

fn test_point_add() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 11, y: 12 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2)
}

/**
如果溢出会抛出异常， 例如     let res: i16 = inter_product(10000, 20000, 30000);
 **/
fn inter_product(a: i16, b: i16, c: i16) -> i16 {
    return a * b + b * c + a * c;
}

/**
String - a modifiable, owned string.
&str - a read-only string. String literals have this type.
 **/
fn test_strings() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence: String = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);

    println!("final senctence: {}", sentence);
    println!("{:?}", &sentence[0..5])
}

/**
tes
 **/

fn test_while() {
    let mut x: i32 = 6;
    print!("{x}");       // Macro for printing, like printf
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}")
    }
    println!();
}

fn test_map() {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);

    let r = match map.get("one") {
        None => 0,
        Some(val) => val + 1
    };

    println!("{r}");
    println!("{:?}", map);
}

fn test() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

fn test_const() {
    const STR1: &str = "abc";
    println!("{STR1}")
}

fn test_str_format() {
    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);
}

fn take_u32(x: u32) {
    println!("u32: {}", x);
}

fn take_i8(x: i8) {
    println!("i8: {}", x);
}

fn test_assert() {
    // let x = 20;
    // let y = 3.14;
    // no implementation for `{integer} == {float}`
    // assert_eq!(x, y);
}

fn test_array() {
    let mut a: [i8; 10] = [42; 10];
    a[0] = 5;
    println!("a: {a:?}");

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
}

fn test_tuple() {
    // let x = (7, true);
    let t: (i8, bool) = (7, false);
    println!("t0: {}", t.0);
    println!("t1: {}", t.1);
}

fn test_match(input: char) {
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("lowercase: {key}"),
        _ => println!("Something else"),
    }
}

fn test_tuple_match(point: (i32, i32)) {
    match point {
        (0, _) => println!("on y axis"),
        (_, 0) => println!("on x axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below of X axis"),
        _ => println!("first quadrant"),
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    return result;
}

fn test_from() {
    let s = String::from("Hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");
}

fn test_into() {
    let s: String = "Hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123_i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}