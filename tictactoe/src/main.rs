use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn greeting() {
    println!(
        "\nRust TicTacToe\n\
        --------------------\n\
        A Simple game written in the rust programming language.\n
        ",
    )
}

fn draw(state: &[char]) {
    println!("\n");

    for i in 0..3 {
        let offset = i * 3;
        print!("-------------\n| ");
        print_player(&state[offset]);
        print!(" | ");
        print_player(&state[offset + 1]);
        print!(" | ");
        print_player(&state[offset + 2]);
        println!(" |");
    }

    println!("-------------");
}

fn print_player(player: &char) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if player == &'X' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
    } else if player == &'O' {
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
    }

    write!(&mut stdout, "{}", player).unwrap();
    stdout.reset().unwrap();
}

fn ask_user(state: &mut [char], player: char) {
    loop {
        print!("Player '");
        print_player(&player);
        println!("', enter a number: ");

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Couldn't read line! Try again.");
            continue;
        }

        if let Ok(number) = input.trim().parse::<usize>() {
            if number < 1 || number > 9 {
                println!("The field number must be between 1 and 9.");
                continue;
            }

            let number = number - 1;

            if state[number] == 'X' || state[number] == 'O' {
                println!("This field is already taken by '");
                print_player(&player);
                println!("'.");
                continue;
            }

            state[number] = player;

            break;
        } else {
            println!("Only numbers are allowed.");
            continue;
        }
    }
}

fn has_won(state: &[char]) -> bool {
    for i in 0..3 {
        // 세로 조건
        if state[i] == state[i + 3] && state[i] == state[i + 6] {
            return true;
        }
        // 가로 조건
        if state[i * 3] == state[i * 3 + 1] && state[i * 3] == state[i * 3 + 2] {
            return true;
        }
    }

    // 대각선
    if (state[0] == state[4] && state[0] == state[8])
        || (state[2] == state[4] && state[2] == state[6])
    {
        return true;
    }

    false
}

#[inline(always)]
fn is_over(state: &[char]) -> bool {
    state.iter().all(|&v| v == 'X' || v == 'O')
}

fn main() {
    let mut state = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut player = 'X';

    greeting();

    loop {
        draw(&state);
        ask_user(&mut state, player);
        draw(&state);

        if has_won(&state) {
            print!("Player '");
            print_player(&player);
            println!("' won!");
            break;
        }

        if is_over(&state) {
            println!("All fields are used. No one won...");
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }
}
