extern crate termcolor;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(PartialEq)]
enum PLAYER {
    Player1,
    Player2,
}

// #[derive(PartialEq)]
// enum PIECE {
//     PLAYER,
//     Empty,
// }

struct TicTacToe {
    state: Vec<Option<PLAYER>>,
    current_player: PLAYER,
}

impl TicTacToe {
    fn build() -> TicTacToe {
        let new = TicTacToe {
            state: vec![None, None, None, None, None, None, None, None, None],
            current_player: PLAYER::Player1,
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

    fn isPossibleMove(&self, num: usize) -> bool {
        if num > 9 && num < 0 {
            return false;
        }

        match self.state.get(num) {
            Some(piece) => match piece {
                Some(piece) => return false,
                None => return true,
            },
            None => return false,
        }
    }

    fn changePieceInState(&mut self, num: usize) {
        let player = self.get_player();
        self.state[num] = Some(player);
    }

    fn has_won(&self) -> bool {
        for i in 0..3 {
            if self.state[i] != None
                && self.state[i] == self.state[i + 3]
                && self.state[i] == self.state[i + 6]
            {
                return true;
            }

            let i = i * 3;

            if self.state[i] != None
                && self.state[i] == self.state[i + 1]
                && self.state[i] == self.state[i + 2]
            {
                return true;
            }
        }

        if (self.state[0] != None
            && self.state[0] == self.state[4]
            && self.state[0] == self.state[8])
            || (self.state[2] != None
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
                Some(_) => (),
                None => return false,
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

    // TODO ta feio e confuso, tou perdida
    fn get_player_piece(&self, num: usize) -> PLAYER {
        if num > 9 && num < 0 {
            return PLAYER::Player1;
        }

        let mut player: PLAYER;

        match self.state.get(num) {
            // tou confusa
            Some(player2) => {
                player = match player2 {
                    Some(wtf) => {
                        if *wtf == PLAYER::Player1 {
                            PLAYER::Player1
                        } else {
                            PLAYER::Player2
                        }
                    }
                    None => PLAYER::Player1,
                }
            }

            None => player = PLAYER::Player1,
        }
        player
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
        if num > 9 && num < 0 {
            return;
        }

        match self.state.get(num) {
            Some(piece) => match piece {
                Some(player) => {
                    let mut stdout = StandardStream::stdout(ColorChoice::Always);
                    // let player = self.get_player_piece(num);
                    if *player == PLAYER::Player1 {
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
                None => {
                    print!("        ");
                    return;
                }
            },
            None => print!("        "),
        }
    }

    fn user_move(&mut self) {
        let player = self.get_player();

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

                if !self.isPossibleMove(number) {
                    println!("The field number must be between 1 and 9.");
                    println!("Or this field is already taken.\n");
                    continue;
                }
                self.changePieceInState(number);

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

fn main() {
    let mut game = TicTacToe::build();

    starting();

    loop {
        // Draw the field
        game.show_field();

        // Ask user
        game.user_move();

        // Check if player won
        if game.has_won() {
            game.show_field();
            game.show_player();
            println!(" won!!!!!!");
            break;
        }

        // Check if all fields are used
        if game.is_over() {
            game.show_field();
            println!("No one won.\nAll fields are used.");
            break;
        }

        game.change_player();
    }
}
