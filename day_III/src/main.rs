use std::fs;

fn main(){

    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    let mut fin_res1:i64=0;
    let mut fin_res2:i64=0;

    fin_res1 += exprcompute(&input);



    println!("Answer for Puzzle 1 = {}", fin_res1);

    let mut input2:String="do()".to_string();
    input2.push_str(&input.trim());
    let pz2_expr= exprfind(&mut input2);

    for i in pz2_expr {
        fin_res2 += exprcompute(&i);
    }


    println!("Answer for Puzzle 2 = {}", fin_res2);

}

fn exprfind(input: &mut String) -> Vec<String> {
let mut fexprs:Vec<String> = Vec::new();
    let mut enable:bool = true;
    loop {
        if enable {
            if let Some(idx) = input.find("do()") {
                *input = input[idx+4..].to_string();
                enable = false;
            } else {
                break;
            }
            } else {
            if let Some(idx) = input.find("don't()") {
            fexprs.push(input[..idx].to_string());
            *input = input[idx+7..].to_string();
                enable = true;
            }
            else {
                fexprs.push(input.to_string());
                enable = true;
            }
        }


    }
    fexprs
}

fn exprcompute(input:&String) -> i64{
    let mut result:i64=0;
    for i in 1..1000{
        for j in 1..1000{
            let expr = format!("mul({},{})",i,j);
            let count =input.matches(expr.as_str()).count();
            result += count as i64* i as i64 * j as i64;
        }
    }
    result
}
