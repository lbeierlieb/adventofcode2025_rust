use itertools::repeat_n;
use nom::{
    character::complete::{digit1, newline, one_of},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::pair,
    IResult, Parser,
};

fn parser(i: &str) -> IResult<&str, Vec<i32>> {
    let line_parser = map(
        pair(one_of("RL"), map_res(digit1, str::parse::<i32>)),
        |(letter, num)| match letter {
            'R' => num,
            'L' => -num,
            _ => unreachable!(),
        },
    );
    let mut list_parser = separated_list0(newline, line_parser);

    list_parser.parse(i)
}

fn parsed(input: &str) -> impl Iterator<Item = i32> {
    parser(&input).unwrap().1.into_iter()
}

fn do_move(pos: i32, mov: i32) -> i32 {
    (pos + mov).rem_euclid(100)
}

pub trait CountZeroHits: Iterator<Item = i32> + Sized {
    fn count_zero_hits(self) -> u64 {
        self.fold((50, 0), |(pos, zeros), mov| {
            let new_pos = do_move(pos, mov);
            let zeros_update = (new_pos == 0) as u64;
            (new_pos, zeros + zeros_update)
        })
        .1
    }
}
impl<I: Iterator<Item = i32>> CountZeroHits for I {}

pub fn task_one(input: String) -> u64 {
    parsed(&input).count_zero_hits()
}

pub fn task_two(input: String) -> u64 {
    parsed(&input)
        .flat_map(|mov| repeat_n(mov.signum(), mov.abs() as usize))
        .count_zero_hits()
}
