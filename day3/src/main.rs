// pt 2
// for every member in list
// fold to the most common char in first place
// reduce the list to the members starting with that char
// if list is one, return that number
// else, repeat 
//  

// or
// for list
// fold to tuple (0 occur, 1 occur)
// reduce the list to the members starting with more common occurances
// if list is one, return that number
// else, repeat 

// fn find_rating(tiebreaker: usize)
// turn list into Vec<(pos, rev_list)>
// while len > 1
//   split into two lists with last char removed
//     (Vec<(pos, &str)>, Vec<(pos, &str)>)
//   return longer, breaking ties with (0 | 1)
// return original_list[pos] as integer  // for this calculation

fn main() {
    // TODO why not include_bytes?
    let input: Vec<Vec<&str>> = include_str!("input.txt")
      .lines()
      // TODO better representation of 0 / 1s than a Vec<str> ?
      // How about String?
      .map(|number| number.split("").collect())
      // TODO better way to filter first and last elements of a vec??
      .map(|list: Vec<&str>| list.into_iter().filter(|entry| *entry != "").collect())
      .collect();
    
    let length = input.len();
    // TODO - use length of first input
    let mut num_zeros = [0;12];

    // TODO for pt2: a way to get hold of iterator again, to pull out chosen numbers?
    for line in &input {
        for (i,c) in line.iter().enumerate() {
            if *c == "0" {
                num_zeros[i] += 1;
            }
        }
    }

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for num in num_zeros {
        if num * 2 > length {
            gamma.push('0');
            epsilon.push('1');
        } else {
          gamma.push('1');
          epsilon.push('0');
        }
    }

    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("{} ", gamma * epsilon);
}
