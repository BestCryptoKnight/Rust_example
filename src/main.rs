use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    println!("please input your score");

    // let secret_number = rand::thread_rng().gen_range(1..101);
    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).expect("Failed to read line!");
    //     print!("your input is {}", input);
    //     let input:u8 = match input.trim().parse() {
    //         Ok(number) => number,
    //         Err(_) => continue,
    //     };

    //     match input.cmp(&secret_number) {
    //         Ordering::Greater => print!("too big"),
    //         Ordering::Less => print!("too small"),
    //         Ordering::Equal => {
    //             print!("win");
    //             break;
    //         },
    //     }
    // }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("failded in");
    let temp:u32 = input.trim().parse().expect("aa");
    let res:u32=
    // assert_eq!(input, "1");
    if input.trim() == "1" {
        test1(input.trim().parse().unwrap())
    } else {
        test1(temp + 10)
    };
    // let res  = test1(4);
    println!("{}", res);

    let x = 5;
    let y = x;

    println!("{}, {}", x, y);
}

fn test() {
    let mut x = [0..5];
    println!("{:?}", x[0]);
    let x = 6;
    println!("{}", x);
    let mut x:u64 = x + 1;
    println!("{}", (x as f32) / 2.0);
    println!("{}", x);
}

fn test1(val: u32) -> u32 {
    let val = val + 8;
    return val;
}
