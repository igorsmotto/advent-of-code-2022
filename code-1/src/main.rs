use std::fs;
use std::collections::BinaryHeap;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let split_contents = contents.split("\n");

    let vec: Vec<&str> = split_contents.collect();
    let mut heap = BinaryHeap::new();
    let mut current_sum = 0;
    for el in vec {
        if el != "" {
            let el_i32: i32 = el.parse().unwrap();
            current_sum += el_i32;
        } else {
            heap.push(current_sum);
            current_sum = 0;
        }
    }
    let max = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{}", max)
}
