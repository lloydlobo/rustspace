use std::{thread::sleep, time::Duration};

use crate::sqrt::sqrt_f64::Sqrt;
use colored::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod sqrt;
fn main() {
    println!("{}", "SQRT This".blue().bold());
    let num: f64  = 999_999_999_999.999;
    let res_float = Sqrt::custom_sqrt(num);
    println!("SQRT_float: {}", res_float);
    let dist = distance();
    info!("distance: {}", dist);
    dist_2d(A::new(1.0, 2.0), B::new(2.0, 4.0));

    fn binary_search_sqrt_f64(value: f64, left: f64, right: f64) -> f64 {
        let root = (left + right) / 2.0;
        if root == left || root == right {
            return root as f64;
        }
        sleep(Duration::from_millis(100));
        // println!("now({}): {:?}", time_ms, now);
        // println!("root {}, value {}, left {}, right {}", root, value, left, right);
        println!("root * root: {}, value: {}", root * root, value);
        if (root * root) as f64 > value {
            println!("{}", "greater".red());
            println!("value {}, left {}, root {}", value, left, root);
            binary_search_sqrt_f64(value, left, root)
        } else {
            println!("{}", "lesser".yellow());
            println!("value {}, root {}, right {}, ", value, root, right);
            binary_search_sqrt_f64(value, root, right)
        }
    }

    pub(crate) fn custom_sqrt(n: f64) -> f64 {
        binary_search_sqrt_f64(n, 0.0, n)
    }

    let res_custom_sqrt = custom_sqrt(num);
    let res_rust_sqrt = num.sqrt();
    println!("sqrt1: {}", res_custom_sqrt);
    println!("sqrt2: {}", res_rust_sqrt);
}

fn distance() -> f64 {
    let vec_a: A = A::new(1.0, 2.0);
    let vec_b: B = B::new(2.0, 4.0);
    info!("vec_a: {:?}, vec_b: {:?}", vec_a, vec_b);
    let (ax, ay): (f64, f64) = (1.0, 2.0);
    let (bx, by): (f64, f64) = (2.0, 4.0);
    let a: f64 = ax - bx;
    let b: f64 = ay - by;
    let result_sqrt = Sqrt::custom_sqrt(a * a + b * b);
    info!("lib | result: {}", result_sqrt);

    result_sqrt
}

#[derive(Debug)]
pub struct A {
    x: f64,
    y: f64,
}
impl A {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
#[derive(Debug)]
pub struct B {
    x: f64,
    y: f64,
}
impl B {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn dist_2d(a: A, b: B) {
    info!("{:?}, {:?}", (a.x, a.y), (b.x, b.y));
}
