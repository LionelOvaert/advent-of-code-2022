fn main() {
    part1();
    part2();
}

fn part1() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n') // split by line
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,)) // convert each letter to a value with base A for the first letter and base X for the second letter
            .map(|(a, b)| 1 + b + 3 * ((1 + b - a).rem_euclid(3))) // first part computes the points for what you played and second part computes the win/loss/draw
            .sum::<i16>(),
    );
}

fn part2() {
    println!(
        "{}",
        include_bytes!("../input.txt")
            .split(|b| *b == b'\n') // split by line
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,)) // convert each letter to a value with base A for the first letter and base X for the second letter
            .map(|(a, b)| 1 + b * 3 + (2 + a + b).rem_euclid(3)) // First part computes the win/loss/draw and second part our score
            .sum::<i16>(),
    );
}