use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn read_file(path: &str, list_one: &mut Vec<i32>, list_two: &mut Vec<i32>) -> Result<(), Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line_iterator = line.split("   ");
                let line_parts: Vec<&str> = line_iterator.collect();
                list_one.push(line_parts[0].to_owned().parse::<i32>().unwrap());
                list_two.push(line_parts[1].to_owned().parse::<i32>().unwrap());
            }
            Err(err) => println!("Erro"),
        }
    }

    Ok(())
}

fn main() {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    read_file("teste.txt", &mut list_one, &mut list_two);

    let mut distance = 0;

    let result: i32 = list_one
        .iter()
        .map(|&i| i * list_two.iter().filter(|&j| *j == i).count() as i32)
        .sum();

    println!("{}", result);
}
