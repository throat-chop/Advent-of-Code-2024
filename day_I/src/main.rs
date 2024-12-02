use std::fs::File;
use std::io::*;

fn main() {

    let input = File::open("input.txt").unwrap();

    let reader = BufReader::new(input);

    let mut list1:Vec<i64> = Vec::new();
    let mut list2:Vec<i64> = Vec::new();

    let mut fin_res1: i64 = 0;
    let mut fin_res2: i64 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let l1ne = line.split("  ").collect::<Vec<&str>>();
        list1.push(l1ne[0].trim().parse::<i64>().unwrap());
        list2.push(l1ne[1].trim().parse::<i64>().unwrap());
    }

    list1.sort();
    list2.sort();


    for i in 0..list1.len() {
        fin_res1 += (list1[i] - list2[i]).abs();
    }

    for i in 0..list1.len() {
        let mut freq:i64 = 0;
        for j in 0..list2.len() {
            if list1[i] == list2[j] { freq += list1[i]; }
        }
        fin_res2 += freq;
    }

    println!("Answer for Puzzle 1 = {}", fin_res1);
    println!("Answer for Puzzle 2 = {}", fin_res2);

}
