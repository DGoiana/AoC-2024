use std::io::BufRead;

fn window_sublist(sublist: Vec<i32>) -> bool {
    let windowed_list = sublist.windows(2);

    windowed_list.clone().all(|w| i32::abs(w[0] - w[1]) <= 3)
        && (windowed_list.clone().all(|w| w[0] < w[1])
            || windowed_list.clone().all(|w| w[0] > w[1]))
}

fn part_one(list: Vec<Vec<i32>>) -> usize {
    let filtered: Vec<_> = list
        .iter()
        .filter(|sublist| window_sublist(sublist.to_vec()))
        .collect();

    filtered.len()
}

fn read_lines() -> Vec<Vec<i32>> {
    let mut list = vec![];

    let reader = utils::open_file();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut sublist = vec![];
                let line_iterator = line.split(" ");
                let line_parts: Vec<&str> = line_iterator.collect();
                line_parts
                    .iter()
                    .for_each(|&e| sublist.push(e.parse::<i32>().unwrap()));
                list.push(sublist)
            }
            Err(_err) => println!("Erro"),
        }
    }

    list
}

fn remove_element_vector(mut list: Vec<i32>, elem: usize) -> Vec<i32> {
    list.remove(elem);

    list
}

fn part_two(list: Vec<Vec<i32>>) -> i32 {
    let r: Vec<Vec<bool>> = list
        .iter()
        .map(|sub| {
            sub.iter()
                .enumerate()
                .map(|(i, _)| window_sublist(remove_element_vector(sub.to_vec(), i)))
                .collect()
        })
        .collect();

    let count: i32 = r
        .iter()
        .map(|f| (f.iter().filter(|&&v| v == false).count() != f.iter().count()) as i32)
        .sum();

    count
}

fn main() {
    let list = read_lines();

    let result_one = part_one(list.clone());

    let result_two = part_two(list);

    println!("First part: {}", result_one);
    println!("Second part: {}", result_two);
}
