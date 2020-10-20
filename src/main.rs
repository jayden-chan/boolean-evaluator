mod evaluator;
mod parser;

use evaluator::evaluate_postfix;
use parser::{shunting_yard, Token};

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn bit_at(num: usize, i: usize) -> bool {
    if i < std::mem::size_of::<usize>() {
        num & (1 << i) != 0
    } else {
        false
    }
}

fn main() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err(String::from("Must specify a file."));
    }

    let mut exprs: Vec<Vec<Token>> = Vec::new();
    let mut variables = HashSet::new();

    let file = File::open(&args[1]).unwrap();
    let all_lines = io::BufReader::new(file)
        .lines()
        .filter_map(|e| if e.is_ok() { Some(e.unwrap()) } else { None })
        .collect::<Vec<String>>();

    for line in &all_lines {
        exprs.push(shunting_yard(&line, &mut variables)?);
    }

    let mut variables = variables.into_iter().collect::<Vec<&str>>();
    variables.sort();

    println!("");
    print!("           ");
    for var in variables.iter() {
        print!("│ \x1b[1;37m{:5}\x1b[0m ", var);
    }
    for i in 1..=exprs.len() {
        print!("│ \x1b[1;37mexpr{:1}\x1b[0m ", i);
    }
    println!("│");

    for i in 0..(2 as usize).pow(variables.len() as u32) {
        let mut varmap = HashMap::new();
        for (j, var) in variables.iter().rev().enumerate() {
            varmap.insert(*var, !bit_at(i, j));
        }

        let eval_results = exprs
            .iter()
            .map(|e| evaluate_postfix(e, &varmap).unwrap())
            .collect::<Vec<bool>>();

        // If this row is invalid or not
        if eval_results[0..eval_results.len() - 1].iter().all(|r| *r)
            && !eval_results.last().unwrap()
        {
            print!("│ {:2} │ \x1b[1;31m{:3}\x1b[0m ", i + 1, "ERR");
        } else {
            print!("│ {:2} │ \x1b[1;32m{:3}\x1b[0m ", i + 1, "OK");
        }

        for var in variables.iter() {
            let result = varmap.get(*var).unwrap();
            if *result {
                print!("│ \x1b[32m{:5}\x1b[0m ", result);
            } else {
                print!("│ \x1b[31m{:5}\x1b[0m ", result);
            }
        }

        for result in eval_results {
            if result {
                print!("│ \x1b[32m{:5}\x1b[0m ", result);
            } else {
                print!("│ \x1b[31m{:5}\x1b[0m ", result);
            }
        }
        println!("│");
        print!("\x1b[0m");
    }

    Ok(())
}
