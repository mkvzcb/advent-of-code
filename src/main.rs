mod exercises;

use exercises::*;

fn main() -> std::io::Result<()>{
    println!("Advent of Code 2021");
    println!("###################");
    println!("Day 1 - Part One: {}", aoc_01a()?);
    println!("Day 1 - Part Two: {}", aoc_01b()?);
    println!("Day 2 - Part One: {}", aoc_02a()?);
    println!("Day 2 - Part Two: {}", aoc_02b()?);
    println!("Day 3 - Part One: {}", aoc_03a()?);


    Ok(())
}
