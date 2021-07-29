extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(PartialEq)]
enum PLAYER {
    Player1,
    Player2,
}

#[derive(PartialEq)]
enum PIECE {
    Player1,
    Player2,
    Empty,
}

struct TicTacToe {
    state: Vec<PIECE>,
    current_player: PLAYER,
    score_player1: u32,
    score_player2: u32,
    score_draws: u32,
}

impl TicTacToe {
    fn build() -> TicTacToe {
        let new = TicTacToe {
            state: vec![
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
                PIECE::Empty,
            ],
            current_player: PLAYER::Player1,
            score_player1: 0,
            score_player2: 0,
            score_draws: 0,
        };
        new
    }

    fn change_player(&mut self) {
        if self.current_player == PLAYER::Player1 {
            self.current_player = PLAYER::Player2;
        } else {
            self.current_player = PLAYER::Player1;
        }
    }

    fn is_possible_move(&self, num: usize) -> bool {
        if num > 9 {
            return false;
        }

        match self.state.get(num) {
            Some(piece) => match piece {
                PIECE::Empty => return true,
                _ => return false,
            },
            None => return true,
        }
    }

    fn change_piece_in_state(&mut self, num: usize) {
        let player = self.get_player();
        self.state[num] = match player {
            PLAYER::Player1 => PIECE::Player1,
            PLAYER::Player2 => PIECE::Player2,
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..3 {
            if self.state[i] != PIECE::Empty
                && self.state[i] == self.state[i + 3]
                && self.state[i] == self.state[i + 6]
            {
                return true;
            }

            let i = i * 3;

            if self.state[i] != PIECE::Empty
                && self.state[i] == self.state[i + 1]
                && self.state[i] == self.state[i + 2]
            {
                return true;
            }
        }

        if (self.state[0] != PIECE::Empty
            && self.state[0] == self.state[4]
            && self.state[0] == self.state[8])
            || (self.state[2] != PIECE::Empty
                && self.state[2] == self.state[4]
                && self.state[2] == self.state[6])
        {
            return true;
        }

        false
    }

    fn is_over(&self) -> bool {
        for elem in self.state.iter() {
            match elem {
                PIECE::Empty => return false,
                _ => (),
            }
        }
        true
    }

    fn get_player(&self) -> PLAYER {
        if self.current_player == PLAYER::Player1 {
            PLAYER::Player1
        } else {
            PLAYER::Player2
        }
    }

    fn show_player(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        if self.current_player == PLAYER::Player1 {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                .unwrap();
            write!(&mut stdout, "Player 1").unwrap();
        } else {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                .unwrap();
            write!(&mut stdout, "Player 2").unwrap();
        }

        stdout.reset().unwrap();
    }

    fn show_piece(&self, num: usize) {
        if num > 9 {
            return;
        }
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        match self.state.get(num) {
            Some(piece) => match piece {
                PIECE::Player1 => {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
                        .unwrap();
                    write!(&mut stdout, "Player 1").unwrap();
                }
                PIECE::Player2 => {
                    stdout
                        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
                        .unwrap();
                    write!(&mut stdout, "Player 2").unwrap();
                }

                PIECE::Empty => print!("        "),
            },
            None => print!("        "),
        }
        stdout.reset().unwrap();
    }

    fn user_move(&mut self) {
        loop {
            self.show_player();
            println!(" enter a number:");

            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                println!("Couldn't read line! Try again.\n");
                continue;
            }

            if let Ok(number) = input.trim().parse::<usize>() {
                let number = number - 1;

                if !self.is_possible_move(number) {
                    println!("The field number must be between 1 and 9.");
                    println!("Or this field is already taken.\n");
                    continue;
                }
                self.change_piece_in_state(number);

                break;
            } else {
                println!("Only numbers are allowed.");
                continue;
            }
        }
    }

    fn show_field(&self) {
        println!("\n");
        for i in 0..9 {
            self.show_piece(i);
            if (i + 1) % 3 == 0 && i != 0 && i != 8 {
                println!("\n---------|----------|---------");
            } else if i != 8 {
                print!(" | ");
            }
        }
        println!("\n");
    }

    fn show_score(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        write!(&mut stdout, "SCORE:\n\n").unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
            .unwrap();
        write!(&mut stdout, "Player 1: {}\n", self.score_player1).unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .unwrap();
        write!(&mut stdout, "Player 2: {}\n", self.score_player2).unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        write!(&mut stdout, "Draws: {}\n\n", self.score_draws).unwrap();
        stdout.reset().unwrap();
    }

    fn change_score_win(&mut self) {
        let player = self.get_player();
        match player {
            PLAYER::Player1 => self.score_player1 += 1,
            PLAYER::Player2 => self.score_player2 += 1,
        }
    }

    fn run_game(&mut self) {
        for i in 0..9 {
            self.state[i] = PIECE::Empty;
        }

        loop {
            // Draw the field
            self.show_field();

            // Ask user
            self.user_move();

            // Check if player won
            if self.has_won() {
                self.show_field();
                self.show_player();
                println!(" won!!!!!!");
                self.change_score_win();
                self.change_player();
                break;
            }

            // Check if all fields are used
            if self.is_over() {
                self.show_field();
                println!("No one won.\nAll fields are used.");
                self.score_draws += 1;
                self.change_player();
                break;
            }
            self.change_player();
        }
    }
}

fn starting() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print!("           ");
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    write!(&mut stdout, "Tic").unwrap();

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
        .unwrap();
    write!(&mut stdout, "Tac").unwrap();

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
        .unwrap();
    write!(&mut stdout, "Toe").unwrap();

    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
        .unwrap();
    write!(&mut stdout, "\n\nSome information").unwrap();
    println!("\nThe field will be like this:\n");
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))
        .unwrap();
    println!(" 1 | 2 | 3");
    println!("---|---|---");
    println!(" 4 | 5 | 6");
    println!("---|---|---");
    println!(" 7 | 8 | 9\n\n");
    stdout.reset().unwrap();
    println!("Let's go start the game");
}

fn print_menu() {
    println!("\nMENU: ");
    println!("1-> Start Game");
    println!("2-> Show score");
    println!("3-> Exit\n");
}

fn read_input() -> usize {
    let mut number;
    loop {
        print_menu();
        println!("Enter a number:");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.\n");
            continue;
        }

        if let Ok(value) = input.trim().parse::<usize>() {
            number = value;
            if number > 3 {
                println!("The field number must be between 1 and 3.");
                continue;
            }
            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
    number
}

fn run_menu(game: &mut TicTacToe) {
    loop {
        match read_input() {
            1 => game.run_game(),
            2 => game.show_score(),
            3 => break,
            _ => (),
        }
    }
}

fn main() {
    let mut game = TicTacToe::build();
    starting();
    run_menu(&mut game);
}
