use rand::prelude::SliceRandom;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CellType {
    Empty,
    Wall,
}
pub struct Level {
    pub cells: Vec<CellType>,
    width: usize,
}

impl Level {
    pub fn maze(width: usize, height: usize) -> Level {
        if width % 2 + height % 2 != 2 {
            panic!("Width and height must be odd");
        }
        let mut rng = rand::thread_rng();
        let mut level = Level {
            width,
            cells: vec![CellType::Wall; width * height],
        };

        // Act I
        let starting_point = (1, 1);
        let mut stack: Vec<(i8, i8)> = vec![starting_point];
        *level.cell_at(starting_point.0 as usize, starting_point.1 as usize) = CellType::Empty;

        loop {
            match stack.pop() {
                Some(current) => {
                    // list of possible moves
                    let mut directions: Vec<(i8, i8)> = vec![
                        (0, 1),  // up
                        (0, -1), // down
                        (-1, 0), // left
                        (1, 0),  // right
                    ];
                    // keep only valid moves
                    directions.retain(|&dir| {
                        let (x, y) = (
                            (current.0 + 2 * dir.0) as usize,
                            (current.1 + 2 * dir.1) as usize,
                        );

                        (0..width).contains(&x)
                            && (0..height).contains(&y)
                            && *level.cell_at(x, y) == CellType::Wall
                    });
                    // randomise move
                    match directions.choose(&mut rng) {
                        Some(dir) => {
                            // make a path
                            *level.cell_at(
                                (current.0 + dir.0) as usize,
                                (current.1 + dir.1) as usize,
                            ) = CellType::Empty;

                            *level.cell_at(
                                (current.0 + 2 * dir.0) as usize,
                                (current.1 + 2 * dir.1) as usize,
                            ) = CellType::Empty;
                            // push new elements onto the stack
                            stack.push(current);
                            stack.push((current.0 + 2 * dir.0, current.1 + 2 * dir.1));
                        }
                        None => {}
                    };
                }
                None => break,
            }
        }
        level
    }

    pub fn cell_at(&mut self, x: usize, y: usize) -> &mut CellType {
        &mut self.cells[y * self.width + x]
    }
}


fn main() {
    let level = Level::maze(5 * 21, 5 * 9);
    
    for (i, cell) in level.cells.iter().enumerate() {
        if i % level.width == 0 {
            println!();
        }
        print!("{}", if *cell == CellType::Wall {"█"} else {" "});
    }
    println!();
}