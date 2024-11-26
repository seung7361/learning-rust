use std::fmt;
use rand::Rng;
use crate::utils::{random_i32, random_usize, two_digit};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS {
    READY,
    PLAYING,
    WIN,
    LOSE,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLOCK {
    MINE = -1,
    NONE = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLAYER_BLOCK {
    OPEN,
    UNOPEN,
    FLAG,
}

#[derive(Clone, Debug)]
pub struct MineSweeper {
    rows: usize,
    cols: usize,
    board: Vec<Vec<BLOCK>>,
    player_board: Vec<Vec<PLAYER_BLOCK>>,
    status: STATUS,
}

impl MineSweeper {
    pub fn new(rows: usize, cols: usize, mines: usize) -> MineSweeper {
        let mut out = MineSweeper {
            rows,
            cols,
            board: vec![vec![BLOCK::NONE; cols]; rows],
            player_board: vec![vec![PLAYER_BLOCK::UNOPEN; cols]; rows],
            status: STATUS::READY,
        };

        // mines in random position
        for _ in 0..mines {
            let mut x = random_usize(0, rows);
            let mut y = random_usize(0, cols);
            
            while out.get_block(x, y) == BLOCK::MINE {
                x = random_usize(0, rows);
                y = random_usize(0, cols);
            }
        
            out.set_mine(x, y);
            println!("mine at ({}, {})", x, y);
        }

        out.init_numbers();
        out.set_status(STATUS::PLAYING);

        out
    }

    pub fn init_numbers(&mut self) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.board[x][y] != BLOCK::MINE {
                    self.board[x][y] = match self.count_num(x, y) {
                        0 => BLOCK::NONE,
                        1 => BLOCK::ONE,
                        2 => BLOCK::TWO,
                        3 => BLOCK::THREE,
                        4 => BLOCK::FOUR,
                        5 => BLOCK::FIVE,
                        6 => BLOCK::SIX,
                        7 => BLOCK::SEVEN,
                        8 => BLOCK::EIGHT,
                        _ => BLOCK::NONE,
                    }
                }
            }
        }
    }

    pub fn count_num(&self, i: usize, j: usize) -> usize {
        let mut result = 0;

        for di in -1..=1 {
            for dj in -1..=1 {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0 && ni < self.rows as i32 && nj >= 0 && nj < self.cols as i32
                && self.board[ni as usize][nj as usize] == BLOCK::MINE {
                    result += 1;
                }
            }
        }

        result
    }

    pub fn get_player_block(&self, x: usize, y: usize) -> PLAYER_BLOCK {
        self.player_board[x][y]
    }

    pub fn get_block(&self, x: usize, y: usize) -> BLOCK {
        self.board[x][y]
    }

    pub fn set_mine(&mut self, x: usize, y: usize) {
        self.board[x][y] = BLOCK::MINE;
    }

    pub fn get_status(&self) -> STATUS {
        self.status
    }

    pub fn set_status(&mut self, status: STATUS) {
        self.status = status;
    }

    pub fn did_win(&self) -> bool {
        for x in 0..self.rows {
            for y in 0..self.cols {
                if self.get_player_block(x, y) == PLAYER_BLOCK::UNOPEN && self.get_block(x, y) != BLOCK::MINE {
                    return false;
                }
            }
        }
        true
    }

    pub fn open_cell(&mut self, x: usize, y: usize) {
        // did win?
        if self.did_win() {
            self.status = STATUS::WIN;
        }

        // did lose?
        if self.board[x][y] == BLOCK::MINE {
            self.status = STATUS::LOSE;
        }

        // is empty?
        if self.board[x][y] == BLOCK::NONE && self.player_board[x][y] != PLAYER_BLOCK::OPEN {
            self.player_board[x][y] = PLAYER_BLOCK::OPEN;

            let dx = vec![0, 0, -1, 1];
            let dy = vec![-1, 1, 0, 0];

            for i in 0..4 {
                let nx = x as i32 + dx[i];
                let ny = y as i32 + dy[i];

                if 0 <= nx && nx < self.rows as i32 && 0 <= ny && ny < self.cols as i32 {
                    self.open_cell(nx as usize, ny as usize);
                }
            }
        }
        self.player_board[x][y] = PLAYER_BLOCK::OPEN;
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.player_board[x][y] == PLAYER_BLOCK::UNOPEN {
            self.player_board[x][y] = PLAYER_BLOCK::FLAG;
        } else if self.player_board[x][y] == PLAYER_BLOCK::FLAG {
            self.player_board[x][y] = PLAYER_BLOCK::UNOPEN;
        }
    }
}

impl fmt::Display for MineSweeper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  ").unwrap();
        for x in 0..self.cols {
            let symbol = two_digit(x);

            write!(f, "{symbol}").unwrap();
        }
        writeln!(f, " +").unwrap();

        for x in 0..self.rows {
            let prefix = two_digit(x);
            write!(f, "{prefix}").unwrap();
            for y in 0..self.cols {
                let symbol = match self.get_player_block(x, y) {
                    PLAYER_BLOCK::FLAG => " F",
                    PLAYER_BLOCK::UNOPEN => " .",
                    PLAYER_BLOCK::OPEN => match self.get_block(x, y) {
                        BLOCK::MINE => " *",
                        BLOCK::NONE => "  ",
                        BLOCK::ONE => " 1",
                        BLOCK::TWO => " 2",
                        BLOCK::THREE => " 3",
                        BLOCK::FOUR => " 4",
                        BLOCK::FIVE => " 5",
                        BLOCK::SIX => " 6",
                        BLOCK::SEVEN => " 7",
                        BLOCK::EIGHT => " 8",
                    }
                };

                write!(f, "{symbol}").unwrap();
            }
            writeln!(f, " |").unwrap();
        }

        write!(f, "+-").unwrap();
        for _ in 0..self.cols {
            write!(f, "--").unwrap();
        }
        writeln!(f, "-+").unwrap();

        Ok(())
    }
}

