use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ptr::read;

fn main() -> io::Result<()> {
    // part one
    // read columns
    let file = File::open("../input.txt")?;
    let reader = io::BufReader::new(file);

    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut iter = line.split_whitespace();
        let val1: i32 = iter.next().unwrap().parse().unwrap();
        let val2: i32 = iter.next().unwrap().parse().unwrap();
        col1.push(val1);
        col2.push(val2);
    }

    // sort it
    col1.sort();
    col2.sort();

    let mut total: Vec<i32> = Vec::new();
    let mut positive_vec: Vec<i32> = Vec::new();

    for (a, b) in col1.iter().zip(col2.iter()) {
        total.push(a - b);
    }

    for num in total.iter() {
        positive_vec.push(num.abs());
    }

    let mut sum = 0;
    for num in positive_vec.iter() {
        sum += num;
    }

    println!("{:?}", sum);

    // part two
    let mut count: i32;
    let mut addTotal: Vec<i32> = Vec::new();

    for a in col1.iter() {
        count = col2.iter().filter(|&x| x == a).count() as i32;
        addTotal.push(a * count);
    }

    let mut sum1: i32 = 0;
    for num in addTotal.iter() {
        sum1 += num;
    }

    println!("{:?}", sum1);
    Ok(())
}
