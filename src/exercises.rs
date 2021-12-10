use itertools::{izip, Itertools};
use std::{ascii::AsciiExt, fs};

// Day one - Part one

pub fn aoc_01a() -> Result<usize, std::io::Error> {
    let file = fs::read_to_string("./input/01")?;
    let iter = file.lines().map(|x| x.parse::<i32>().unwrap());
    let answer = iter
        .clone()
        .zip(iter.skip(1))
        .filter(|(x, y)| x < y)
        .count();
    Ok(answer)
}

// Day one - Part two

pub fn aoc_01b() -> Result<usize, std::io::Error> {
    let file = fs::read_to_string("./input/01")?;
    let iter = file.lines().map(|x| x.parse::<i32>().unwrap());
    let answer = izip!(
        iter.clone(),
        iter.clone().skip(1),
        iter.clone().skip(2),
        iter.skip(3)
    )
    .filter(|(a, b, c, d)| a + b + c < b + c + d)
    .count();
    Ok(answer)
}

// Day two - Part one

pub fn aoc_02a() -> Result<i32, std::io::Error> {
    let file = fs::read_to_string("./input/02")?;
    let mut depth = 0;
    let mut horizontal = 0;

    file.lines()
        .map(|x| x.split(" ").map(str::to_owned).collect::<Vec<_>>())
        .for_each(|y| match y[0].as_ref() {
            "forward" => {
                horizontal += y[1].parse::<i32>().unwrap();
            }
            "down" => {
                depth += y[1].parse::<i32>().unwrap();
            }
            "up" => {
                depth -= y[1].parse::<i32>().unwrap();
            }
            _ => {}
        });
    Ok(depth * horizontal)
}

// Day two - Part two

pub fn aoc_02b() -> Result<i32, std::io::Error> {
    let file = fs::read_to_string("./input/02")?;
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    file.lines()
        .map(|x| x.split(" ").map(str::to_owned).collect::<Vec<_>>())
        .for_each(|y| match y[0].as_ref() {
            "forward" => {
                horizontal += y[1].parse::<i32>().unwrap();
                depth += aim * y[1].parse::<i32>().unwrap()
            }
            "down" => {
                aim += y[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= y[1].parse::<i32>().unwrap();
            }
            _ => {}
        });
    Ok(depth * horizontal)
}

// Day three - Part one

pub fn aoc_03a() -> Result<i32, std::io::Error> {
    let file = fs::read_to_string("./input/03")?;
    let mut new_vec: Vec<i32> = file
        .lines()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    let mut counter = vec![0; 12];
    for n in (0..12).rev() {
        new_vec.clone().iter().enumerate().for_each(|(x, y)| {
            if y / i32::pow(2, n) > 0 {
                counter[n as usize] += 1;
                new_vec[x] = y - i32::pow(2, n);
            }
        });
    }
    counter.reverse();
    let answer_1 = counter
        .clone()
        .into_iter()
        .map(|x| if x > 500 { '1' } else { '0' })
        .collect::<String>();

    let answer_2 = answer_1
        .chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect::<String>();
    let gamma = i32::from_str_radix(&answer_1, 2).unwrap();
    let epsilon = i32::from_str_radix(&answer_2, 2).unwrap();
    Ok(epsilon * gamma)
}

// Day three - Part two
// todo REDO whole day 3 part two, can be much shorter
fn aoc_3b_oxy(xx: Vec<String>, n: usize) -> Vec<String> {
    let counter = xx.iter().fold(0, |acc, x| {
        if x.chars().nth(n) == Some('1') {
            acc + 1
        } else {
            acc
        }
    });
    xx.iter()
        .filter_map(|x| {
            if counter >= xx.len() / 2 + xx.len() % 2 {
                if x.chars().nth(n) == Some('1') {
                    Some(x.to_owned())
                } else {
                    None
                }
            } else {
                if x.chars().nth(n) == Some('0') {
                    Some(x.to_owned())
                } else {
                    None
                }
            }
        })
        .collect()
}
// todo REDO whole day 3 part two, can be much shorter
fn aoc_3b_co2(xx: Vec<String>, n: usize) -> Vec<String> {
    let counter = xx.iter().fold(0, |acc, x| {
        if x.chars().nth(n) == Some('1') {
            acc + 1
        } else {
            acc
        }
    });
    xx.iter()
        .filter_map(|x| {
            if counter >= xx.len() / 2 + xx.len() % 2 {
                if x.chars().nth(n) == Some('1') {
                    None
                } else {
                    Some(x.to_owned())
                }
            } else {
                if x.chars().nth(n) == Some('1') {
                    Some(x.to_owned())
                } else {
                    None
                }
            }
        })
        .collect()
}
// todo REDO whole day 3 part two, can be much shorter
pub fn aoc_03b() -> Result<usize, std::io::Error> {
    let file = fs::read_to_string("./input/03")?;
    let mut oxy: Vec<String> = file.lines().map(|x| x.to_owned()).collect();
    let mut o2: Vec<String> = oxy.clone();
    for n in 0..12 {
        if oxy.clone().iter().count() > 1 {
            oxy = aoc_3b_oxy(oxy, n);
        }
        if o2.clone().iter().count() > 1 {
            o2 = aoc_3b_co2(o2, n);
        }
    }
    Ok(usize::from_str_radix(oxy.first().unwrap(), 2).unwrap()
        * usize::from_str_radix(o2.first().unwrap(), 2).unwrap())
}
// todo REDO whole day 3 part two, can be much shorter

// Day four - part one
