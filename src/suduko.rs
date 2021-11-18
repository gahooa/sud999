
use std::{fmt::Write, io::Read};
const BIT_NONE:u32 = 0b000000000;
const BIT_NINE:u32 = 0b111111111;



type BoardStruct = [[i8; 9]; 9];
type BoardBits = [[u32; 9]; 9];

struct Position{
    line: usize,
    char: usize,
}
struct GameStruct {
    board: BoardStruct,
    board_bits: BoardBits,
}

pub fn run() {

    let mut game = GameStruct{
        board: [[ -1; 9]; 9],
        board_bits: [[BIT_NONE;9]; 9],  
    };

    
    print_board(&game.board);

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
                game.board = brd;
                break;
            },
            Err(m) => println!("Error in board format: {}", m),
        }
    }
    
    println!("\n*** Board Accepted ***\n");
    print_board(&game.board);

    solve_game(&mut game);
}



fn parse_board_string(bs:&mut String) -> Result<BoardStruct, String>{
    const NEGATIVE_ONE:i8 = -1;
    let mut pos = Position{line:0, char:0};
    let mut brd: BoardStruct = [[NEGATIVE_ONE; 9]; 9];
    let mut actual_char_number = 0;
    let mut actual_line_number = 1;
    
    if bs.trim() == "" {
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

    return Ok(brd);

}


fn print_board(board:&BoardStruct) {

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

    println!("+-------+-------+-------+");
    for (i,l) in board.iter().enumerate() {
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |", 
            pc(l[0]), pc(l[1]), pc(l[2]), pc(l[3]), pc(l[4]), pc(l[5]), pc(l[6]), pc(l[7]), pc(l[8])
        );
        if i == 2 || i == 5 || i == 8 {
            println!("+-------+-------+-------+");
        }
    }
}

fn print_bits(board_bits:&BoardBits) {
    for row in 0..9 {
        for col in 0..9 {
            print!("{:#011b}  ", board_bits[row][col]);
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


fn solve_game(game:&mut GameStruct){
    loop{

        let updated = solve_game_iteration(game);

        print_board(&game.board);
        print_bits(&game.board_bits);

        if ! updated {
            println!("Nothing updated");
            return;
        }

        // Determine game is solved by counting zeros
        let mut count_empty = 0;
        for row in 0..9{
            for col in 0..9 {
                if game.board[row][col] == 0 {
                    count_empty += 1;
                }
            }
        }
        
        if count_empty == 0 {
            println!("SOLVED");
            return;
        }
    }
    
}

fn solve_game_iteration(game:&mut GameStruct) -> bool{

    // Update all of the board bits
    for row in 0..9 {
        for col in 0..9 {
            if game.board[row][col] == 0 {
                game.board_bits[row][col] = solve_bits(&game, row, col);
            }
            else {
                game.board_bits[row][col] = 2u32.pow((game.board[row][col]-1).try_into().unwrap()); 
            }
        }
    }

    // At this point we have an accurate BoardBits struct with all of the bits

    for row in 0..9{
        for col in 0..9 {
            if game.board[row][col] == 0 {
                for digit in 1..10 {
                    println!("searching for onlies: row {} col {} digit {}", row+1, col+1, digit);
                    let digit_bits = 2u32.pow((digit-1).try_into().unwrap());
                    
                    // Only care about checking digits that are a possiblity
                    if game.board_bits[row][col] & digit_bits == 0 {
                        continue;
                    }
                    
                    // If only digit in the row, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for c in 0..9 {
                            if c != col {
                                if game.board_bits[row][c] & digit_bits == digit_bits {
                                    found_elsewhere = true;
                                }
                            }
                        }
                        if ! found_elsewhere {
                            game.board[row][col] = digit;
                            return true;
                        }
                    }

                    // If only digit in the col, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for r in 0..9 {
                            if r != row {
                                if game.board_bits[r][col] & digit_bits == digit_bits {
                                    found_elsewhere = true;
                                }
                            }
                        }
                        if ! found_elsewhere {
                            game.board[row][col] = digit;
                            return true;
                        }
                    }
                    
                    // If only digit in the col, then set that cell and return
                    {
                        let mut found_elsewhere = false;
                        for r in row/3*3..row/3*3+3 {
                            for c in col/3*3..col/3*3+3 {
                                if r != row && c != col {
                                    if game.board_bits[r][c] & digit_bits == digit_bits {
                                        found_elsewhere = true;
                                    }
                                }
                            }
                        }
                        if ! found_elsewhere {
                            game.board[row][col] = digit;
                            return true;
                        }
                    }
                }
            }
        }
    }

    
    return false;


}

fn solve_bits(game:&GameStruct, row:usize, col:usize) -> u32 {
    
    let mut rval = BIT_NINE;

    for digit in 1i8..10 {
        let digit_bits = 2u32.pow((digit-1).try_into().unwrap());
        
        // subtract any matches on this row
        for c in 0..9 {
            if game.board[row][c] == digit {
                rval = rval & (!digit_bits);
                break;
            }
        }
        
        // subtract any matches on this col
        for r in 0..9 {
            if game.board[r][col] == digit {
                rval = rval & (!digit_bits);
                break;
            }
        }

        // subtract any matches in this house
        for c in col/3*3..col/3*3+3 {
            for r in row/3*3..row/3*3+3 {
                if game.board[r][c] == digit {
                    rval = rval & (!digit_bits);
                    break;
                }
            }
        }
       
    }

    return rval;
}

