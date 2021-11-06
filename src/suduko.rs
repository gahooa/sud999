use std::io::Read;

struct GameStruct {
    board: [[u8; 9]; 9]
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

    game.board[0][0] = 0;
    
    print_board(&game);

    let bs = read_board_string();
    parse_board_string(&game, &bs);
    
    print_board(&game);
}


fn parse_board_string(game:&GameStruct, bs:&String) {
    let mut line:u8 = 0;

    for c in bs.chars() {
        match c {
            '\n' => {
                line += 1;
                println!("");
            },
            '.' | '0' => {
                print!(".");
            },
            '1' | '2' | '3'| '4' | '5' | '6' | '7' | '8' | '9' => {
                print!("{}", c)
            },
            _ => {
                
            },
        }
    }
}

fn read_board_string() -> String {
    let mut buffer = String::new();
    let result = std::io::stdin().read_to_string(&mut buffer);
    result.expect("Error reading");
    return buffer;
}


fn print_board(game:&GameStruct) {
    println!("+-------+-------+-------+");
    for (i,l) in game.board.iter().enumerate() {
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |", 
            pc(l[0]), pc(l[1]), pc(l[2]), pc(l[3]), pc(l[4]), pc(l[5]), pc(l[6]), pc(l[7]), pc(l[8])
        );
        if i == 2 || i == 5 || i == 8 {
            println!("+-------+-------+-------+");
        }
    }
}

fn pc(num:u8) -> char {
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
}