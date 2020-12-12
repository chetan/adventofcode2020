use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() == 1 || &args[1] == "one" {
    one_a();
  } else {
    one_b();
  }
}

fn one_b() {
  let ints = read_ints();
  for a in &ints {
    for b in &ints {
      for c in &ints {
        if a + b + c == 2020 {
          println!("found it! {} + {} + {} = 2020", a, b, c);
          println!("          {} * {} * {} = {}", a, b, c, a * b * c);
          exit(0);
        }
      }
    }
  }
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

fn one_a() {
  if let Ok(lines) = read_lines("./input.txt") {
    for line in lines {
      if let Ok(str_a) = line {
        let a = str_a.parse::<i32>().unwrap();

        if let Ok(lines_b) = read_lines("./input.txt") {
          for line_b in lines_b {
            if let Ok(str_b) = line_b {
              let b = str_b.parse::<i32>().unwrap();
              if a + b == 2020 {
                println!("found it! {} + {} = 2020", a, b);
                println!("          {} * {} = {}", a, b, a * b);
                exit(0);
              }
            }
          }
        }
      }
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
