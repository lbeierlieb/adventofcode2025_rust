use nom::{
    character::complete::{digit1, newline},
    multi::separated_list0,
    IResult, Parser,
};

fn parser(i: &str) -> IResult<&str, Vec<&str>> {
    let mut list_parser = separated_list0(newline, digit1);

    list_parser.parse(i)
}

fn parsed(input: &str) -> impl Iterator<Item = &str> {
    parser(&input).unwrap().1.into_iter()
}

fn max_jolt(jolts: usize, battery: &str) -> u64 {
    let mut digits = vec![];
    let mut last_index = None;
    let len = battery.len();

    for i in 0..jolts {
        let start_index = last_index.map(|i| i + 1).unwrap_or(0);
        let search_range = &battery[start_index..len - (jolts - 1 - i)];
        let digit = search_range.chars().max().unwrap();
        last_index = Some(start_index + search_range.chars().position(|c| c == digit).unwrap());
        digits.push(digit);
    }

    digits.into_iter().collect::<String>().parse().unwrap()
}

pub fn jolt_sum(input: &str, jolts: usize) -> u64 {
    parsed(&input).map(|line| max_jolt(jolts, line)).sum()
}

pub fn task_one(input: String) -> u64 {
    jolt_sum(&input, 2)
}

pub fn task_two(input: String) -> u64 {
    jolt_sum(&input, 12)
}
