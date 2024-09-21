use itertools::Itertools;
use std::fs::{self, read_to_string};

fn main() {
    let first = "first-b64.txt";
    let second = "second-b64.txt";

    let s1 = fs::read_to_string(first).unwrap();
    let s2 = fs::read_to_string(second).unwrap();
    let mut ses: Vec<char> = vec![];
    let mut fes: Vec<char> = vec![];
    let mut previous = 0;
    for (i, (f, s)) in s1.chars().zip(s2.chars()).enumerate() {
        if f != s {
            panic!("{i}");
            // fes.push(f);
            // ses.push(s);
            let inc = previous == i - 1;
            if !inc {
                panic!("{i}");
            }
            println!("{i}-{}", previous == i - 1);
        } else {
            // print!("{}-{} ", f, i);
        }
        previous = i;
    }
    // println!("{fes:?}");
}
