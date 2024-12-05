use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::ptr;

fn check_levels(list: Vec<Vec<i32>>) -> usize {
    let filtered: Vec<_> = list
        .iter()
        .filter(|sublist| {
            let mut windowed_list = sublist.windows(2);

            windowed_list.clone().all(|w| i32::abs(w[0] - w[1]) <= 3)
                && (windowed_list.clone().all(|w| w[0] < w[1])
                    || windowed_list.clone().all(|w| w[0] > w[1]))
        })
        .collect();

    filtered.len()
}

fn read_file(path: &str, list: &mut Vec<Vec<i32>>) -> Result<(), Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut sublist: Vec<i32> = Vec::new();
                let line_iterator = line.split(" ");
                let line_parts: Vec<&str> = line_iterator.collect();
                line_parts
                    .iter()
                    .for_each(|&e| sublist.push(e.parse::<i32>().unwrap()));
                list.push(sublist)
            }
            Err(err) => println!("Erro"),
        }
    }

    Ok(())
}

fn main() {
    let mut list: Vec<Vec<i32>> = Vec::new();
    read_file("puzzle.txt", &mut list);

    let result = check_levels(list);

    println!("{}", result);
}
