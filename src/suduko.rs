
use std::{fmt::Write, io::Read};
const BIT_NONE:u32 = 0b000000000;
const BIT_NINE:u32 = 0b111111111;


type BoardGrid = [[i8; 9]; 9];
type BoardBits = [[u32; 9]; 9];


#[derive(Copy, Clone)]
struct Board{
    grid: BoardGrid,
    bits: BoardBits,
}

impl Board {
    /*
    fn copy(&self) -> Board {
        let mut board = Board{
            grid: [[0; 9]; 9],
            bits: [[BIT_NONE;9]; 9],  
        };
        
        board.grid.copy_from_slice(&self.grid);

        return board;
    }
     */
    fn set_bits(&mut self) {
        for row in 0..9 {
            for col in 0..9 {
                self._set_bits_for_cell(row, col);
            }
        }
    }

    fn _set_bits_for_cell(&mut self, row:usize, col:usize) {
        let mut cell_bits = BIT_NINE;

        if self.grid[row][col] > 0 {
            self.bits[row][col] = 2u32.pow((self.grid[row][col]-1).try_into().unwrap());
            return;
        }

        for digit in 1i8..10 {
            let digit_bits = 2u32.pow((digit-1).try_into().unwrap());
            
            // subtract any matches on this row
            for c in 0..9 {
                if self.grid[row][c] == digit {
                    cell_bits = cell_bits & (!digit_bits);
                    break;
                }
            }
            
            // subtract any matches on this col
            for r in 0..9 {
                if self.grid[r][col] == digit {
                    cell_bits = cell_bits & (!digit_bits);
                    break;
                }
            }

            // subtract any matches in this house
            for c in col/3*3..col/3*3+3 {
                for r in row/3*3..row/3*3+3 {
                    if self.grid[r][c] == digit {
                        cell_bits = cell_bits & (!digit_bits);
                        break;
                    }
                }
            }
        }
        self.bits[row][col] = cell_bits;
    }

    fn print(&self) {

        let pc = |num:i8| {
            match num {
                -1 => 'N',
                0 => '.',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => panic!("Invalid number!"),
            }
        };
    
        for (i,l) in self.grid.iter().enumerate() {
            if i % 3 == 0 {
                println!("+-------+-------+-------+");
            }
            println!(
                "| {} {} {} | {} {} {} | {} {} {} |", 
                pc(l[0]), pc(l[1]), pc(l[2]), pc(l[3]), pc(l[4]), pc(l[5]), pc(l[6]), pc(l[7]), pc(l[8])
            );
        }
        println!("+-------+-------+-------+");
        
        for row in 0..9 {
            for col in 0..9 {
                print!("{:#011b}  ", self.bits[row][col]);
                if col % 3 == 2 {
                    print!("  ");
                }
            }
            if row % 3 == 2 {
                println!("");
            }
            println!("");
        }
    }
}


struct Position{
    line: usize,
    char: usize,
}
struct Game {
    board: Board,
    iterations: u64,
    verbose: bool,
}


impl Game {
    fn solve(&mut self) {
        loop{
            self.iterations += 1;
            println!("Starting iteration {}", self.iterations);
            let updated = solve_game_iteration(&mut self.board);
    
            self.print();
    
            if ! updated {
                println!("Nothing updated");
                return;
            }
    
            // Determine game is solved by counting zeros
            let mut count_empty = 0;
            for row in 0..9{
                for col in 0..9 {
                    if self.board.grid[row][col] == 0 {
                        count_empty += 1;
                    }
                }
            }
            
            if count_empty == 0 {
                println!("SOLVED");
                return;
            }
    
            //let mut s = String::new();
            //println!("Enter to continue...");
            //std::io::stdin().read_line(&mut s).expect("Fatal Error");
        }

    }

    fn print(&self) {
        self.board.print();
        println!("Iterations: {}", self.iterations);
    }
}


pub fn run() {

    let mut game = Game{
        board: Board{
            grid: [[ -1; 9]; 9],
            bits: [[BIT_NONE;9]; 9],  
        },
        iterations: 0,
        verbose: false,
    };

   
    loop {
        println!("Paste new board with a line break at the end followed by ctrl+d.");
        println!("Hint: press ctrl+d to generate an example board.");

        let mut buffer = String::new();
        match std::io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {},
            Err(m) => {
                eprintln!("Error reading input: {}", m);
                continue;
            },
        }

        
        match parse_board_string(&mut buffer) {
            Ok(brd) => {
                game.board.grid = brd;
                break;
            },
            Err(m) => println!("Error in board format: {}", m),
        }
    }
    
    println!("\n*** Board Accepted ***\n");
    game.print();
    game.solve();
    game.print();
}



