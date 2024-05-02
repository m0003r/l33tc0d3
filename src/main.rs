use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

use std::ops::Range;

mod t121;
mod t122;
mod t123;
mod t167;
mod t189;
mod t238;
mod t27;
mod t274;
mod t392;
mod t55;
mod t58;
mod t76;
mod t88;
mod t31;

#[derive(Eq, PartialEq)]
struct Answer {
    x: usize,
    y: usize,
    z: usize,
    dist: usize,
}

impl Answer {
    fn new(x: usize, y: usize, z: usize, target: &Range<usize>) -> Option<Self> {
        let r = x * y * z;
        if !target.contains(&r) {
            return None;
        }

        let d5 = r % 10;
        let r4 = r / 10;
        let d4 = r4 % 10;
        let r3 = r4 / 10;
        let d3 = r3 % 10;
        let r2 = r3 / 10;
        let d2 = r2 % 10;
        let r1 = r2 / 10;
        let d1 = r1 % 10;

        if d2 != d5 || d3 != d4 {
            return None;
        }
        if d1 == d2 || d2 == d3 || d1 == d3 {
            return None;
        }

        let dist = z.abs_diff(y) + y.abs_diff(x);
        Some(Answer { x, y, z, dist })
    }
}

impl PartialOrd<Self> for Answer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Answer {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}×{}×{} = {} (dist = {})",
            self.x,
            self.y,
            self.z,
            self.x * self.y * self.z,
            self.dist
        )
    }
}

fn main() {
    let _buf = String::with_capacity(10);
    let target = 10000..100000;
    let mut output = BinaryHeap::with_capacity(40000);
    for x in 2..(target.end) {
        for y in (x + 1)..(target.end / x) {
            let z = y - x + y;
            //for z in (y + 1)..(target.end / (x * y)) {
                if let Some(r) = Answer::new(x, y, z, &target) {
                    output.push(r)
                }
            //}
        }
    }

    eprintln!("Output len: {}", output.len());

    for _ in 0..50 {
        if let Some(d) = output.pop() {
            println!("{}", d)
        }
    }
}
