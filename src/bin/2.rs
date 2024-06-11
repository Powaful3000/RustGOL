//// Rust learning endeavor
//// John Conway's game of life impl in rust

const BOARD_WIDTH: u32 = 32;
const BOARD_HEIGHT: u32 = 16;

#[derive(Clone, Debug)]
struct Board {
    width: u32,
    height: u32,
    size: u32,
    cells: Vec<bool>,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        let size = width * height;
        let cells = vec![false; size as usize];
        Board {
            width,
            height,
            size,
            cells,
        }
    }
    // return byte location based off x,y search
    fn get_index(&self, x: u32, y: u32) -> u32 {
        print!("x: {}, y: {}, index: ", x, y);
        let index = x + (y * self.width);
        println!("{}", index);
        return index;
    }

    // set cell to alive or dead
    fn set_cell(&mut self, x: u32, y: u32, alive: bool) {
        let idx: u32 = self.get_index(x, y);
        self.cells[idx as usize] = alive;
    }

    fn set_cell_idx(&mut self, index: u32, alive: bool) {
        self.cells[index as usize] = alive;
    }

    // get cell state
    fn get_cell(&self, x: u32, y: u32) -> bool {
        let idx: u32 = self.get_index(x, y);
        return self.cells[idx as usize];
    }

    fn get_cell_index(&self, index: u32) -> bool {
        return self.cells[index as usize];
    }

    // calculate number of neighbors a cell has
    // **wrapping around edges**
    fn calculate_neighbors(&self, x: u32, y: u32) -> u32 {
        let idx: u32 = self.get_index(x, y);
        let mut neighbor_idx: u32 = 0;
        let mut neighbor_x: u32 = 0;
        let mut neighbor_y: u32 = 0;
        let mut count: u32 = 0;
        // loop y (by row)
        for i in -1..=1 {
            // loop x (by row)
            for j in -1..=1 {
                // skip self
                if j == 0 && i == 0 {
                    continue;
                }
                // calculate neighbor index from offsets
                neighbor_x = ((x as i32 + j) % self.width as i32) as u32;
                neighbor_y = ((y as i32 + i) & self.height as i32) as u32;
                neighbor_idx = self.get_index(neighbor_x, neighbor_y);
                // if cell is living, increment count
                if (self.get_cell_index(neighbor_idx)) {
                    count += 1;
                }
            }
        }
        return count;
    }

    fn query_cell_fate(&mut self, x: u32, y: u32) -> bool {
        let mut alive: bool = self.get_cell(x, y);
        let neighbors: u32 = self.calculate_neighbors(x, y);
        // check conway rules
        if (alive && (neighbors < 2 || neighbors > 3)) {
            // kill alive cells is under or over popped
            alive = false;
        }
        if (!alive && neighbors == 3) {
            // birth new cell if dead cell has 3 neighbors
            alive = true;
        }
        return alive;
    }
    // set user defined initial Board state
    fn set_init_state(&mut self) {
        // classic conway glider
        // [ 0 , 1 , 0 ]
        // [ 0 , 0 , 1 ]
        // [ 1 , 1 , 1 ]
        // 2d vec in yx form
        let init_matrix: Vec<Vec<bool>> = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];

        // Set cells
        for y in 0..init_matrix.len() {
            for x in 0..init_matrix[0].len() {
                if init_matrix[y][x] {
                    self.set_cell(x as u32, y as u32, true);
                }
            }
        }
    }
}

// Boards container to simplify main
struct Universe {
    width: u32,
    height: u32,
    front_board: Board,
    back_board: Board,
}

impl Universe {
    fn new(width: u32, height: u32) -> Universe {
        let mut front_board: Board = Board::new(width, height);
        let mut back_board: Board = front_board.clone();

        front_board.set_init_state();

        Universe {
            width,
            height,
            front_board,
            back_board,
        }
    }

    fn update_universe(&mut self) {
        // calculate new board stored in back_board from calculating
        // neighbors, and thus future state, of each cell

        // loop cells
        // left to right read via yx
        for y in 0..&self.front_board.height - 1 {
            for x in 0..&self.front_board.width - 1 {
                self.back_board
                    .set_cell(x, y, self.front_board.query_cell_fate(x, y));
            }
        }
        // clone resulting back board into front board
        self.front_board = self.back_board.clone();
    }

    fn draw_universe(&self) {
        // draw current universe
        // yummy simple bit vector to String conversion
        // let chars: Vec<char> = self
        let chars_iter = self
            .front_board
            .cells
            .iter()
            .map(|&f| if f { '#' } else { '.' });
        // .collect();
        let mut string_representation = String::from_iter(chars_iter);
        // insert new lines where necessary
        let mut index = 0;
        for line in 0..self.height {
            index = line * self.width + line;
            string_representation.insert(index as usize, '\n');
        }
        println!("{}", string_representation);
    }
}

// Trying to OOP to simplify main as much as possible
fn main() {
    // Create Universe
    let mut universe: Universe = Universe::new(BOARD_WIDTH, BOARD_HEIGHT);
    universe.draw_universe();
    universe.update_universe();
    universe.draw_universe();
}
