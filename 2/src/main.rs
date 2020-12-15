
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 || &args[1] == "one" {
    one_a();
  } else {
    one_b();
  }
}

fn one_b() {
  let mut valid = 0;

  if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
      if let Ok(l) = line {
        let split: Vec<&str> = l.trim().split(':').collect();
        let policy = &split[0];
        let password = &split[1].trim();

        // target char
        let c = &policy.chars().collect::<Vec<char>>().pop().unwrap();

        // first & second position
        let psplit: Vec<&str> = policy[0..policy.len()-2].split('-').collect();
        let first = psplit[0].trim().parse::<usize>().unwrap() - 1; // get 0-based index
        let second = psplit[1].trim().parse::<usize>().unwrap() - 1;

        let chars: Vec<char> = password.trim().chars().collect();
        if &chars[first] == c && &chars[second] != c {
          valid += 1;
        } else if &chars[first] != c && &chars[second] == c {
          valid += 1;
        }
      }
    }
  }

  println!("found {} valid passwords", valid);
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
