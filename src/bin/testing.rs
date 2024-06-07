use std::string;

// 2D vector representing board of alive and dead cells
const COLUMN_SIZE: usize = 16;
const ROW_SIZE: usize = 32;
const REFRESH_RATE_MILLIS: u64 = 100;

// Board struct
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

fn board_to_string(board: &mut Board, padding_lines: u32) {
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
    for col in 0..ROW_SIZE - 1 {
        for row in 0..COLUMN_SIZE - 1 {
            // if cell alive
            if board.yx[col][row] {
                char_index = ((ROW_SIZE + 1) * col) + row + 1;
                println!("(x,y,char_index): {}, {}, {}", row, col, char_index);
                string_vec[char_index] = '#';
            }
        }
    }

    println!("{}", String::from_iter(&string_vec));
}

// not working correctly
fn print_board(board: &mut Board, padding_lines: u32) {
    println!("{:?}", board.yx[0]);
    let mut cell_buf: char;
    for x in 0..COLUMN_SIZE - 1 {
        // println!("{:?}", board.yx[y]);
        for y in 0..ROW_SIZE - 1 {
            // println!("y: {} x: {}", y , x );
            if board.yx[y][x] {
                cell_buf = '#';
                println!("\nx:{},y:{}", x,y);
            }
            else {
                cell_buf = '.';
            }
            // cell_buf = if board.yx[y][x] { '#' } else { '.' };
            print!("{}", cell_buf);
        }
        println!();
    }
    // loop adding defined padded lines
    for _lines in 0..padding_lines {
        println!();
    }
}

fn init_board(board: &mut Board) {
    board.yx[0][1] = true; // (x,y):(1,0)
    board.yx[1][2] = true; // (x,y):(2,1)
    board.yx[2][0] = true; // (x,y):(0,2)
    board.yx[2][1] = true; // (x,y):(1,2)
    board.yx[2][2] = true; // (x,y):(2,2)
}

fn main() {
    // let mut arr: [i32; COLUMN_SIZE] = [0; COLUMN_SIZE];
    // // let mut index: isize;
    // for index in 0..=(COLUMN_SIZE as isize - 1) {
    //     // println!("{}", index as i32);
    //     arr[index as usize] = index as i32;
    // }
    // for index in -3..=3 {
    //     println!(
    //         "{}",
    //         ((index + COLUMN_SIZE as isize) % COLUMN_SIZE as isize)
    //     );
    // }

    // println!("{}", (-1 + COLUMN_SIZE as isize) % COLUMN_SIZE as isize);
    // println!("{}", (0 + COLUMN_SIZE as isize) % COLUMN_SIZE as isize);

    let mut board: Board = Board::new();
    init_board(&mut board);
    print_board(&mut board, 1);
    board_to_string(&mut board, 1);
}
