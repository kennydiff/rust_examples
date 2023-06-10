#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

use std::str::FromStr;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if let Some(first_arg) = args.get(0) {
        println!("第0个参数是: {}", first_arg);
    } else {
        println!("没有传递任何参数。");
    }
    // println!(env::args().get(0));  // K_23530 打印args()的第一个参数



    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {  // K_23530 skip(1) : 跳过第一个参数(程序文件名自己)
        numbers.push(u64::from_str(&arg)
                     .expect("error parsing argument"));
    }

    if numbers.len() == 0 {  // K_23530 检查vector是否不为空，如果为空就退出程序
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {  // K_23530 `&x`是借用`x`的引用
        d = gcd(d, *m);  // K_23530 `*m`返回引用`m`指向的值
    }

    println!("The greatest common divisor of {:?} is {}",  // K_23530 最大公约数是...
             numbers, d);
}
