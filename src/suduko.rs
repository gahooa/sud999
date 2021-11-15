
use std::{fmt::Write, io::Read};


type BoardStruct = [[i8; 9]; 9];


struct Position{
    line: usize,
    char: usize,
}
struct GameStruct {
    board: BoardStruct,
}

pub fn run() {

    let mut game = GameStruct{
        board: [   
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,0,0,0], 
            [0,0,0,0,0,0,9,0,0], 
            [0,0,0,0,0,2,0,0,0], 
            [0,0,0,0,5,0,0,0,0], 
        ]
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

    solve_board(&game.board, 0, &mut 0);
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


fn solve_board(board:&BoardStruct, depth:i8, iteration:&mut u64){
    *iteration += 1;
    
    if *iteration % 1000000 == 0 {
        println!("iteration {} depth {} ", iteration, depth);
        print_board(&board);
    }

    let mut zero_count:i8 = 0;
    for row in 0..9 {
        for col in 0..9 {
            if board[row][col] == 0 {
                zero_count += 1;
                for digit in 1i8..10 {
                    if is_elegible(&board, col, row, digit) {
                        let mut board2 = board.clone();
                        //println!("On row {} col {}, digit {} is possible at depth {}", row+1, col+1, digit, depth);
                        board2[row][col] = digit;
                        solve_board(&board2, depth+1, iteration);
                    }
                }
            }
        }
    }


    if zero_count == 0 {
        print_board(&board);
        panic!("found!");
    }
}

fn is_elegible(board:&BoardStruct, col:usize, row:usize, digit:i8) -> bool {
    for c in 0..9 {
        if board[row][c] == digit {
            return false;
        }
    }

    for r in 0..9 {
        if board[r][col] == digit {
            return false;
        }
    }

    for c in col/3*3..col/3*3+3 {
        for r in row/3*3..row/3*3+3 {
            if board[r][c] == digit {
                return false;
            }
        }
    }

    return true;
}
