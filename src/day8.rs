#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<u32>{
    input.chars().map(|c| c.to_digit(10).expect("Failed to parse u32")).collect()
}

#[derive(Debug, Clone)]
pub struct Layer {
    data: Vec<u32>
}

impl Layer {
    pub fn new(data: &[u32]) -> Layer {
        Layer { data: data.to_vec() }
    }

    pub fn get_2d(&self, width: usize, height: usize) -> Vec<Vec<u32>>{
        let mut layer = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(self.data[y * width + x]);
            }
            layer.push(row);
        }

        layer
    }

    pub fn checksum(&self) -> (u32, u32){
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for x in self.data.iter() {
            match x {
                0 => zeroes += 1,
                1 => ones += 1,
                2 => twos += 1,

                _ => unimplemented!()
            };
        }

        (zeroes, ones * twos)
    }
}

#[aoc(day8, part1)]
fn part_one(data: &[u32]) -> String {
    let width = 25;
    let height = 6;

    solve_part_one(data, width, height)
}

#[aoc(day8, part2)]
fn part_two(data: &[u32]) -> String {
    let width = 25;
    let height = 6;

    solve_part_two(data, width, height)
}


fn solve_part_one(data: &[u32], width: usize, height: usize ) -> String {
    let mut layers: Vec<(u32, u32)> = data.chunks(width * height).map(|w| Layer::new(w).checksum()).collect();
    layers.sort_by(|a, b| a.0.cmp(&b.0));

    format!("{:?}", layers[0])
}

fn solve_part_two(data: &[u32], width: usize, height: usize ) -> String {
    let mut raw_layers: Vec<Layer> = data.chunks(width * height).map(|w| Layer::new(w)).collect();
    let mut layers = vec![];
    for layer in raw_layers.iter() {
        layers.push(layer.get_2d(width, height));
    }

    let mut top_layer = vec![vec![2; width]; height];

    for layer in layers.iter() {
        for (y, row) in layer.iter().enumerate() {
            for (x, val) in row.iter().enumerate() {
                top_layer[y][x] = match top_layer[y][x] {
                    2 => *val,
                    _ => top_layer[y][x]
                };
            }
        }
    }

    for row in top_layer {
        println!("{:?}", row);
    }

    println!("\n\n");




    format!("{:?}", "test")

}
