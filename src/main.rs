use std::fs;

const FILE_TYPE: &str = "prod";
fn main() {
    day1();
}

fn day1() {
    let file_path = String::from(format!("./src/day1.{}", FILE_TYPE));

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // split the file into a group of vectors of numbers
    let numbers: Vec<Vec<usize>> = contents
        .split("\n\n") // split the file into groups
        .map(|group| {
            // split the group into lines
            group
                .split("\n") // split the lines into numbers
                .flat_map(|line| line.parse::<usize>()) // parse the numbers into usize, while ignoring the errors
                .collect() // collect the numbers into a vector
        })
        .collect(); // collect the groups into a vector
                    // get the max without using unwrap
    let max = numbers
        .iter() // iterate over the groups
        .map(|group| group.iter().sum::<usize>()) // get the sum of each group
        .max(); // get the max of the sums

    let max = match max {
        // match the max to get the value
        Some(max) => max, // if there is a max, return it
        None => 0,        // if there is no max, return 0
    };
    println!("{:?}", max);
}
