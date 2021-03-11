mod utils;

use std::char;
use std::fmt;

use utils::read_lines;

const INPUT_FN : &str = "input/day3.txt";

#[derive(Debug)]
struct TobogganMap {
    data: Vec<bool>,
    base_width: u64,
    height: u64,
}

impl fmt::Display for TobogganMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let the_map = self.render();
        write!(f, "{:}", the_map.as_str())
    }
}

impl TobogganMap {

    fn render(&self) -> String {
        let mut rendered_map = String::new();
        let mut col_idx = 0u64;
        for x in self.data.iter() {
            rendered_map.push(match *x {
                true => '#',
                false => '.',
            });
            col_idx += 1;
            if col_idx >= self.base_width {
                rendered_map.push('\n');
                col_idx = 0;
            }
        }

        rendered_map
        
    }
    
    fn load_file(filename: &str) -> Self {
        let mut line_no = 0u64;
        let mut data = vec![];
        let mut base_width: Option<u64> = None;
        let mut height: u64 = 0;

        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                line_no += 1;
                if let Ok(terrain_line) = line {
                    let mut line_width = 0u64;
                    for c in terrain_line.chars() {
                        let tree = match c {
                            '.' => false,
                            '#' => true,
                            _ => unreachable!()
                        };
                        data.push(tree);
                        line_width += 1;
                    }
                    match base_width {
                        Some(w) => assert_eq!(line_width, w),
                        None => {
                            base_width = Some(line_width);
                        }
                    }
                    height += 1;
                }
            }
        }

        TobogganMap {
            data,
            base_width: base_width.unwrap(),
            height
        }

    }

    fn has_tree_at(&self, col: u64, row: u64) -> Option<bool> {
        if row >= self.height {
            return None;
        }
        let base_col = col % self.base_width;
        let data_idx = (row * self.base_width) + base_col;
        Some(self.data[data_idx as usize])
    }

    fn zoom(&self, slope: (u64, u64)) -> u64 {
        let mut pos = (0u64, 0u64);
        let mut tree_count = 0u64;
        while pos.1 < self.height {
            tree_count += match self.has_tree_at(pos.0, pos.1) {
                Some(true) => 1,
                Some(false) => 0,
                _ => unreachable!(),
            };
            pos.0 += slope.0;
            pos.1 += slope.1;
        }
        
        tree_count
    }
}

fn main() -> Result<(), String> {
    let t_map = TobogganMap::load_file(INPUT_FN);
    println!("{:}", t_map);
    
    let part1_slope = (3, 1);
    let tree_count = t_map.zoom(part1_slope);
    println!("part 1 tree_count = {:?}", tree_count);

    let slopes = vec![(1u64, 1u64), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part2_answer = slopes.iter().map(
        |s| t_map.zoom(*s)
    ).fold(1u64,
         |total, count| total * count );
    println!("part 2 answer = {:?}", part2_answer);
    
    Ok(())
}