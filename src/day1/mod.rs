use std::cmp::min;

pub(crate) fn part1(data: String) -> i32 {
    let mut count = 0;
    let split = data.split("\r\n");
    let splited_data = split
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    for i in 0..splited_data.len() {
        if i == 0 {
            continue;
        } else if splited_data[i] > splited_data[i - 1] {
            count += 1;
        }
    }
    count
}

pub(crate) fn part2(data: String) -> i32 {
    let mut count = 0;
    let split = data.split("\r\n");
    let splited_data = split
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut previous_sum = splited_data[0] + splited_data[1] + splited_data[2];

    for i in 1..splited_data.len() {
        let mut sum = 0;
        for j in i..min(i + 3, splited_data.len()) {
            sum += splited_data[j];
        }

        if sum > previous_sum {
            count += 1;
        }
        previous_sum = sum;
    }

    count
}
