use permutohedron;
use std::io;

fn main() {
    println!("Welcome to Tic, Tac, Toe!");
    println!("Player 1, please choose X or O.");

    let mut player = String::new();
    io::stdin().read_line(&mut player).unwrap();

    let mut board = Board::new(player.trim());
    board.print();

    while board.free_spaces() {
        println!(
            "Player {:?}, input the number of the cell in which you would like to play.",
            board.player
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_num = input.trim().parse::<u8>().unwrap();

        board.change_cell(input_num);
        match board.player {
            Player::X => board.xs.push(input_num),
            Player::O => board.os.push(input_num),
        }
        println!("\n");
        board.print();
        if board.winner() {
            println!("{:?} wins!", board.player);
            break;
        }
        board.change_player();
    }
    if !board.winner() {
        println!("It's a draw!");
    }
}

enum State {
    Empty,
    Exy,
    Osy,
}

#[derive(Debug)]
enum Player {
    X,
    O,
}

struct Cell {
    state: State,
    number: u8,
}

impl Cell {
    fn print(&self) -> String {
        match &self.state {
            State::Empty => self.number.to_string(),
            State::Exy => String::from("X"),
            State::Osy => String::from("O"),
        }
    }

    fn change(&mut self, player: &Player) {
        match self.state {
            State::Empty => match player {
                Player::X => self.state = State::Exy,
                Player::O => self.state = State::Osy,
            },
            _ => println!("That square is already filled!"),
        }
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
    player: Player,
    xs: Vec<u8>,
    os: Vec<u8>,
}

impl Board {
    fn new(input: &str) -> Board {
        let nums = vec![vec![2, 7, 6], vec![9, 5, 1], vec![4, 3, 8]];
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for row in nums {
            cells.push(vec![
                Cell {
                    number: row[0],
                    state: State::Empty,
                },
                Cell {
                    number: row[1],
                    state: State::Empty,
                },
                Cell {
                    number: row[2],
                    state: State::Empty,
                },
            ])
        }
        let player = if input == "X" || input == "x" {
            Player::X
        } else {
            Player::O
        };
        let xs = Vec::new();
        let os = Vec::new();
        Board {
            cells,
            player,
            xs,
            os,
        }
    }

    fn print(&self) {
        for row in &self.cells {
            printrow(row);
        }
    }

    fn change_cell(&mut self, num: u8) {
        for row in &mut self.cells {
            for cell in row {
                if cell.number == num {
                    cell.change(&self.player);
                }
            }
        }
    }

    fn change_player(&mut self) {
        match self.player {
            Player::X => self.player = Player::O,
            Player::O => self.player = Player::X,
        }
    }

    fn free_spaces(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if let State::Empty = cell.state {
                    return true;
                }
            }
        }
        false
    }

    fn winner(&self) -> bool {
        let mut checklist = match self.player {
            Player::X => self.xs.clone(),
            Player::O => self.os.clone(),
        };
        let mut permutations: Vec<Vec<u8>> = Vec::new();
        match checklist.len() {
            0 | 1 | 2 => false,
            3 => checklist.into_iter().sum::<u8>() == 15,
            4 | 5 => {
                permutohedron::heap_recursive(&mut checklist, |permutation| {
                    permutations.push(permutation.to_vec())
                });
                for mut x in permutations {
                    x.truncate(3);
                    if x.into_iter().sum::<u8>() == 15 {
                        return true;
                    }
                }
                false
            }
            _ => true,
        }
    }
}

fn printrow(row: &[Cell]) {
    let mut nums = Vec::new();
    for cell in row {
        nums.push(cell.print());
    }
    println!("{:?}", nums);
}
