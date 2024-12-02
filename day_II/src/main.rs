use std::fs::File;
use std::io::*;

fn main() {

    let input = File::open("input.txt").unwrap();

    let reader = BufReader::new(input);

    let mut fin_res1:i64=0;
    let mut fin_res2:i64=0;

   for line in reader.lines() {
        let line = line.unwrap();
        let values : Vec<i64> = line.split(" ")
                                 .map(|s| s.trim().parse::<i64>().unwrap())
                                 .collect();
       if safe(&values) {
           fin_res1 +=1;
           fin_res2 +=1;
       }else {
           for i in 0..values.len() {
               let mut temp = values.clone();
               temp.remove(i);
               if safe(&temp) {
                   fin_res2 +=1;
                   break;
               }
           }
       }

       }

    println!("Answer for Puzzle 1={}", fin_res1);
    println!("Answer for Puzzle 1={}", fin_res2);
}

fn safe(line:&Vec<i64>)-> bool {
    if line.windows(2).all(|w| (w[0] - w[1]).abs() <= 3) {
        if line.windows(2).all(|w| w[0] > w[1]) {
            true
        } else if line.windows(2).all(|w| w[0] < w[1]) {
            true
        } else { false }
    } else { false }
}