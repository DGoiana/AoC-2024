use std::io::BufRead;

fn read_lines() -> (Vec<i32>, Vec<i32>) {
    let (mut list_one, mut list_two) = (vec![], vec![]);

    let reader = utils::open_file();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line_iterator = line.trim().split_whitespace();
                let line_parts: Vec<&str> = line_iterator.collect();

                list_one.push(line_parts[0].parse().unwrap());
                list_two.push(line_parts[1].parse().unwrap());
            }
            Err(_err) => println!("Erro"),
        }
    }

    (list_one, list_two)
}

fn part_two(list_one: Vec<i32>, list_two: Vec<i32>) -> i32 {
    let result: i32 = list_one
        .iter()
        .map(|&i| i * list_two.iter().filter(|&j| *j == i).count() as i32)
        .sum();

    result
}

fn part_one(mut list_one: Vec<i32>, mut list_two: Vec<i32>) -> i32 {
    list_one.sort();
    list_two.sort();

    let zipped: Vec<_> = list_one.iter().zip(list_two.iter()).collect();

    let result: i32 = zipped.iter().map(|(&i1, &i2)| i32::abs(i1 - i2)).sum();

    result
}

fn main() {
    let (list_one, list_two) = read_lines();

    let distance = part_one(list_one.clone(), list_two.clone());
    let difference: i32 = part_two(list_one, list_two);

    println!("Distance is {}", distance);
    println!("Difference is {}", difference);
}
