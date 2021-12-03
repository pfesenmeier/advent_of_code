mod find_gamma_epsilon;
// fn find_rating(tiebreaker: usize)
// turn list into Vec<(pos, rev_list)>
// while len > 1
//   split into two lists with last char removed
//     (Vec<(pos, &str)>, Vec<(pos, &str)>)
//   return longer, breaking ties with (0 | 1)
// return original_list[pos] as integer  // for this calculation

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let (gamma, epsilon) = find_gamma_epsilon::find_gamma_epsilon(input);
    println!("Day 1: {}", gamma * epsilon);
}
