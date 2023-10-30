use std::env;

fn calculate_maximum_hailstone_numbers(a: u64, b: u64) -> Vec<u64> {
    let mut current_index: usize = 0;
    let mut max: Vec<u64> = Vec::new();

    for i in a..=b {
        let mut s: u64 = i;
        max.push(s);
        while s > 4 {
            if s % 2 == 0 {
                s = s / 2;
            } else {
                s = s * 3 + 1;
            }
            if s > max[current_index] {
                //always replace the entry of the array with the biggest hailstone number in the current sequence
                max[current_index] = s;
            }
        }
        current_index += 1;
    }
    return max;
}

fn main() {
    let numbers: Vec<String> = env::args().collect();
    if numbers.len() < 2 {
        eprintln!("Please provide atleast 2 commmand-line arguments.");
        std::process::exit(1);
    }

    let number1: u64 = numbers[1]
        .parse()
        .expect("You need to provide an integer number");
    let number2: u64 = numbers[2]
        .parse()
        .expect("You need to provide an integer number");

    let bind = calculate_maximum_hailstone_numbers(number1, number2);
    let min_hailstone = bind.iter().min(); //to find the minimum of the returned array.

    match min_hailstone {
        Some(min) => println!("{}", min),
        None => println!("Vector is Empty"),
    }
}
