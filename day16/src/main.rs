fn main() {
    let input = include_str!("input.txt");

    println!("{}", part_1(&input));
}

fn part_1(input: &str) -> usize {
    todo!()
}

fn part_2(input: &str) -> usize {
    todo!()
}

mod bits {
    // [x] convert BITS hext to binary
    // bit_packet = packet+ '0'*
    // packet = version + typeid
    // version = 3b_number
    // typeid = 3b_number
    // 3b_number = [0-9]{3}, to number
//
    // pkt = literal_value | operator
}

mod literal {
    use itertools::Itertools;

    struct Literal {}

    impl Literal {

       /// requires s without version or id
       pub fn parse(s: &str) -> u32 {
           println!("hello");
           let mut str = "".to_string();

           for chunk in &s.chars().chunks(5) {
               let mut chunk = chunk.into_iter();
               let first = chunk.next().unwrap();
                
               for c in chunk {
                    str.push(c);
                }

                if first == '0' {
                    break;
                }
            }

           u32::from_str_radix(&str, 2).unwrap()
        }
    }
   
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn twenty_twenty_one() {
           assert_eq!(Literal::parse("101111111000101000"), 2021);

        }

        #[test]
        fn binary_2021() {
           assert_eq!(u32::from_str_radix("011111100101", 2).unwrap(), 2021);
        }

    }
}

mod three_bits {
    pub fn three_bit_str(s: &str) -> u8 {
        u8::from_str_radix(s, 2).unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn one_hundred_to_four() {
            assert_eq!(three_bit_str("100"), 4)
        }
    }
}

mod hex {
    pub fn str_to_binary_array(input: &str) -> Vec<String> {
        input
            .chars()
            .map(hex_char_to_binary_str)
            .collect::<Vec<String>>()
    }
    fn hex_char_to_binary_str(c: char) -> String {
        let num = u8::from_str_radix(&c.to_string(), 16).unwrap();
        format!("{:b}", num)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn f_to_four_ones() {
            assert_eq!(hex_char_to_binary_str('F'), "1111");
        }

        #[test]
        fn should_convert_binary_str_to_u64() {
            let num = u64::from_str_radix("1011", 2).unwrap();
            assert_eq!(num, 11);
        }

        #[test]
        fn binary_str() {
            let binary = u8::from_str_radix("B", 16).unwrap();
            assert_eq!(format!("{:b}", binary), "1011");
        }
    }
}

mod literal_value {
    fn decode_literal(literal: &str) -> String {
        "".to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn should_decode_literal_value() {
            assert_eq!(decode_literal("110100101111111000101000"), "D2FE28");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_pass_on_first_sample_input() {
        assert_eq!(part_1("8A004A801A8002F478"), 16);
    }

    #[test]
    fn part_1_should_pass_on_second_sample_input() {
        assert_eq!(part_1("620080001611562C8802118E34"), 12);
    }

    #[test]
    fn part_1_should_pass_on_third_sample_input() {
        assert_eq!(part_1("C0015000016115A2E0802F182340"), 23);
    }

    #[test]
    fn part_1_should_pass_on_fourth_sample_input() {
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780"), 31);
    }
}
