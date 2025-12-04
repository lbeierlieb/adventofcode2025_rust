use std::fs::read_to_string;

#[test]
fn day1_task1() {
    let input = read_to_string("inputs/day1_task1_test").unwrap();
    let result = 3;
    assert_eq!(result, crate::day1::task_one(input));
}

#[test]
fn day1_task2() {
    let input = read_to_string("inputs/day1_task2_test").unwrap();
    let result = 6;
    assert_eq!(result, crate::day1::task_two(input));
}

#[test]
fn day2_task1() {
    let input = read_to_string("inputs/day2_task1_test").unwrap();
    let result = 1227775554;
    assert_eq!(result, crate::day2::task_one(input));
}

#[test]
fn day2_task2() {
    let input = read_to_string("inputs/day2_task2_test").unwrap();
    let result = 4174379265;
    assert_eq!(result, crate::day2::task_two(input));
}

#[test]
fn day3_task1() {
    let input = read_to_string("inputs/day3_task1_test").unwrap();
    let result = 357;
    assert_eq!(result, crate::day3::task_one(input));
}

#[test]
fn day3_task2() {
    let input = read_to_string("inputs/day3_task2_test").unwrap();
    let result = 3121910778619;
    assert_eq!(result, crate::day3::task_two(input));
}
