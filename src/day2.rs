use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list0,
    sequence::separated_pair, IResult, Parser,
};

fn parser(i: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let range_parser = separated_pair(
        map_res(digit1, str::parse::<u64>),
        tag("-"),
        map_res(digit1, str::parse::<u64>),
    );
    let mut list_parser = separated_list0(tag(","), range_parser);

    list_parser.parse(i)
}

fn parsed(input: &str) -> impl Iterator<Item = (u64, u64)> {
    parser(&input).unwrap().1.into_iter()
}

fn is_repetition(rep_count: usize, str: &str) -> bool {
    let len = str.chars().count();
    if len % rep_count != 0 {
        return false;
    }
    let slice_size = len / rep_count;
    (0..rep_count - 1).all(|i| {
        &str[i * slice_size..(i + 1) * slice_size]
            == &str[(i + 1) * slice_size..(i + 2) * slice_size]
    })
}

fn is_any_repetion(num: u64) -> bool {
    let string = num.to_string();
    (2..=string.len()).any(|i| is_repetition(i, &string))
}

fn sum_of_invalid_numbers<F>(input: &str, validity_checker: F) -> u64
where
    F: Fn(&u64) -> bool,
{
    parsed(input)
        .flat_map(|(start, end)| start..=end)
        .filter(validity_checker)
        .sum()
}

pub fn task_one(input: String) -> u64 {
    sum_of_invalid_numbers(&input, |i| is_repetition(2, &i.to_string()))
}

pub fn task_two(input: String) -> u64 {
    sum_of_invalid_numbers(&input, |i| is_any_repetion(*i))
}
