fn main() {
    // TODO why not include_bytes?
    let input: Vec<Vec<&str>> = include_str!("input.txt")
      .lines()
      .map(|number| number.split("").collect())
      .map(|list: Vec<&str>| list.into_iter().filter(|entry| *entry != "").collect())
      .collect();
    
    let length = input.len();
    let mut num_zeros = [0;12];

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
