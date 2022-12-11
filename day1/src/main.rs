fn main() {
    part2();
}

fn part1() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n\n")
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            .max()
            .unwrap(),
    );
}

fn part2() {
    let mut input = include_str!("../input.txt").split("\n\n").map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>()).collect::<Vec<u32>>();
    input.sort();
    println!("{}", input.into_iter().rev().take(3).sum::<u32>())
}
