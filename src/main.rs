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

    let file = File::open(&args[1]).expect("Failed to open file");
    let all_lines = io::BufReader::new(file)
        .lines()
        .filter_map(|e| e.ok())
        .collect::<Vec<String>>();

    let mut variables = HashSet::new();

    let exprs = all_lines
        .iter()
        .map(|line| shunting_yard(line, &mut variables))
        .collect::<Result<Vec<Vec<Token>>, String>>()?;

    let mut variables = variables.into_iter().collect::<Vec<&str>>();
    variables.sort();

    println!("");
    print!("            ");
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
            .map(|e| evaluate_postfix(e, &varmap))
            .collect::<Result<Vec<bool>, String>>()?;

        // If this row is invalid or not
        if eval_results[0..eval_results.len() - 1].iter().all(|r| *r)
            && !eval_results.last().unwrap()
        {
            print!("│ {:3} │ \x1b[1;31m{:3}\x1b[0m ", i + 1, "ERR");
        } else {
            print!("│ {:3} │ \x1b[1;32m{:3}\x1b[0m ", i + 1, "OK");
        }

        for var in variables.iter() {
            match varmap.get(*var).unwrap() {
                true => print!("│ \x1b[32m{:5}\x1b[0m ", "true"),
                false => print!("│ \x1b[31m{:5}\x1b[0m ", "false"),
            }
        }

        for result in eval_results {
            match result {
                true => print!("│ \x1b[32m{:5}\x1b[0m ", result),
                false => print!("│ \x1b[31m{:5}\x1b[0m ", result),
            }
        }
        println!("│");
        print!("\x1b[0m");
    }

    Ok(())
}
