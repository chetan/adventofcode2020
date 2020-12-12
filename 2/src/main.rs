
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 || &args[1] == "one" {
    one_a();
//   } else {
//     one_b();
  }
}

fn one_a() {

  let mut valid = 0;

  if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
      if let Ok(l) = line {
        let split: Vec<&str> = l.trim().split(':').collect();
        let policy = &split[0];
        let password = &split[1].trim();

        let c = &policy[&policy.len()-1..policy.len()];
        // println!("Hello, {}!", c);

        let psplit: Vec<&str> = policy[0..policy.len()-2].split('-').collect();
        let min = psplit[0].trim().parse::<u32>().unwrap();
        let max = psplit[1].trim().parse::<u32>().unwrap();

        let mut count = 0;
        for a in password.trim().chars() {
          if a == c.chars().next().unwrap() {
            count += 1;
          }
        }
        if count >= min && count <= max {
          valid += 1;
        }

      }
    }
  }

  println!("found {} valid passwords", valid);
}

fn read_ints() -> Vec<u32> {
  let mut ints: Vec<u32> = Vec::new();
  if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
      if let Ok(str_a) = line {
        let a = str_a.parse::<u32>().unwrap();
        ints.push(a);
      }
    }
  }
  return ints;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
