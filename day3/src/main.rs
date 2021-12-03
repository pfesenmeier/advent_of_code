mod part_1;
mod part_2;

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let (gamma, epsilon) = part_1::find_gamma_epsilon(&input);
    println!("Day 1: {}", gamma * epsilon);

    let (oxygen, co2) = part_2::find_oxygen_co2_rating(&input);
    println!("Day 2: {}", oxygen * co2);
}
