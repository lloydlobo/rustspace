use crate::sqrt::sqrt_f64::Sqrt;
use colored::*;

pub mod sqrt;
fn main() {
    println!("{}", "SQRT This".blue().bold());
    let num: f64 = 9.0;
    let res_float = Sqrt::custom_sqrt(num);
    println!("SQRT_float: {}", res_float);
    let dist = distance();
    println!("distance: {}", dist);
    dist_2d(A::new(1.0, 2.0), B::new(2.0, 4.0));
}

fn distance() -> f64 {
    let vec_a: A = A::new(1.0, 2.0);
    let vec_b: B = B::new(2.0, 4.0);
    println!("vec_a: {:?}, vec_b: {:?}", vec_a, vec_b);
    let (ax, ay): (f64, f64) = (1.0, 2.0);
    let (bx, by): (f64, f64) = (2.0, 4.0);
    let a: f64 = ax - bx;
    let b: f64 = ay - by;
    Sqrt::custom_sqrt(a * a + b * b)
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
    println!("{:?}, {:?}", (a.x, a.y), (b.x, b.y));
}