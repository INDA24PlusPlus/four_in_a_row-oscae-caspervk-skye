use std::process::Command;
use colored::*;

fn main() {
    let mut grid: [[Slot; 6]; 7] = [[Slot::None; 6]; 7];

    let mut red_turn = true;


    loop {
        print_board(&grid);
        
        let mut num: i8;
        loop {
            let mut input = String::new();
                
            // Read input from standard input
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<i8>() {
                Ok(i) => {
                    num = i - 1;
                    if num < 0 || num > 6 || grid[num as usize][0] != Slot::None {
                        println!("Invalid position!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Invalid position!");
                    continue;
                },
            }
            break;
        }

        drop(&mut red_turn, &mut grid, num);

        if check_for_win(&grid, red_turn) {
            print_board(&grid);
            match red_turn {
                true => println!("RED WINS!"),
                false => println!("YELLOW WINS!"),
            }
            return;
        }

        red_turn = !red_turn;
    }
}

fn drop(red_turn: &mut bool, grid: &mut [[Slot; 6]; 7], column: i8) {
    for y in 0..=6 {
        if y == 6 {
            grid[column as usize][5] = match red_turn {
                true => Slot::Red,
                false => Slot::Yellow,
            };
            break;
        } else if grid[column as usize][y] != Slot::None {
            grid[column as usize][y - 1] = match red_turn {
                true => Slot::Red,
                false => Slot::Yellow,
            };
            break;
        }
    }
}

fn check_for_win(grid: &[[Slot; 6]; 7], test_red: bool) -> bool {
    let slot = match test_red {
        true => Slot::Red,
        false => Slot::Yellow,
    };

    // check vertical
    for y in 0..3 {
        for x in 0..7 {
            if grid[x][y] == slot && grid[x][y + 1] == slot && grid[x][y + 2] == slot && grid[x][y + 3] == slot {
                return true;
            }
        }
    }

    // check horiontal
    for y in 0..6 {
        for x in 0..4 {
            if grid[x][y] == slot && grid[x + 1][y] == slot && grid[x + 2][y] == slot && grid[x + 3][y] == slot {
                return true;
            }
        }
    }

    // check diagonals
    for y in 0..3 {
        for x in 0..4 {
            if grid[x][y] == slot && grid[x + 1][y + 1] == slot && grid[x + 2][y + 2] == slot && grid[x + 3][y + 3] == slot {
                return true;
            }
        }
    }
    for y in 0..3 {
        for x in 3..7 {
            if grid[x][y] == slot && grid[x - 1][y + 1] == slot && grid[x - 2][y + 2] == slot && grid[x - 3][y + 3] == slot {
                return true;
            }
        }
    }
    false
}

fn print_board(grid: &[[Slot; 6]; 7]) {
    clear_console();
    println!("1 2 3 4 5 6 7");
    for y in 0..6 {
        for x in 0..7 {
            let col = match grid[x][y] {
                Slot::Red => Color::Red,
                Slot::Yellow => Color::Yellow,
                Slot::None => Color::Black,
            };
            print!("{}", String::from("⬤ ").color(col));
        }
        println!();
    }
}

fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear console");
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Slot {
    Red,
    Yellow,
    None,
}

// XXXXXXX
// XXXXXXX
// XXXXXXX
// XXXXXXX
// XXXXXXX
// XXXXXXX