fn parse_board_string(bs:&mut String) -> Result<BoardGrid, String>{
    const NEGATIVE_ONE:i8 = -1;
    let mut pos = Position{line:0, char:0};
    let mut brd: BoardGrid = [[NEGATIVE_ONE; 9]; 9];
    let mut actual_char_number = 0;
    let mut actual_line_number = 1;
    
    match bs.trim() {
        "s1" => {
            bs.clear();
            bs.write_str("
            . . . | 3 8 . | 6 . 5
            . . 7 | . . . | . . .
            . . . | 6 7 5 | 1 . .
            ------|-------|------
            . . . | . . . | . . 4
            . . . | 7 . 8 | . 6 3
            . . . | 5 . . | 8 . .
            ------|-------|------
            8 . . | . 3 4 | 9 . 1
            . . 9 | 1 . . | . . 7
            . . 3 | . . . | . . .
            ").expect("Panic");
        },
        "s2" => {
            bs.clear();
            bs.write_str("
            . . . | . . . | . . .
            . . 4 | . 5 . | 3 . 7
            . . . | 6 . . | 5 1 .
           -------|-------|-------
            7 . . | . 9 8 | . . .
            . 8 3 | . . . | . . .
            . 9 1 | 3 7 . | . . .
           -------|-------|-------
            . 6 . | . . . | . . .
            . 1 7 | 4 3 9 | . . .
            . . 8 | . 1 . | . . 4
            ").expect("Panic");
        },
        "x1" => {
            bs.clear();
            bs.write_str("
            7 . . | 9 . . | . . 8
            . 6 . | . . . | 7 3 .
            . 3 . | 8 . 6 | . . 4
           -------|-------|-------
            . . . | . . . | . . .
            6 . . | . . . | 1 2 .
            . 4 9 | . . . | 3 . .
           -------|-------|-------
            . . . | . . . | . 6 .
            2 1 . | 4 . . | 9 7 .
            . . . | 3 . . | 8 . 1
            ").expect("Panic");
        },
        _ => (),
    }

    for c in bs.chars() {
        actual_char_number += 1;
        match c {
            // no digits found on line, skip line
            '\n' => {
                if pos.char > 0 {
                    pos.line += 1;
                    pos.char = 0;
                }
                actual_char_number = 0;
                actual_line_number += 1;
            }, 

            // Found a 0 or . on char 0-8
            '.'|'0'..='9' => {
                if pos.char > 8 {
                    return Err(format!("Board contains too many numbers on input line {} column {}.", actual_line_number, actual_char_number));
                }
                if pos.line > 8 {
                    return Err(format!("Board contains too many lines with digits starting on input line {}.", actual_line_number));
                }
                
                if c == '.' {
                    brd[pos.line][pos.char] = 0;
                }
                else{
                    brd[pos.line][pos.char] = c.to_digit(10).unwrap() as i8;
                }
                pos.char += 1;
            },

            // Any other character in any position is ignored
            _ => {},
        }
    }

    // Validate that no -1 remains
    for (line_number, line_value) in brd.iter().enumerate() {
        for (char_number, char_value) in line_value.iter().enumerate() {
            if char_value == &NEGATIVE_ONE {
                return Err(format!("Row {} Column {} doesn't have enough digits", line_number+1, char_number+1));
            }
        }
    }

    return Result::Ok(brd);

}






fn solve_game_iteration(board:&mut Board) -> bool{

    board.set_bits();

    // At this point we have an accurate BoardBits struct with all of the bits

    for digit in 1..10 {
        let digit_bits = 2u32.pow((digit-1).try_into().unwrap());
        for row in 0..9{
            for col in 0..9 {
                if board.grid[row][col] == 0 {
                    
                    // Only care about checking digits that are a possiblity
                    if board.bits[row][col] & digit_bits == 0 {
                        continue;
                    }

                    
                    // If only digit in the row, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for c in 0..9 {
                            if c != col {
                                if board.bits[row][c] & digit_bits == digit_bits {
                                    found_elsewhere = true;
                                }
                            }
                        }
                        if ! found_elsewhere {
                            board.grid[row][col] = digit;
                            return true;
                        }
                    }

                    // If only digit in the col, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for r in 0..9 {
                            if r != row {
                                if board.bits[r][col] & digit_bits == digit_bits {
                                    found_elsewhere = true;
                                }
                            }
                        }
                        if ! found_elsewhere {
                            board.grid[row][col] = digit;
                            return true;
                        }
                    }
                    
                    // If only digit in the house, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for r in row/3*3..row/3*3+3 {
                            for c in col/3*3..col/3*3+3 {
                                if ! (r == row && c == col) {
                                    if board.bits[r][c] & digit_bits == digit_bits {
                                        found_elsewhere = true;
                                    }
                                }
                            }
                        }
                        if ! found_elsewhere {
                            board.grid[row][col] = digit;
                            return true;
                        }
                    }
                }
            }
        }
    }

    
    return false;


}
