pub fn main() {
    // const input = include_str!("../inputs/2019/02_01.txt");
    // let input = "1,0,0,0,99";
    let input = "1,1,1,4,99,5,6,0,99";
    let nums: Vec<u32> = input
        .split(",")
        .into_iter()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect();
    let mut cp = Computer::new(nums);

    loop {
        match cp.process() {
            LoopResult::Error(message) => {
                println!("Something bad happened: {}", message);
                break;
            }
            LoopResult::Pending(data) => println!("Internal State: {:?}", data),
            LoopResult::Ready(data) => {
                println!("Result: {:?}", data);
                break;
            }
        };
    }
}

struct Computer {
    pos: u32,
    data: Vec<u32>,
}

impl Computer {
    fn new(data: Vec<u32>) -> Computer {
        Computer { pos: 0, data }
    }

    fn process(&mut self) -> LoopResult<'_> {
        println!("pos: {}", &self.pos);
        let opt = self.data[self.pos as usize];
        println!("opt: {}", &opt);

        if !is_opt_code(opt) {
            return LoopResult::Error(format!("In Bad State. Found bad opt code: {}", opt));
        }

        if opt == 99 {
            return LoopResult::Ready(&self.data);
        }
        let lhs: usize = self.data[(self.pos + 1) as usize].try_into().unwrap();
        let rhs: usize = self.data[(self.pos + 2) as usize].try_into().unwrap();
        let dest: usize = self.data[(self.pos + 3) as usize].try_into().unwrap();

        if opt == 1 {
            self.data[dest] = self.data[lhs] + self.data[rhs];
        }
        if opt == 2 {
            self.data[dest] = self.data[lhs] * self.data[rhs];
        }

        self.pos += 4;

        LoopResult::Pending(&self.data)
    }
}

enum LoopResult<'a> {
    Error(String),
    Pending(&'a Vec<u32>),
    Ready(&'a Vec<u32>),
}

fn is_opt_code(code: u32) -> bool {
    code == 99 || code == 1 || code == 2
}
