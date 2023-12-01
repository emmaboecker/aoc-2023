use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input/day01.txt").to_owned();

    // println!("{}", find_numbers(&input));

    println!("{}", find_numbers(&input, true));
}

fn find_numbers(input: &str, written: bool) -> u32 {
    let output = input.lines().map(|line| {
        let line = line.to_owned();
        println!("{}", line);

        let valid = find_valid(&line, written);

        println!("{:?}", valid);

        format!("{}{}", valid.first().unwrap(), valid.last().unwrap())
    }).collect::<Vec<String>>();

    output.iter().map(|string| string.parse::<u32>().unwrap()).sum::<u32>()
}

fn find_valid(input: &str, written: bool) -> Vec<u32> {
    let mut valid = vec![];

    if written {
        let mut written_numbers = HashMap::new();
        written_numbers.insert("one", 1);
        written_numbers.insert("two", 2);
        written_numbers.insert("three", 3);
        written_numbers.insert("four", 4);
        written_numbers.insert("five", 5);
        written_numbers.insert("six", 6);
        written_numbers.insert("seven", 7);
        written_numbers.insert("eight", 8);
        written_numbers.insert("nine", 9);

        for (written, value) in written_numbers {
            let indices = input.match_indices(written);

            for (index, _) in indices {
                valid.push((index, value));
            }
        }
    }

    for value in 1..10 {
        let string_value = value.to_string();
        let indices = input.match_indices(&string_value);

        for (index, _) in indices {
            valid.push((index, value));
        }
    }

    valid.sort_by(|a, b| a.0.cmp(&b.0));
    valid.into_iter().map(|(_, value)| value).collect::<Vec<u32>>()
}