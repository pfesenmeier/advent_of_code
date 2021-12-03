pub    fn find_gamma_epsilon(input: Vec<&str>) -> (usize, usize) {
        let num_entries = input.len();
        let line_length: usize = input[0].len();
        let mut num_zeros_each_column: Vec<usize> = Vec::with_capacity(line_length);
        for _i in 0..line_length {
            num_zeros_each_column.push(0);
        }

        for line in input {
            for (i, c) in line.chars().enumerate() {
                if c == '0' {
                    num_zeros_each_column[i] += 1;
                }
            }
        }

        let mut gamma = "".to_string();
        let mut epsilon = "".to_string();

        for num in num_zeros_each_column {
            if num * 2 > num_entries {
                gamma.push('0');
                epsilon.push('1');
            } else {
                gamma.push('1');
                epsilon.push('0');
            }
        }

        let gamma = usize::from_str_radix(&gamma, 2).unwrap();
        let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

        // show how bitmasks work
        let mask = 2_usize.pow(line_length.try_into().unwrap()) - 1;
        let e_2 = gamma ^ mask;
        assert_eq!(e_2, epsilon);

        (gamma, epsilon)
    }
