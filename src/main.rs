use rand::{Rng, distr::Uniform};
use std::{io, usize};

fn main() {
    let mut wins = 0;
    let mut losses = 0;
    let mut draws = 0;
    let mut board: [u8; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut player = 1;
    let bot_mode = true;
    loop {
        if player == 2 && bot_mode {
            board[random_bot(board) as usize] = player;
            player = 1;
            let winner = check_winner(board);
            if winner > 0 {
                show_board(board);
                match winner {
                    1 => wins += 1,
                    2 => {
                        panic!("Somehow lost")
                    }
                    3 => draws += 1,
                    _ => panic!("Illegal winner. {} is not 1 or 2", winner),
                }
                show_result(wins, losses, draws);
                board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
            }
            continue;
        } else if player == 1 {
            board[perfect_bot(board) as usize] = player;
            player = 2;
            let winner = check_winner(board);
            if winner > 0 {
                show_board(board);
                match winner {
                    1 => wins += 1,
                    2 => {
                        panic!("Somehow lost")
                    }
                    3 => draws += 1,
                    _ => panic!("Illegal winner. {} is not 1 or 2", winner),
                }
                show_result(wins, losses, draws);
                board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
            }
            continue;
        }
        show_board(board);
        println!("Podaj gdzie chcesz zagrać: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let square_played: u8 = input.trim().parse::<u8>().unwrap();
        if board[(square_played - 1) as usize] == 0 {
            board[(square_played - 1) as usize] = player;
            player %= 2;
            player += 1;
        }
        let winner = check_winner(board);
        if winner > 0 {
            match winner {
                1 => println!("X won"),
                2 => println!("O won"),
                3 => println!("DRAW!"),
                _ => panic!("Illegal winner. {} is not 1 or 2", winner),
            }
            board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
        }
    }
}

fn check_winner(board: [u8; 9]) -> u8 {
    // ROWS
    for i in 0..3 {
        if board[3 * i] == board[3 * i + 1]
            && board[3 * i + 1] == board[3 * i + 2]
            && board[3 * i + 1] != 0
        {
            return board[3 * i];
        }
    }
    // COLUMNS
    for i in 0..3 {
        if board[i] == board[i + 3] && board[i + 3] == board[i + 6] && board[i + 6] != 0 {
            return board[i];
        }
    }
    // DIAGONALS
    // 0 1 2
    // 3 4 5
    // 6 7 8
    if board[0] == board[4] && board[4] == board[8] && board[0] != 0 {
        return board[0];
    }

    if board[2] == board[4] && board[4] == board[6] && board[2] != 0 {
        return board[2];
    }

    for val in board.iter() {
        if *val == 0 {
            return 0;
        }
    }

    3
}

fn show_board(board: [u8; 9]) {
    for (index, value) in board.iter().enumerate() {
        if index % 3 == 0 {
            println!()
        } else {
            print!("|")
        }
        match value {
            0 => print!("{}", index + 1),
            1 => print!("X"),
            2 => print!("O"),
            _ => panic!("Incorrect board state!"),
        }
    }
    println!();
}

fn random_bot(board: [u8; 9]) -> u8 {
    loop {
        let mut test = rand::rng();
        let played_square: u8 = test.sample(Uniform::new(0, 9).unwrap());
        if board[played_square as usize] == 0 {
            return played_square;
        }
    }
}

fn two_out_of_three_and_four(a: u8, b: u8, c: u8, d: u8) -> bool {
    if two_out_of_three_but_not_three(a, b, c) {
        return a == d;
    }
    false
}

fn two_out_of_three_but_not_three(a: u8, b: u8, c: u8) -> bool {
    if a == b && b == c {
        return false;
    }

    if a == b {
        if a != 0 {
            return true;
        }
    }

    if a == c {
        if a != 0 {
            return true;
        }
    }

    if b == c {
        if b != 0 {
            return true;
        }
    }

    false
}

fn bot_3_in_a_row_handler(board: [u8; 9], player: u8) -> u8 {
    // ROWS
    for row in 0..3 {
        if two_out_of_three_and_four(
            board[(3 * row) as usize],
            board[(3 * row + 1) as usize],
            board[(3 * row + 2) as usize],
            player,
        ) {
            for j in 0..3 {
                if board[3 * row + j] == 0 {
                    return (3 * row + j) as u8;
                }
            }
        }
    }

    // COLUMNS
    for column in 0..3 {
        if two_out_of_three_and_four(
            board[(column) as usize],
            board[(3 + column) as usize],
            board[(6 + column) as usize],
            player,
        ) {
            for j in 0..3 {
                if board[column + (3 * j)] == 0 {
                    return (column + (3 * j)) as u8;
                }
            }
        }
    }

    // DIAGONALS
    if two_out_of_three_and_four(board[0], board[4], board[8], player) {
        for j in 0..3 {
            if board[4 * j] == 0 {
                return (4 * j) as u8;
            }
        }
    }

    if two_out_of_three_and_four(board[2], board[4], board[6], player) {
        for j in 0..3 {
            if board[2 + (2 * j)] == 0 {
                return (2 + (2 * j)) as u8;
            }
        }
    }
    20
}

fn perfect_bot(board: [u8; 9]) -> u8 {
    // ROWS
    for row in 0..3 {
        if two_out_of_three_but_not_three(
            board[(3 * row) as usize],
            board[(3 * row + 1) as usize],
            board[(3 * row + 2) as usize],
        ) {
            for j in 0..3 {
                if board[3 * row + j] == 0 {
                    return (3 * row + j) as u8;
                }
            }
        }
    }

    // COLUMNS
    for column in 0..3 {
        if two_out_of_three_but_not_three(
            board[(column) as usize],
            board[(3 + column) as usize],
            board[(6 + column) as usize],
        ) {
            for j in 0..3 {
                if board[column + (3 * j)] == 0 {
                    return (column + (3 * j)) as u8;
                }
            }
        }
    }

    // DIAGONALS
    if two_out_of_three_but_not_three(board[0], board[4], board[8]) {
        for j in 0..3 {
            if board[4 * j] == 0 {
                return (4 * j) as u8;
            }
        }
    }

    if two_out_of_three_but_not_three(board[2], board[4], board[6]) {
        for j in 0..3 {
            if board[2 + (2 * j)] == 0 {
                return (2 + (2 * j)) as u8;
            }
        }
    }

    for player in [2, 1] {
        let in_a_row = bot_3_in_a_row_handler(board, player);
        if in_a_row < 10 {
            return in_a_row;
        }
    }

    // IF THERE IS NOTHING TO BLOCK OR THERE IS NO WAY TO WIN, TRY TO PLAY IN THIS ORDER
    let mut order = [4, 0, 8, 2, 6, 1, 3, 5, 7];
    let side_order = [4, 1, 7, 5, 3, 2, 8, 6, 0];

    // HANDLE EDGE CASES
    if two_out_of_three_but_not_three(board[0], board[8], 0)
        || two_out_of_three_but_not_three(board[2], board[6], 0)
    {
        // swap the order
        order = side_order;
    }

    if two_out_of_three_but_not_three(board[0], board[5], board[7]) {
        if board[8] == 0 {
            return 8;
        }
    }

    if two_out_of_three_but_not_three(board[2], board[3], board[7]) {
        if board[6] == 0 {
            return 6;
        }
    }

    if two_out_of_three_but_not_three(board[6], board[1], board[5]) {
        if board[2] == 0 {
            return 2;
        }
    }

    if two_out_of_three_but_not_three(board[8], board[3], board[1]) {
        if board[0] == 0 {
            return 0;
        }
    }

    if two_out_of_three_but_not_three(board[7], board[5], 0) {
        if board[8] == 0 {
            return 8;
        }
    }

    for item in order {
        if board[item] == 0 {
            return item as u8;
        }
    }

    random_bot(board)
}

fn show_result(wins: i32, losses: i32, draws: i32) {
    println!("Wins: {}", wins);
    println!("Draws: {}", draws);
    println!("Losses: {}", losses);
}
