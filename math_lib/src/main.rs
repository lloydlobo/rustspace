use std::{thread::sleep, time::Duration};

use crate::sqrt::sqrt_f64::Sqrt;
use colored::*;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod sqrt;
fn main() {
    println!("{}", "SQRT This".blue().bold());
    let dist = distance();
    println!("distance: {}", dist);
    dist_2d(A::new(1.0, 2.0), B::new(2.0, 4.0));
}

fn distance() -> f64 {
    let va: A = A::new(1.0, 2.0);
    let vb: B = B::new(2.0, 4.0);
    println!("vec_a: {:?}, vec_b: {:?}", va, vb);
    let (ax, ay): (f64, f64) = (1.0, 2.0);
    let (bx, by): (f64, f64) = (2.0, 4.0);
    let a: f64 = ax - bx;
    let b: f64 = ay - by;
    let result_sqrt = Sqrt::custom_sqrt(a * a + b * b);
    let ab_x = (va.0 - vb.0);
    let ab_y = (va.1 - vb.1);
    let a_b = ab_x * ab_x + ab_y * ab_y;
    let distance_std = a_b.sqrt();

    println!("lib | result: {}", result_sqrt);

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
