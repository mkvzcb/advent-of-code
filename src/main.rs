use std::fs;
use std::io::BufRead;
use std::io::BufReader;

fn aoc_01() -> std::io::Result<()> {
    let file = fs::read_to_string("./input/01")?;
    let iter = file.lines().map(|x| x.parse::<i32>().unwrap());
    let answer = iter
        .clone()
        .zip(iter.skip(1))
        .filter(|(x, y)| x < y)
        .into_iter()
        .count();

    println!("Day 1 - Part One: {:?}", answer);

    Ok(())
}

fn aoc_02a() -> std::io::Result<()> {
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
    println!("Day 2 - Part One: {}", depth * horizontal);
    Ok(())
}

fn aoc_02b() -> std::io::Result<()> {
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
    println!("Day 2 - Part Two: {}", depth * horizontal);
    Ok(())
}

/*fn aoc_03() -> std::io::Result<()> {
    let file = fs::read_to_string("./input/01")?;


    println!("Day 3: {:?}", answer);

    Ok(())
}*/
fn main() {
    println!("Advent of Code 2021");
    println!("###################");
    aoc_01();
    aoc_02a();
    aoc_02b();
}
