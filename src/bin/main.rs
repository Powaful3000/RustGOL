use std::{thread::sleep, time::Duration, vec};


//// Rust learning endeavor
//// John Conway's game of life impl in rust

// Rules:
// >3 neihgbors, cell dies
// 2 or 3 neighbors, cell survives
// dead cell with 3 neighbors becomes live cell

// 2D vector representing board of alive and dead cells
const COLUMN_SIZE: usize = 16;
const ROW_SIZE: usize = 32;
const REFRESH_RATE_MILLIS: u64 = 50;

//// Board struct
// yx over xy for memory optimization
#[derive(Debug, Clone)]
struct Board {
    yx: Vec<Vec<bool>>,
}

impl Board {
    // creates full 2d board of all 0 (false)
    pub fn new() -> Self {
        Board {
            yx: vec![vec![false; COLUMN_SIZE]; ROW_SIZE],
        }
    }
}

//// Function init_board()
// create classic GOL glider at 0,0 on board
// 010      .#.
// 001      ..#
// 111      ###
// Represented in memory as:
// [
//  [0,1,0]
//  [0,0,1]
//  [1,1,1]
// ]
fn init_board(board: &mut Board) {
    board.yx[0][1] = true; // (x,y):(1,0)
    board.yx[1][2] = true; // (x,y):(2,1)
    board.yx[2][0] = true; // (x,y):(0,2)
    board.yx[2][1] = true; // (x,y):(1,2)
    board.yx[2][2] = true; // (x,y):(2,2)
}

//// Function update_back_board
// Update the cell in back_board based on the rules
// Rules:
// >3 neighbors, cell dies
// 2 or 3 neighbors, cell survives
// dead cell with 3 neighbors becomes live cell
fn update_back_board(
    x: usize,
    y: usize,
    neighbor_count: i32,
    front_board: &mut Board,
    back_board: &mut Board,
) {
    back_board.yx[y][x] = match neighbor_count {
        2 | 3 if front_board.yx[y][x] => true,
        3 if !front_board.yx[y][x] => true,
        _ => false,
    };
}

//// Function calc_cell_neighbors
// (x: usize, y: usize)
// iter cell in FRONT_BOARD and storing results in BACK_BOARD
// before finally copying the final BACK_BOARD to FRONT_BOARD
// need to count alive cells surrounding given cell
//
// Glider Example: .=0, #=1
//  .#.
//  ..#
//  ###
// given cell has 5 alive members
fn calc_cell_neighbors(x: usize, y: usize, front_board: &mut Board, back_board: &mut Board) {
    let mut neighbor_count: i32 = 0;

    // Iterate over the neighboring cells and count the alive ones
    // Counts neighbors across board bounds
    for dy in -1isize..=1isize {
        for dx in -1isize..=1isize {
            // Skip the current cell by exiting the loop
            if dx == 0 && dy == 0 {
                continue;
            }
            // Wraps out of bounds index to other end of board
            // Adds offset dy and dx to given location y,x respectively
            // Mods result with max size to wrap around out of bounds
            let nx: usize =
                ((x as isize + dx + COLUMN_SIZE as isize) % COLUMN_SIZE as isize) as usize;
            let ny: usize = ((y as isize + dy + ROW_SIZE as isize) % ROW_SIZE as isize) as usize;
            if front_board.yx[ny][nx] {
                neighbor_count += 1;
            }
        }
    }
    update_back_board(x, y, neighbor_count, front_board, back_board)
}

// Function refresh_front_board
// Takes computed back_board and moves data into front_board
fn refresh_front_board(front_board: &mut Board, back_board: &mut Board) {
    // clone board data not entire board object
    // *front_board = back_board.clone();
    front_board.yx = back_board.yx.clone();
}

// Function update_board
// Loops entire board, calculates next board storing it in back_board
// Once calculated, replace front_board with back_board
fn update_board(mut front_board: &mut Board, mut back_board: &mut Board) {
    for y in 0..COLUMN_SIZE {
        for x in 0..ROW_SIZE {
            calc_cell_neighbors(y, x, &mut front_board, &mut back_board);
        }
    }
    refresh_front_board(front_board, back_board);
}

// Function print_board
// Prints the contents of the board.
// not working properly?
// fn print_board(board: &mut Board, padding_lines: u32) {
//     let mut cell_buf: char;
//     for line in 0..COLUMN_SIZE-1 {
//         println!("");
//         for chr in 0..ROW_SIZE-1 {
//             cell_buf = if board.yx[chr][line] { '#' } else { '.' };
//             print!("{}", cell_buf);
//         }
//     }
//     // loop adding defined padded lines
//     for _lines in 0..padding_lines {
//         println!();
//     }
// }


fn print_board(board: &mut Board, padding_lines: u32) {
    let mut vec_length = ((ROW_SIZE + 1) * COLUMN_SIZE);
    let mut string_vec: Vec<char> = vec!['.'; vec_length];
    let mut char_index: usize = 0;

    // Loop every line and set ROW_SIZE+1 as a newline char
    // This breaks up
    for line in 0..COLUMN_SIZE {
        char_index = (ROW_SIZE + 1) * line;
        string_vec[char_index] = '\n';
        // println!("newline index: {}", char_index);
    }

    // println!("{}", String::from_iter(&string_vec));

    // find living cells and reflect that in the string
    for x in 0..ROW_SIZE - 1 {
        for y in 0..COLUMN_SIZE - 1 {
            // if cell alive
            if board.yx[x][y] {
                // char_index = (((ROW_SIZE + 1) * col) + row + 1) % vec_length;
                char_index = (((ROW_SIZE + 1) * x) + y + 1);
                
                println!("(x,y,char_index): {}, {}, {}", y, x, char_index);
                string_vec[char_index % vec_length] = '#';
            }
        }
    }

    println!("{}", String::from_iter(&string_vec));
}

// Function main
fn main() {
    // Create two instances of the Board struct
    let mut front_board: Board = Board::new();
    let mut back_board: Board = Board::new();

    // Initialize the front board
    init_board(&mut front_board);

    // Print the initial state of the front board
    print_board(&mut front_board, 1);

    // Enter the game loop
    loop {
        // Update the front board based on the back board
        update_board(&mut front_board, &mut back_board);

        // Print the current state of the front board
        print_board(&mut front_board, 1);

        // Pause the program for a specified duration
        sleep(Duration::from_millis(REFRESH_RATE_MILLIS));
        // break;
    }
}
