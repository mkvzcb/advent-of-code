use itertools::izip;
use std::fs;

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

pub fn aoc_03a() -> Result<i32, std::io::Error> {
    let file = fs::read_to_string("./input/03")?;
    let mut new_vec: Vec<i32> = file
        .lines()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    let mut counter = vec![0; 12];
    for n in (0..12).rev() {
        new_vec.clone().into_iter().enumerate().for_each(|(x, y)| {
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
    /*  let answer_2 = counter
    .into_iter()
    .map(|x| if x > 500 { '0' } else { '1' })
    .collect::<String>();*/
    let answer_2 = answer_1
        .chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect::<String>();
    let gamma = i32::from_str_radix(&answer_1, 2).unwrap();
    let epsilon = i32::from_str_radix(&answer_2, 2).unwrap();
    Ok(epsilon * gamma)
}
