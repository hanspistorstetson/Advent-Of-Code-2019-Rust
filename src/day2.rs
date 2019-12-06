#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|a| a.parse::<i32>().unwrap())
        .collect()
}

#[derive(Clone)]
pub struct Program {
    data: Vec<i32>,
    pointer: usize,
}

impl Program {
    pub fn new(data: Vec<i32>) -> Self {
        Program { data, pointer: 0 }
    }

    pub fn solve_for(&mut self, verb: i32, noun: i32) -> i32 {
        self.data[1] = verb;
        self.data[2] = noun;
        while self.next() {}
        self.data[0]
    }

    pub fn next(&mut self) -> bool {
        let op_code = self.data[self.pointer];
        let idx_a = self.data[self.pointer + 1] as usize;
        let idx_b = self.data[self.pointer + 2] as usize;
        let idx_c = self.data[self.pointer + 3] as usize;
        let res = match op_code {
            1 => {
                let val = self.data[idx_a] + self.data[idx_b];
                self.data[idx_c] = val;
                true
            }
            2 => {
                let val = self.data[idx_a] * self.data[idx_b];
                self.data[idx_c] = val;
                true
            }
            _ => false,
        };

        if res {
            self.pointer += 4;
        }

        res
    }
}

#[aoc(day2, part1)]
fn part_one(input: &[i32]) -> i32 {
    let mut program = Program::new(input.to_vec());
    program.solve_for(12, 2)
}
