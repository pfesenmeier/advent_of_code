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

    let ( dist, depth ) = elvin_sub.where_am_i();
    println!("{}", dist * depth);
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct Submarine {
    position: Position,
    aim: i32,
}

#[derive(Debug, PartialEq)]
pub enum Step {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl Submarine {
    fn new() -> Self {
        Submarine::default()
    }

    fn dive(&mut self, step: Step) {
        match step {
            Step::Up(dist) => self.aim -= dist,
            Step::Down(dist) => self.aim += dist,
            Step::Forward(dist) => {
                self.position.x += dist;
                self.position.y += self.aim * dist
            },
        }
    }

    fn where_am_i(&self) -> (i32,i32) {
        (self.position.x, self.position.y)
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
