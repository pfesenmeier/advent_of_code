#[derive(Debug, PartialEq)]
pub struct Id {
    label: char,
    num: u32,
}

peg::parser! {
  grammar list_parser() for str {
    rule num() -> u32
      = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule label() -> char
      = l:$(['A'..='Z']) {? l.parse().or(Err("foo")) }

    rule id() -> Id
      = l:label() n:num() { Id { label: l, num: n } }

    pub rule list() -> Vec<Id>
      =  l:id() ** "," { l }
  }
}

fn main() {
    let ids = "R75,D30,V123,A32";
    let ids = list_parser::list(ids).unwrap();
    for id in ids {
        println!("id: {}, label: {}", id.num, id.label);
    }
}

#[test]
fn test() -> () {
    let ids = "R75,D30";
    assert_eq!(
        list_parser::list(ids),
        Ok(vec![
            Id {
                label: 'R',
                num: 75
            },
            Id {
                label: 'D',
                num: 30
            }
        ])
    );
}
