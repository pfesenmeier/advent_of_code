fn main() {
    let steps: Vec<Step> = include_str!("input.txt")
        .lines()
        .map(step_parser::step)
        .map(Result::unwrap)
        .collect();

    let mut elvin_sub = Submarine::new();
    for step in steps {
        elvin_sub.dive(step);
    }

    assert_eq!(elvin_sub.position.x, 15);
    assert_eq!(elvin_sub.position.y, 60);

    let result = elvin_sub.position.x * elvin_sub.position.y;
    assert_eq!(result, 900);
    println!("{}", result);
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Submarine {
    position: Position,
}

#[derive(Debug, PartialEq)]
pub enum Step {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            position: Position { x: 0, y: 0 },
        }
    }
    fn dive(&mut self, step: Step) {
        match step {
            Step::Up(dist) => self.position.y -= dist,
            Step::Down(dist) => self.position.y += dist,
            Step::Forward(dist) => self.position.x += dist,
        }
    }
}

peg::parser! {
  grammar step_parser() for str {
    rule _ = " "

    rule dist() -> i32
      = d:$(['0'..='9']+)  {? d.parse().or(Err("i32")) }

    rule forward() -> Step
      = $("forward") _ d:dist()  { Step::Forward(d) }

    rule up() -> Step
      = $("up") _ d:dist()  { Step::Up(d) }

    rule down() -> Step
      = $("down") _ d:dist()  { Step::Down(d) }

    pub rule step() -> Step
      =  s:forward() / s:up() / s:down()  { s }
  }
}
